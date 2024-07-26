use std::{cell::RefCell, rc::Rc};

mod es0402;


fn main() 
{
    let original = Rc::new(RefCell::new(1));

    let copia1 = original.clone();
    let copia2 = original.clone();

    let a = (*copia1).borrow_mut();
    let b = (*copia2).borrow();

    let somma = *a + *b;
}
