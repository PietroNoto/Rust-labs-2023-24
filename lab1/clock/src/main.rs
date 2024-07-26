use clock::Clock;

pub fn main()
{
    let c2 = Clock::new(24, 91).to_string();
    let c3 = Clock::new(100, 0).to_string();
    let c4 = Clock::new(100, 62).to_string();
    let c1 = Clock::new(3, 0).to_string();
}