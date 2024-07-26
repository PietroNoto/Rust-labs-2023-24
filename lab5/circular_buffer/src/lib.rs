use std::ops::{Deref, DerefMut, Index, IndexMut};

pub struct CircularBuffer<T: Copy>
{
    r: usize,
    w: usize,
    size: usize,
    buffer: Vec<Option<T>>
}


pub enum Error
{
    FullBuffer(String),
}


impl<T: Copy> Index<usize> for CircularBuffer<T>
{
    type Output = Option<T>;

    fn index(&self, index: usize) -> &Self::Output 
    {
        if index >= self.buffer.capacity() - 1
        {
            panic!("Index out of bounds.");
        }
        else 
        {
            let j = (self.r + index) % (self.buffer.capacity() - 1);
            &self.buffer[j]
        }
    }
}


impl<T: Copy> IndexMut<usize> for CircularBuffer<T>
{
    fn index_mut(&mut self, index: usize) -> &mut Self::Output 
    {
        if index >= self.buffer.capacity() - 1
        {
            panic!("Index out of bounds.");
        }
        else 
        {
            let j = (self.r + index) % (self.buffer.capacity() - 1);
            &mut self.buffer[j]
        }
    }
}


impl<T: Copy> Deref for CircularBuffer<T>
{
    type Target = [Option<T>];

    fn deref(&self) -> &Self::Target 
    {
        if self.r > self.w
        {
            panic!("Buffer is not contiguous");
        }
        else 
        {
            &self.buffer[0..self.buffer.capacity() - 1] 
        }
    }
}


impl<T: Copy> DerefMut for CircularBuffer<T>
{
    fn deref_mut(&mut self) -> &mut Self::Target 
    {
        if self.r > self.w
        {
            panic!("Buffer is not contiguous");
        }
        else 
        {
            let cap = self.buffer.capacity();
            &mut self.buffer[0..cap - 1]
        }
    }
}


impl<T: Copy> CircularBuffer<T>
{
    pub fn new(capacity: usize) -> Self 
    {
        let mut buf = Vec::<Option<T>>::new();
        buf.resize_with(capacity + 1, Option::default);

        Self {r: 0, w: 0, size: 0, buffer: buf}
    }


    pub fn size(&self) -> usize
    {
        self.size
    }


    pub fn write(&mut self, item: T) -> Result<(), Error>
    {
        if self.size == self.buffer.capacity() - 1
        {
            Err(Error::FullBuffer("Buffer full!".to_string()))
        }
        else 
        {
            self.buffer[self.w] = Some(item);
            self.w = (self.w + 1) % (self.buffer.capacity() - 1);
            self.size += 1;
            Ok(())    
        }
    }


    pub fn read(&mut self) -> Option<T>
    {
        if self.size == 0
        {
            None
        }
        else 
        {
            let item = self.buffer[self.r];
            self.buffer[self.r] = None;
            self.r = (self.r + 1) % (self.buffer.capacity() - 1);
            self.size -= 1;
            
            item
        }
    }


    pub fn clear(&mut self)
    {
        for be in self.buffer.iter_mut()
        {
            *be = None;
        }
        self.size = 0;
        self.w = 0;
        self.r = 0;
    }


    pub fn overwrite(&mut self, item: T)
    {
        if self.size < self.buffer.capacity() - 1
        {
            self.buffer[self.w] = Some(item);
            self.w = (self.w + 1) % (self.buffer.capacity() - 1);
            self.size += 1;
        }
        else 
        {
            self.buffer[self.r] = Some(item);
            self.r = (self.r + 1) % (self.buffer.capacity() - 1); 
            self.w = (self.w + 1) % (self.buffer.capacity() - 1);   
        }
    }


    pub fn make_contiguous(&mut self)
    {
        if self.w >= self.r
        {
            return
        }
        let mut buf_copy = Vec::<Option<T>>::new();
        let start = &self.buffer[self.r..self.buffer.capacity() - 1]; 
        let end = &self.buffer[0..self.w]; 
        
        buf_copy.extend_from_slice(start);
        buf_copy.extend_from_slice(end);
        buf_copy.resize_with(self.buffer.capacity(), Option::default);

        self.buffer = buf_copy;
        self.r = 0;
        self.w = self.size;
    }
}