use slugify_extented::MySlug;

fn main() 
{
    let s1 = "i-am-a-slice";
    let s2 = "I am a slice";
    let s3 = "i-am-a-string".to_string();
    let s4 = "I am a string".to_string();

    println!("s1 is a slug: {}", s1.is_slug());
    println!("s2 is a slug: {}", s2.is_slug());
    println!("s3 is a slug: {}", s3.is_slug());
    println!("s3 is a slug: {}", s4.is_slug());

    println!("s1.to_slug: {}", s1.to_slug());
    println!("s2.to_slug: {}", s2.to_slug());
    println!("s3.to_slug: {}", s3.to_slug());
    println!("s4.to_slug: {}", s4.to_slug());
}
