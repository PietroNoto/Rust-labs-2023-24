pub trait MySlug
{
    fn to_slug(&self) -> String;
    
    fn is_slug(&self) -> bool;
}


impl<T> MySlug for T where T: AsRef<str>
{
    fn to_slug(&self) -> String 
    {
        let s = self.as_ref();

        if s != "" && s != "-"
        {
            let source = s.chars().collect::<Vec<char>>();
            let mut target = Vec::<char>::new();

            for index in 0..source.len()
            {
                if !source[index].is_alphanumeric()
                {
                    if index < source.len() - 1 && !source[index + 1].is_alphanumeric()
                    {
                        continue;
                    }
                    if index < source.len() - 1
                    {
                        target.push('-');
                    } 
                }
                else 
                {
                    target.push(conv(source[index].to_ascii_lowercase()));
                }
            }
            target.iter().collect::<String>()
        }
        else 
        {
            s.to_string()
        }
    }


    fn is_slug(&self) -> bool 
    {
        let chars = self.as_ref().chars().collect::<Vec<char>>();

        for (i, c) in chars.iter().enumerate()
        {
            if c.is_ascii_uppercase()
            {
                return false;
            }
            else if !c.is_alphanumeric() && *c != '-'
            {
                return false;
            }
            else if i < chars.len() - 1 && *c == '-' && chars[i + 1] == '-'
            {
                return false;
            }
        }
        true
    }
}

fn conv(c: char) -> char
{
    const SUBS_I : &str = "àáâäæãåāăąçćčđďèéêëēėęěğǵḧîïíīįìıİłḿñńǹňôöòóœøōõőṕŕřßśšşșťțûüùúūǘůűųẃẍÿýžźż";
    const SUBS_O: &str = "aaaaaaaaaacccddeeeeeeeegghiiiiiiiilmnnnnoooooooooprrsssssttuuuuuuuuuwxyyzzz";
    
    match SUBS_I.chars().position(|ch| ch == c)
    {
        Some(pos) => 
        {
            let char_array = SUBS_O.to_string().chars().collect::<Vec<char>>();
            return char_array[pos]
        },
        None => return c,
    }
}