use std::{env::args, thread, time::{SystemTime, UNIX_EPOCH}};
use itertools::Itertools;


fn main() 
{
    // let numbers: Vec<u8> = args().skip(1)
    //     .map(|n| n.parse::<u8>().unwrap()).collect();

    let numbers = [1, 2, 3, 4, 5, 6, 7, 8];
    let ops = ['+', '-', '*', '/'];
    let threads: usize = 6;
    let np: usize = 8;
    let num_perms: Vec<Vec<&u32>> = numbers.iter().permutations(np).collect(); 

    let start_time = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();

    thread::scope(|s|
    {
        for t in 0..threads
        {
            let inf = num_perms.len() / threads * t;
            let sup = inf + num_perms.len() / threads;
            let perms = num_perms[inf..sup].to_vec();

            s.spawn(||
            {
                for perm in perms
                {
                    for op_comb in ops.iter().combinations_with_replacement(np - 1)
                    {
                        let res = evaluator(&perm, &op_comb);
                        if res.is_ok()
                        {
                            println!("{} = 10", res.unwrap());
                        }
                    }
                }
            });
        }
    });

    let end_time = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    let duration = (end_time - start_time).as_micros();
    println!("Durata: {}ms", duration/1000);

}


fn evaluator(numbers: &Vec<&u32>, ops: &Vec<&char>) -> Result<String, ()>
{
    let mut tot: i32 = *numbers[0] as i32;
    let mut s = tot.to_string();

    for i in 0..ops.len()
    {
        s.push(*ops[i]);
        s.push(char::from_digit(*numbers[i + 1], 10).unwrap());

        match *ops[i]
        {
            '+' => tot += *numbers[i + 1] as i32,
            '-' => tot -= *numbers[i + 1] as i32,
            '*' => tot *= *numbers[i + 1] as i32,
            '/' => 
            {
                if *numbers[i + 1] == 0
                {
                    return Err(());
                }
                else 
                {
                    tot /= *numbers[i + 1] as i32;
                } 
            },
            _ => return Err(())
        }
    }
    if tot == 10
    {
        Ok(s)
    }
    else 
    {
        Err(())    
    }
}
