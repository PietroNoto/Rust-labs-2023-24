use core::panic;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::rc::{Rc, Weak};
use std::cell::RefCell;


type NodeLink = Option<Rc<RefCell<Node>>>;
type NodeBackLink = Option<Weak<RefCell<Node>>>;
type NodeEntry = Rc<RefCell<Node>>;


#[derive(PartialEq, Clone, Copy)]
pub enum NodeFunction 
{
    Generator(bool),
    Switch(bool),
    Light,
}

pub struct Node 
{
    name: String,
    function: NodeFunction,
    parent: NodeBackLink,
    outs: [NodeLink; 2]
}

impl Node 
{
    pub fn new(name: String, function: NodeFunction, parent: NodeBackLink) -> Self
    {
        Self {name: name, function: function, parent: parent, outs: [None, None]}
    }
    

    /// Turn on or off the switch or the generator, if it's a light return an error 
    pub fn switch(&mut self) -> Result<(), String> 
    {
        match self.function
        {
            NodeFunction::Generator(is_on) => 
            {
                self.function = NodeFunction::Generator(!is_on);
                Ok(())
            },
            NodeFunction::Switch(is_on) => 
            {
                self.function = NodeFunction::Switch(!is_on);
                Ok(())
            },
            NodeFunction::Light => Err("Error: cannot swith a light".to_string())
        }
    }
}


pub struct CircuitTree
{
    root: NodeLink,
    names: HashMap<String, NodeEntry>
}

impl CircuitTree
{
    pub fn new() -> Self
    {
        let f = File::open("input.txt");
        if f.is_err()
        {
            panic!();
        }

        // Campi CircuitTree
        let root: NodeLink = None;
        let names: HashMap<String, NodeEntry> = HashMap::new();
        let mut ct = CircuitTree {root: root, names: names};

        let buf = BufReader::new(f.unwrap());
        for l in buf.lines()
        {
            if l.is_err()
            {
                panic!();
            }

            // Parsing della riga del file
            let line = l.unwrap();
            let fields: Vec<&str> = line.split(" ").collect();
            let function_str = fields[0];
            let name = fields[1];
            let parent_str = fields[2];

            // Function
            let mut function: NodeFunction = NodeFunction::Light;
            let mut is_on: bool = false;

            // Generatore / Interruttore
            if function_str.to_ascii_lowercase() == "g" || function_str.to_ascii_lowercase() == "s"
            {
                // Esisterà il campo on/off (fields[3])
                if fields[3].to_ascii_lowercase() == "off"
                {
                    is_on = false;
                } 
                else if fields[3].to_ascii_lowercase() == "on"
                {
                    is_on = true;
                }

                // Caso generatore
                if function_str.to_ascii_lowercase() == "g"
                {
                    function = NodeFunction::Generator(is_on);
                }
                else 
                {
                    function = NodeFunction::Switch(is_on);
                } 
            }

            // Creo il nodo e chiamo self.add()
            let node = Node::new(name.to_string(), function, None);
            CircuitTree::add(&mut ct, parent_str, node);
        }
        return ct;
    }


    /// Get a node by name
    pub fn get(&self, name: &str) -> NodeLink 
    {
        match self.names.get(name)
        {
            Some(node) => Some(node.clone()),
            None => None,
        }
    }


    /// Add a new node after finding its parent and updates parent's outs
    pub fn add(&mut self, parent_name: &str, node: Node)
    {
        let node_name = node.name.clone();
        let new_node = Rc::new(RefCell::new(node));

        if parent_name != "-"
        {
            // Cerco  nella mappa il parent del nodo da aggiungere
            let pn: NodeEntry = self.get(parent_name).unwrap();

            // Aggiungo il nodo corrente ai suoi figli
            let outs: &mut [NodeLink; 2] = &mut (*pn).borrow_mut().outs;
            let pos = outs.iter().filter(|el: &&NodeLink| el.is_some()).count();
            if pos < 2
            {
                outs[pos] = Some(Rc::clone(&new_node));
            }

            // Aggiungo il parent al campo parent del nodo nuovo
            let parent = Some(Rc::downgrade(&pn));
            (*new_node).borrow_mut().parent = parent;
        }
        else    // caso generatore
        {
            self.root = Some(Rc::clone(&new_node));
        }
        
        // Aggiungo il nuovo nodo alla mappa
        self.names.insert(node_name, new_node);
    }


    /// Is the light on? Error if it's not a light
    pub fn light_status(&self, name: &str) -> Result<bool, String> 
    {
        let mut cur: NodeLink = self.get(name);
        let mut i: usize = 0;

        while cur.is_some()
        {   
            let cur_clone: NodeLink = cur.clone();
            let package = cur_clone.unwrap();
            let node = package.borrow();

            let function = node.function;
            if i == 0 && function != NodeFunction::Light
            {
                return Err("Error: item provided is not a light".to_string());
            }
            if function == NodeFunction::Switch(false) || function == NodeFunction::Generator(false)
            {
                return Ok(false);
            }

            let parent_weak: NodeBackLink = node.parent.clone();
            let parent = Weak::upgrade(&parent_weak.unwrap());

            cur = parent;
            i += 1;
        }
        return Ok(true);
    }


    pub fn turn_light_on(&self, name: &str) 
    {
        let mut cur: NodeLink = self.get(name);
        let mut i: usize = 0;

        while cur.is_some()
        {
            let cur_clone: NodeLink = cur.clone();

            let package = cur_clone.unwrap();
            let mut node = package.borrow_mut();

            // Accendi l'interruttore / generatore
            if i > 0 && node.switch().is_err()
            {
                return;
            }

            let parent_weak: NodeBackLink = node.parent.clone();
            let parent = Weak::upgrade(&parent_weak.unwrap());

            cur = parent;
            i += 1;
        }      
    }
}