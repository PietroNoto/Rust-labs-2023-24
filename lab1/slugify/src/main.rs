use clap::Parser;

fn main() 
{
    #[derive(Parser, Debug)]
    struct Args
    {
        slug_in: String,
    }

    let args = Args::parse();
    println!("{}", slugify(&args.slug_in));
}

fn slugify(s: &str) -> String
{
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


#[cfg(test)]
mod tests 
{
    use super::*;

    #[test]
    fn accented_letter()
    {
        assert_eq!(slugify("è"), "e");
    }

    #[test]
    fn non_accented_letter()
    {
        assert_eq!(slugify("E"), "e");
    }  

    #[test]
    fn multiple_words_with_spaces()
    {
        assert_eq!(slugify("Ciao   #  come va?"), "ciao-come-va-");
    } 

    #[test]
    fn string_with_accented_letters()
    {
        assert_eq!(slugify("Perché lui è così?"), "perche-lui-e-cosi-");
    }  

    #[test]
    fn empty_string()
    {
        assert_eq!(slugify(""), "");
    } 

    #[test]
    fn multiple_scores()
    {
        assert_eq!(slugify("---a"), "-a");
    } 

    #[test]
    fn score_at_the_end()
    {
        assert_eq!(slugify("Ciao-"), "ciao");
    }

    #[test]
    fn multiple_scores_at_the_end()
    {
        assert_eq!(slugify("Ciao--"), "ciao");
    }
}
