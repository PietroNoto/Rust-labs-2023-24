use std::{cell::RefCell, rc::{Rc, Weak}};

pub mod List1 
{
    use std::mem;

    #[derive(Default, Clone)]
    pub enum ListLink<T> 
    {
        Cons(T, Box<ListLink<T>>),
        #[default] Nil,
    }

    #[derive(Clone)]
    pub struct List<T : Default + Clone> 
    {
        head: ListLink<T>,
    }


    impl<T: Default + Clone> List<T> 
    {
        pub fn new() -> Self 
        {
            Self { head: ListLink::Nil }
        }

        /// Insert a new element at the beginning of the list
        // you may encouter a problem with the borrow checker while trying to move self.head to a new variable
        // why? look at mem::replace for solving it
        pub fn push(&mut self, elem: T)
        {
            match &mut self.head
            {
                ListLink::Cons(head, next) => 
                {
                    let old_head = mem::replace(head, elem);
                    let mut new_next = ListLink::Cons(old_head, next.clone());
                    *next = Box::new(new_next);
                }, 
                ListLink::Nil => self.head = ListLink::Cons(elem, Box::new(ListLink::Nil))
            }
        }


        fn pop(&mut self) -> Option<T> 
        {
            unimplemented!()
        }

        // return a referece to the first element of the list
        pub fn peek(&self) -> Option<&T> {
            unimplemented!()
        }

        // uncomment after having implemented the ListIter struct
        // return an interator over the list values
        //fn iter(&self) -> ListIter<T> {
        //    unimplemented!()
        //}

        // take the first n elements of the list and return a new list with them
        pub fn take(&mut self, n: usize) -> List<T> 
        {
            unimplemented!()
        }
    }


    // struct ListIter 
    // {
    //    // implement the iterator trait for ListIter
    // }
    

    // impl Iterator for ListIter 
    // {
    //    //type Item = ...
    
    //    fn next(&mut self) -> Option<Self::Item> 
    //    {
    //        unimplemented!()
    //    }
    // }

    // something that may be useful for the iterator implementation:
    // let a = Some(T);
    // let b = &a;
    // match b { Some(i) => ... } // here i is a reference to T

}


pub mod List2 
{

    #[derive(Clone)]
    pub struct Node<T: Clone> 
    {
        elem: T,
        next: NodeLink<T>,
    }

    type NodeLink<T> = Option<Box<Node<T>>>;

    pub struct List<T: Clone> 
    {
        head: NodeLink<T>,
    }

    // for this implementation, since we are using option, take a look at the take method in Option<T>.
    // It allows to move the value of the option into another option and replace it with None
    // let mut a = Some(5);
    // let b = a.take(); // a is now None and b is Some(5)
    impl<T: Clone> List<T> 
    {
        pub fn new() -> Self 
        {
            Self { head: None }
        }

    
        pub fn push(&mut self, elem: T)
        {
            match self.head.take()
            {
                Some(old_head) => 
                {
                    let new_head = Node {elem: elem, next: Some(old_head)};
                    self.head = Some(Box::new(new_head));
                },
                None => self.head = Some(Box::new(Node {elem: elem, next: None})),
            }  
        }


        pub fn pop(&mut self) -> Option<T>
        {   
            match self.head.take()
            {
                Some(h) =>
                {
                    self.head = h.next;
                    Some(h.elem)
                },
                None => None
            }
        }

        /// Return a referece to the first element of the list
        pub fn peek(&self) -> Option<&T> 
        {
            match &self.head
            {
                Some(h) => Some(&h.elem),
                None => None
            }
        }


        /// Take the first n elements of the list and return a new list with them
        pub fn take(&mut self, n: usize) -> List<T> 
        {
            let mut new_list = List::<T>::new();
            let mut cur = self.head.clone();

            for _ in 0..n
            {
                if cur.is_none()
                {
                    return List::<T>::new();
                }
                let el: T = cur.clone().unwrap().elem;
                new_list.push(el);
                cur = cur.clone().unwrap().next;
            }
            return new_list;
        }


        fn iter(&self) -> ListIter<T> 
        {
            ListIter { list: self }
        }
    }


    struct ListIter<'a, T: Clone>
    {
       list: &'a List<T>
    }

    impl<'a, T: Clone> Iterator for ListIter<'a, T>
    {
        type Item = &'a NodeLink<T>;
    
        fn next(&mut self) -> Option<Self::Item> 
        {
            match &self.list.head
            {
                Some(n) => Some(&n.next),
                None => None
            }
        }
    }
}


// *****
// double linked list suggestion: use Rc, since we need more than one reference to the same node
// for mutating the list and changing the next and prev fields we also need to be able to mutate the node, therefore we can use RefCell

// how to access content of Rc<RefCell<T>>:
// es let a = Rc::new(RefCell::new(5));
// let mut x = (*a).borrow_mut();  // with (*a) we dereference the Rc, with (*a).borrow_mut() we get a mutable reference to the content of the RefCell
// *x = 6; // we can now change the content of the RefCell

// to take a value from a Rc (useful when popping a value from the list): usually it is not possible since it may be referenced elsewhere.
// if you can guarantee it's the only reference to the value  youu can use Rc::try_unwrap(a).unwrap().into_inner() to get the value
// it first takes out the value from the Rc, then it tries to unwrap the value from the Result, and finally it takes the inner value from the Result
// see here
// https://stackoverflow.com/questions/70404603/how-to-return-the-contents-of-an-rc

// other hint that may be useful: Option<T> has a default clone implementation which calls the clone of T. Therefore: 
// Some(T).clone() ->  Some(T.clone())
// None.clone() -> None


type NodeLink<T> = Option<Rc<RefCell<DNode<T>>>>; // we define a type alias for better readibility
type NodeBackLink<T> = Option<Weak<RefCell<DNode<T>>>>;

struct DNode<T: Clone> 
{
    v: T,
    prev: NodeBackLink<T>, // here we can't put NodeLink to avoid a cycle reference, what do we use?
    next: NodeLink<T> 
}

struct DList<T: Clone> 
{
    head: NodeLink<T>,
    tail: NodeLink<T>
}

impl<T: Clone> DList<T>
{
    pub fn new() -> Self
    {
        Self { head: None, tail: None }
    }


    pub fn push_front(&mut self, elem: T)
    {
        match self.head.take()
        {
            Some(old_head) => 
            {
                let new_head = DNode {v: elem, prev: None, next: Some(Rc::clone(&old_head))};
                let prev = Rc::new(RefCell::new(new_head));
                (*Rc::clone(&old_head)).borrow_mut().prev = Some(Rc::downgrade(&prev));
            }, 
            None => 
            {
                let new_node = DNode {v: elem, prev: None, next: None};
                let package = Rc::new(RefCell::new(new_node));
                self.head = Some(Rc::clone(&package));
                self.tail = Some(Rc::clone(&package));
            }
        }
    }


    pub fn push_back(&mut self, elem: T)
    {
        match self.head.take()
        {
            Some(old_tail) => 
            {
                let prev = Rc::clone(&old_tail);
                let new_tail = DNode {v: elem, prev: Some(Rc::downgrade(&prev)), next: None};
                (*Rc::clone(&old_tail)).borrow_mut().next = Some(Rc::new(RefCell::new(new_tail)));
            },
            None => 
            {
                let new_node = DNode {v: elem, prev: None, next: None};
                let package = Rc::new(RefCell::new(new_node));
                self.head = Some(Rc::clone(&package));
                self.tail = Some(Rc::clone(&package));
            }
        }
    }


    pub fn pop_front(&mut self) -> Option<T>
    {
        match self.head.take()
        {
            Some(package) => 
            {
                let old_head = (*package).borrow();
                self.head = old_head.next.clone();
                Some(old_head.v.clone())
            },
            None => None
        }
    }


    pub fn pop_back(&mut self) -> Option<T>
    {
        match self.tail.take()
        {
            Some(old_tail) =>
            {
                let prev: NodeBackLink<T> = (*old_tail).borrow().prev.clone();
                if prev.is_none()
                {   
                    self.tail = None;
                }
                else 
                {
                    self.tail = Weak::upgrade(&prev.unwrap()).clone();
                }
                let v = (*old_tail).borrow().v.clone();
                Some(v)
            }, 
            None => None
        }
    }


    pub fn peek(&self) -> Option<Rc<T>> 
    {
        match self.head.clone()
        {
            Some(h) =>
            {
                let v = (*h).borrow().v.clone();
                let rcv = Rc::new(v);
                Some(rcv)
            },
            None => None
        }
    }

    /// Removes the nth item in the list
    fn popn(&mut self, n: usize) -> Option<T>
    {
        let mut cur = self.head.clone();

        for i in 0..n
        {
            if cur.is_none()
            {
                return None;
            }
            cur = (*cur.unwrap()).borrow().next.clone();
        }

        match cur
        {
            Some(node) => 
            {
                let val = (*node).borrow().v.clone();
                let prev_weak: NodeBackLink<T> = (*Rc::clone(&node)).borrow().prev.clone();
                let prev: NodeLink<T> = Weak::upgrade(&prev_weak.unwrap());
                let next: NodeLink<T> = (*Rc::clone(&node)).borrow().next.clone();

                if prev.is_none() && next.is_some()
                {
                    // cur is the head
                    (*next.clone().unwrap()).borrow_mut().prev = None;
                    self.head = next.clone();
                }

                else if next.clone().is_none() && prev.is_some()
                {
                    // cur is the tail
                    (*prev.clone().unwrap()).borrow_mut().next = None;
                    self.tail = prev.clone();
                }
                else 
                {
                    // cur is an intermediate node
                    (*prev.clone().unwrap()).borrow_mut().next = next.clone();
                    (*next.clone().unwrap()).borrow_mut().prev = Some(Rc::downgrade(&prev.unwrap()));
                }
                return Some(val);
            },
            None => None
        }
    }
    
}
