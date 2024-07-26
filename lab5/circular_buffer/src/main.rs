use std::{sync::{Arc, Mutex}, thread, time::Duration};

use circular_buffer::CircularBuffer;

fn main() 
{
    let cb_ = CircularBuffer::<(i32, i32)>::new(100);
    let cb = Arc::new(Mutex::new(cb_));
    let mut val = 0;

    let mut threads = Vec::new();
    let producers = 1;
    let consumers = 1;
    let producer_sleep_time = Duration::from_secs(1);
    let consumer_sleep_time = Duration::from_secs(2);

    for p in 0..producers
    {
        let cb_clone = cb.clone();
        threads.push(thread::spawn(move||
        {
            loop 
            {
                let mut cb = cb_clone.lock().unwrap();

                match cb.write((p, val))
                {
                    Ok(()) => 
                    {
                        println!("({}, {}) scritti correttamente!", p, val);
                        val += 1;
                    },
                    Err(_) => println!("Buffer pieno!")
                }
                drop(cb);
                thread::sleep(producer_sleep_time);
            }
        }));
    }
    
    for c in 0..consumers
    {
        let cb_clone = cb.clone();
        threads.push(thread::spawn(move||
        {
            loop 
            {
                let mut cb = cb_clone.lock().unwrap();
                match cb.read()
                {
                    Some((p, val)) => println!("({}, {})", p, val),
                    None => println!("Buffer vuoto!")
                }
                drop(cb);
                thread::sleep(consumer_sleep_time);
            }
        }));
    }  

    for t in threads
    {
        t.join().unwrap();
    } 
}