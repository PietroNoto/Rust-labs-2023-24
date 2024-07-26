use std::time::SystemTime;


pub enum Error
{
    Simple(SystemTime),
    Complex(SystemTime, String)
}

pub enum MulErr
{
    Overflow,
    NegativeNumber
}

struct Node 
{
    name: String,
    size: u32,
    count: u32
}

impl Node
{
    pub fn new(name: String) -> Node
    {
        Node {name: name, size: 0, count: 0}
    }

    pub fn size(&self, size: u32) -> Node
    {
        Node {name: self.name.clone(), size: size, count: self.count}
    }

    pub fn count(&self, count: u32) -> Node
    {
        Node {name: self.name.clone(), size: self.size, count: count}
    }

    pub fn to_string(&self) -> String
    {
        format!("Name: {}, size: {}, count: {}", self.name, self.size, self.count)
    }
}



fn main() 
{
    let res: u32 = 0;

    let n = Node::new(String::from("nodo"))
        .size(10)
        .count(5);

    println!("{}", n.to_string());

    for a in -10..10
    {
        for b in -10..10
        {
            match mul(a, b)
            {
                Err(MulErr::NegativeNumber) => println!("Negative number!"),
                Err(MulErr::Overflow) => break,
                Ok(res) => println!("{}", res)
            }
        }
    }
}


fn print_error(e: Error)
{
    match e
    {
        Error::Simple(_) => println!("Simple error!"),
        Error::Complex(_, msg) => println!("Complex error: {}", msg),
    }
}

pub fn mul(a: i32, b: i32) -> Result<u32, MulErr> 
{
    if a < 0 || b < 0
    {
        return Err(MulErr::NegativeNumber);
    }
    let res: u32 = a as u32 * b as u32;
    if res > u32::MAX
    {
        return Err(MulErr::Overflow);
    }
    else 
    {
        return Ok(res);
    }
}
