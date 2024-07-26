mod es0401;
use std::{cell::RefCell, rc::Rc};

use es0401::{List1, List2};

struct Prova
{
    a: i32,
    b: i32
}

fn main() 
{
    // let mut l2 = List2::List::<i32>::new();
    // l2.push(1);
    // l2.push(2);

    // let a = Rc::new(RefCell::new(1));
    // let b = (*a).borrow();
    // let c = (*a).borrow_mut();

    let prova = Prova {a: 1, b: 2};

    let package = Rc::new(RefCell::new(prova));

    let clone1 = Rc::clone(&package);
    let clone2 = Rc::clone(&package);

    let mut borrowed = (*clone1).borrow_mut();

    let b = borrowed.b;

    borrowed.a = 3;

    let o = Some(Rc::new(RefCell::new(1)));

    let p = o.clone();
}
