use std::sync::{Condvar, Mutex};

pub struct CyclicBarrier
{
    nthreads: usize,
    state: Mutex<(usize, bool)>,
    cv: Condvar
}


impl CyclicBarrier
{
    pub fn new(nthreads: usize) -> Self
    {
        let state = Mutex::new((0, false));
        let cv = Condvar::new();
        Self { nthreads: nthreads, state: state, cv: cv }
    }


    /// 1. (solo se la barriera è chiusa) metto il thread in attesa aumentando il contatore dei thread in attesa
    /// 2. Se il numero di thread in attesa = nthreads apro la barriera
    /// 3. Notifico tutti i thread
    /// 4. Resetto il contatore dei thread in attesa
    /// 5. Quando l'ultimo thread è uscito chiudo la barriera
    pub fn wait(&self)
    {
        let mut guard = self.state.lock().unwrap();

        while (*guard).1 {}

        if (*guard).0 >= self.nthreads - 1
        {
            (*guard).1 = true;
            self.cv.notify_all();
            (*guard).0 = 0;
            (*guard).1 = false;
        }
        else 
        {
            (*guard).0 += 1;
            guard = self.cv.wait(guard).unwrap();
        }    
    }
}