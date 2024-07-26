use std::{rc::Rc, sync::{mpsc::{sync_channel, Receiver, SyncSender}, Arc}};

pub struct CyclicBarrier
{
    nthreads: usize,
    waiting_threads: usize,
    sender: SyncSender<()>,
    receiver: Receiver<()>
}


impl CyclicBarrier
{
    pub fn new(nthreads: usize) -> Self
    {
        let (s, r) = sync_channel::<()>(nthreads);

        Self {nthreads: nthreads, waiting_threads: 0, sender: s, receiver: r}
    }


    pub fn wait(&self)
    {

    }
}