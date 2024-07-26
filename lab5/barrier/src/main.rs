mod cb;
use cb::CyclicBarrier;

use std::{sync::Arc, thread};


fn main()
{
    let nthreads = 10;

    let arc = Arc::new(CyclicBarrier::new(nthreads));
    let mut threads = Vec::new();

    for i in 0..nthreads
    {
        let arc_clone = Arc::clone(&arc);

        threads.push(thread::spawn(move||
        {
            for j in 0..10
            {
                arc_clone.wait();
                println!("After barrier: thread {} = {}", i, j);
            }
        }));
    }

    for t in threads
    {
        t.join().unwrap();
    }
}