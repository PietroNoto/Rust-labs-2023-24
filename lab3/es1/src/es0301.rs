use std::collections::HashSet;

// find all subsequences of seq in s and return a vector of tuples containing the start position
// and the found subsequences as string slices
// ignore overlaps: if a subsequence is found, the search must continue from the next character
// missing lifetimes: the result string slices depend only from one input parameter, which one?

// suggestion: write a function find_sub(&str, &str) -> Option<(usize, &str)> that finds the first 
// subsequence in a string, you can use it in all the following functions

fn find_sub<'a>(s: &'a str, seq: &str) -> Option<(usize, &'a str)>
{
    let symbols = seq.split(",").collect::<Vec<&str>>();
    let s_array = s.chars().collect::<Vec<char>>();
    let mut keys = Vec::<char>::new();
    let mut values = Vec::<(usize, usize)>::new();
    let mut start: Option<usize> = None;
    let mut k = 0;
    let mut len: usize = 0;

    for symbol in symbols
    {
        let (l, interval) = symbol.split_at(1);
        let minmax = interval.split("-").collect::<Vec<&str>>();
        keys.push(l.parse().unwrap());
        values.push((minmax[0].parse().unwrap(), minmax[1].parse().unwrap()));
    } 

    while k < keys.len()
    {
        let mut i = 0;
        while i < s_array.len()
        {
            if k >= keys.len()
            {
                break;
            }
            let l_key = keys[k];
            let min = values[k].0;
            let max = values[k].1;
            if s_array[i] == l_key
            {
                let mut contig: usize = 1;
                len += 1;
                if len == 1
                {
                    start = Some(i);
                }
                for j in (i + 1)..s_array.len()
                {
                    if s_array[j] == l_key
                    {
                        contig += 1;
                        len += 1;
                    }
                    else 
                    {
                        break;
                    }
                }
                if (contig < min || contig > max) && k < keys.len() - 1
                {
                    k = 0;
                    start = None;
                    len = 0;
                }
                else if contig > max && k >= keys.len() - 1
                {
                    len = len - (contig - max);
                    contig = max;
                    k += 1;
                }
                else 
                {
                    k += 1;    
                }
            }
            i += 1;
            if i >= s_array.len()
            {
                return None;
            }
        }
    }
    if start.is_some()
    {
        let pos = start.unwrap();
        let sub_str = &s[pos..pos + len];
        Some((pos, sub_str))
    }
    else 
    {
        None    
    }
}


fn subsequences1<'a>(s: &'a str, seq: &str) -> Vec<(usize, &'a str)>
{
    let mut sub_seqs: HashSet<(usize, &str)> = HashSet::new();
    for i in 0..s.len()
    {
        let ss = find_sub(&s[i..s.len()], seq);
        if ss.is_some()
        {
            let (pos, st) = ss.unwrap();
            sub_seqs.insert((pos + i, st));
        }
    }
    Vec::from_iter(sub_seqs.into_iter())
}


pub fn demo1() 
{
    let a = "AACGGTAACC".to_string();
    let seq = "A1-1,C2-4";

    for (off, sub) in subsequences1(&a, seq) 
    {
        println!("Found subsequence at position {}: {}", off, sub);
    }
}

// Now we want to find different subsequences at the same time, seq is a vector of string slices 
// with many subsequence to search
// For each subsequence find all the matches and to the results (there may be overlaps, ignore them), but in this way you can reuse the previous solution
// The result will contain: the start position in s, the found subsequence as string slice and the mached subsequence in seq
// Now the string slices in the rsult depend from two input parameters, which ones?

fn subsequences2<'a, 'b>(s: &'a str, seqs: &[&'b str]) -> Vec<(usize, &'a str, &'b str)> 
{
    let mut sub_seqs: HashSet<(usize, &str, &str)> = HashSet::new();

    for seq in seqs
    {
        for i in 0..s.len()
        {
            let ss = find_sub(&s[i..s.len()], seq);
            if ss.is_some()
            {
                let (pos, st) = ss.unwrap();
                sub_seqs.insert((pos + i, st, seq));
            }
        }
    }
    Vec::from_iter(sub_seqs.into_iter())
}

pub fn demo2() 
{
    let a = "AACGGTTAACC".to_string();
    let seqs = ["A1-1,C2-4", "G1-1,T2-4"];

    for (off, matched, sub) in subsequences2(&a, &seqs) 
    {
        println!("Found subsequence {} at position {}: {}", matched, off, sub);
    }
}

// Now we want to do some DNA editing! Therefore we receive a mutable string and we'd like to return a vector of mutable string slices
// Follow this steps:
// 1. adjust the lifetimes without any implementation yet: does it compile?
// 2. try to implement the function: does it compile?
// 3. if it doesn't compile, try to understand why from the compiler errors and draw all the necessary lifetimes
// 4. Spoiler: basically it's not possibile to return more then one mutable reference to the same data
// 5. Try this workaround: return a vector of indexes (first solution) and let the caller extract the mutable references
// 7. (later in the course you will learn about smart pointers, which can be used to solve this kind of problems in a more elegant way)

fn subsequences3<'a>(s: &'a mut str, seq: &str) -> Vec<(usize, usize)> 
{
    let mut sub_seqs: HashSet<(usize, usize)> = HashSet::new();
    for i in 0..s.len()
    {
        let ss = find_sub(&s[i..s.len()], seq);
        if ss.is_some()
        {
            let (pos, st) = ss.unwrap();
            sub_seqs.insert((pos + i, st.len()));
        }
    }
    Vec::from_iter(sub_seqs.into_iter())
}


pub fn demo3() 
{
    let mut a = "AACGGTAACC".to_string();
    let seq = "A1-1,C2-4";

    for (off, len) in subsequences3(&mut a, seq) 
    {
        println!("Found subsequence at position {}: {}", off, &a[off..off + len]);
    }
}

// DNA strings may be very long and we can get a lot of matches.
// Therefore we want to process a subsequence as soon as we find it, without storing it in a vector
// A solution is to pass a closure to the function, which will be called for each match
// do you need to put lifetime annotations in the closure? why?

fn subsequence4<'a>(s: &'a str, seq: &str, print_f: fn(usize, &str)) 
{
    let mut sub_seqs: HashSet<(usize, &str)> = HashSet::new();
    for i in 0..s.len()
    {
        let ss = find_sub(&s[i..s.len()], seq);
        if ss.is_some()
        {
            let (pos, st) = ss.unwrap();
            if sub_seqs.insert((pos + i, st))
            {
                print_f(pos + i, st);
            }
        }
    }
}


pub fn demo4() 
{
    let a = "AACGGTAACC".to_string();
    let seq = "A1-1,C2-4";

    subsequence4(&a, seq, |pos, sub| 
        {
            println!("Found subsequence at position {}: {}", pos, sub);
        });
}

// Now let's define a struct SimpleDNAIter (add the required lifetimes), memorizing a DNA sequence and the subsequence to search
// Then we add a next() method to the struct, which will return the next subsequence found in the DNA sequence after each call
// The result of next() is a tuple, but it's wrapped in an Option, because a call to next() may find no more subsequences in the DNA sequence
// In order to implement it, you may add any other attribute to the struct (remember: the struct is stateful and after each call to next() you must start from the last position found)
// The struct may be used as shown in the demo_SimpleDNAIter() function
// This approach is similar to the previous one, but it's more flexible and it can be used in more complex scenarios. For example you may interrupt it
// at any time and resume it later

struct SimpleDNAIter<'a> 
{
    s: &'a str,
    seq: &'a str,
    current_pos: usize
}


impl<'a> SimpleDNAIter<'a>
{
    pub fn new(s: &'a str, seq: &'a str) -> Self 
    {
        SimpleDNAIter { s: s, seq: seq, current_pos: 0 }
    }

    pub fn next(&mut self) -> Option<(usize, &str)> 
    {
        let mut sub_seqs: HashSet<(usize, &str)> = HashSet::new();
        let s = self.s;
        let i = self.current_pos;
        let ss = find_sub(&s[i..s.len()], self.seq);

        match find_sub(&s[i..s.len()], self.seq)
        {
            Some(ss) =>
            {
                let (pos, st) = ss;
                if sub_seqs.insert((pos + i, st))
                {
                    self.current_pos += 1;
                    return Some((pos + i, st));
                }
                else 
                { 
                    self.current_pos += 1;
                    return None;
                }
            },
            None => return None  
        }
        
    }
}


pub fn demo_SimpleDNAIter() 
{
    let mut dna_iter = SimpleDNAIter::new("ACGTACGTACGTACGT", "A1-1,C1-1");

    while let Some((pos, subseq)) = dna_iter.next() 
    {
        println!("Found subsequence at position {}: {}", pos, subseq);
        // we can break and stop if we have found what we were looking for
    }
}

// finally we want to implement a real iterator, so that it can be used in a for loop and it may be combined we all the most common iterator methods
// The struct DNAIter is already defined, you have to implement the Iterator trait for it and add lifetimes

struct DNAIter<'a> 
{
    s: &'a str,
    seq: &'a str,
    pos: usize
}


impl<'a> DNAIter<'a> 
{
    pub fn new(s: &'a str, seq: &'a str) -> DNAIter<'a>
    {
        DNAIter { s: s, seq: seq, pos: 0 }
    }
}


impl<'a> Iterator for DNAIter<'a>
{
    type Item = (usize, &'a str);

    fn next(&mut self) -> Option<Self::Item> 
    {
        if self.s.is_empty()
        {
            return None;
        }
        else 
        {
            match find_sub(&self.s[self.pos..], self.seq)
            {
                Some((i, st)) => 
                {
                    self.pos += 1;
                    Some((i, st))
                }, 
                None => 
                {
                    self.pos += 1;
                    None
                }
            }
        }
    }
}


pub fn demo_dna_iter() 
{
    let dna_iter = DNAIter::new("ACGTACGTAAACCCGTACGT", "A1-3,C1-2");

    // now you can combine it with all the iterator modifiers!!!
    dna_iter
        .filter(|(pos, sub)| sub.len() >= 5)
        .for_each(|(pos, sub)| {
            println!(
                "Found subsequence at least long 5 at position {}: {}",
                pos, sub
            )
        });
}

// now let's return an iterator without defining a struct, just using a closure
// the std lib of rust support you with the std::from_fn() function
// we supply a skeleton implementation, you have to fill the closure

// fn subsequence5_iter<'a, 'b>(s: &'a str, seq: &'b str) -> impl Iterator<Item = (usize, &'a str)> 
// {
//     let mut pos = 0;
//     // and any other necessary variable to remember the state
//     std::iter::from_fn(move || 
//     {
//         // if let Some((i, st)) = find_sub(&s[pos..], seq)
//         // {
//         //     pos += 1;
//         //     Some((i + pos, &st[pos..]))
//         // } 
//         // else 
//         // {
//         //     None
//         // }
//         match find_sub(&s[pos..], seq)
//         {
//             Some((i, st)) => 
//             {
//                 pos += 1;
//                 Some((i + pos - 1, st))
//             }, 
//             None => None
//         }
//     })
// }


// fn demo_dna_iter2() 
// {
//     subsequence5_iter("ACGTACGTAAACCGTACGT", "A1-3,C1-2")
//         .filter(|(pos, sub)| sub.len() >= 5)
//         .for_each(|(pos, sub)| {
//             println!(
//                 "Found subsequence at least long 5 at position {}: {}",
//                 pos, sub
//             )
//         });
// }
