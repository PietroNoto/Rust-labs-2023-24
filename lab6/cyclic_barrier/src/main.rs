mod cb;
use std::{sync::Arc, thread};

use cb::CyclicBarrier;

fn main() 
{
    const NTHREADS: usize = 3;
    
    let mut cbarrier = Arc::new(CyclicBarrier::new(NTHREADS));
    let mut threads = Vec::new();

    for i in 0..3
    {
        // let mut cb_clone = Arc::clone(&cbarrier);
        // let waiter = cbarrier.get_waiter();
        let cb_clone = Arc::clone(&cbarrier);

        threads.push(thread::spawn( move|| 
        {
            for j in 0..10
            {
                cb_clone.wait();
            }
        }));
    }
}
