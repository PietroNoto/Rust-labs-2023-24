use std::fmt::Display;

#[derive(Debug)]
pub struct Clock
{
    hours: i32,
    minutes: i32
}


impl Display for Clock
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result 
    {
        write!(f, "{}", self.to_string())
    }
}


impl PartialEq for Clock
{
    fn eq(&self, other: &Self) -> bool 
    {
        self.to_string() == other.to_string()
    }
}

impl Clock 
{
    pub fn new(hours: i32, minutes: i32) -> Self 
    {
        let mut new_minutes: i32 = 0;
        let mut new_hours: i32 = 0;
        let mut rem: i32 = 0;
        if minutes.is_negative()
        {
            rem = minutes.abs() / 60 + 1;
            new_minutes = minutes + 60 * rem;
            new_hours = hours - rem + new_minutes / 60;
            new_minutes %= 60;
        }
        else 
        {
            new_minutes = minutes % 60;
            rem = minutes / 60;
            new_hours = hours + rem;
        }
        
        if new_hours.is_negative()
        {
            new_hours = new_hours + 24 * (new_hours.abs() / 24 + 1);
        }

        Clock {hours: new_hours % 24, minutes: new_minutes}
    }


    pub fn add_minutes(&self, minutes: i32) -> Self 
    {
        let mut new_minutes: i32 = self.minutes + minutes;
        let mut new_hours: i32 = 0;
        let mut rem: i32 = 0;

        if new_minutes.is_negative()
        {
            rem = new_minutes.abs() / 60 + 1;
            new_minutes = new_minutes + 60 * rem;
            new_hours = self.hours - rem + new_minutes / 60;
            new_minutes %= 60;
        }
        else
        {
            //norm = new_minutes / 60;
            rem = new_minutes / 60;
            new_minutes = new_minutes % 60;
            new_hours = (self.hours + rem) % 24;
        }

        if new_hours.is_negative()
        {
            new_hours = new_hours + 24 * (new_hours.abs() / 24 + 1);
            new_hours %= 24;
        }

        Clock {hours: new_hours, minutes: new_minutes}
    }


    pub fn to_string(&self) -> String
    {
        let mut h = String::new();
        let mut m = String::new();
        if self.hours < 10
        {
            h = "0".to_string();
        }
        if self.minutes < 10
        {
            m = "0".to_string();
        }
        
        format!("{}{}:{}{}", h, self.hours, m, self.minutes)
    }
}
