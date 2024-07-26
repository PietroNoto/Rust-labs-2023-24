use std::ops::Deref;

use circular_buffer::CircularBuffer;

fn main() 
{
    let mut cb = CircularBuffer::<u32>::new(10);
    let a = cb.deref();
    for el in a
    {
        println!("{}", el.unwrap_or(0));
    }
    
}
