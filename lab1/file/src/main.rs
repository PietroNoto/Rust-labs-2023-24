use std::{fs::OpenOptions, io::{Read, Write}};

fn main() 
{
    let file_name = "test.txt";
    let rf = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(file_name);

    match rf
    {
        Ok(mut f) =>
        {
            let mut content = String::new(); 
            let mut content_b = [0; 256];
            f.read_to_string(&mut content).unwrap();
            //f.read(&mut content_b).unwrap(); --> Quando si fa la write() vengono scritti tutti i byte del buffer

            for _ in 0..10
            {
                f.write(&content_b).unwrap();
            }
        },
        Err(e) =>
        {
            println!("Error: {}", e.to_string());
        }
    }
    
    println!("Hello, world!");
}
