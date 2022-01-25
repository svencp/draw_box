/// # Examples
/// '''
/// const STYLE: Style = Style::Doubled;        
/// let mut str1: String = String::new();
/// 
/// let corner1 = Char::upper_left(STYLE);
/// let hor = Char::horizontal(STYLE);
/// let td = Char::t_down(STYLE);
/// let corner2 = Char::upper_right(STYLE);
/// let tl = Char::t_left(STYLE);
/// let corner3 = Char::lower_right(STYLE);
/// let tu = Char::t_up(STYLE);
/// let corner4 = Char::lower_left(STYLE);
/// let tr = Char::t_right(STYLE);
/// let cross = Char::cross(STYLE);
/// 
/// 
/// str1.push_str(&corner1);
/// str1.push_str(&hor);
/// str1.push_str(&td);
/// str1.push_str(&hor);
/// str1.push_str(&corner2);
/// str1.push_str("\n");
/// str1.push_str(&tr);
/// str1.push_str(&hor);
/// str1.push_str(&cross);
/// str1.push_str(&hor);
/// str1.push_str(&tl);
/// str1.push_str("\n");
/// str1.push_str(&corner4);
/// str1.push_str(&hor);
/// str1.push_str(&tu);
/// str1.push_str(&hor);
/// str1.push_str(&corner3);
/// 
/// println!("{}",str1);
/// 
/// let target = format!("{}{}{}{}{}\n{}{}{}{}{}\n{}{}{}{}{}",
/// &corner1, &hor, &td, &hor, &corner2, 
/// &tr, &hor, &cross, &hor, &tl,
/// &corner4, &hor, tu, &hor, &corner3  );
/// 
/// println!("{}",target);
/// 
/// assert_eq!(target, "
/// ╔═╦═╗
/// ╠═╬═╣
/// ╚═╩═╝
/// ".trim());
/// 
/// 


use std::fmt;
use std::str::FromStr;


#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Style {
    Normal,
    Thick,
    Doubled,
}


impl FromStr for Style {

    type Err = String;

    fn from_str(input: &str) -> Result<Style, Self::Err> {
        match input {
            "Normal"  => Ok(Style::Normal),
            "Thick"  => Ok(Style::Thick),
            "Doubled"  => Ok(Style::Doubled),
            _       => Err("Problem reading Style".to_string()),
        }
    }
}


impl fmt::Display for Style {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}



#[derive(Copy, Clone, Debug)]
pub struct Char;


#[rustfmt::skip]
impl Char {
    pub fn upper_left(s: Style) -> String{
        match s {
            Style::Normal  =>  { "┌".to_string() }
            Style::Thick   =>  { "┏".to_string() }
            Style::Doubled =>  { "╔".to_string() } 
        }
    }
    
    pub fn horizontal(s: Style) -> String{
        match s {
            Style::Normal  =>  { "─".to_string() }
            Style::Thick   =>  { "━".to_string() }
            Style::Doubled =>  { "═".to_string() } 
        }
    }
    
    pub fn t_down(s: Style) -> String{
        match s {
            Style::Normal  =>  { "┬".to_string() }
            Style::Thick   =>  { "┳".to_string() }
            Style::Doubled =>  { "╦".to_string() } 
        }
    }
    
    pub fn upper_right(s: Style) -> String{
        match s {
            Style::Normal  =>  { "┐".to_string() }
            Style::Thick   =>  { "┓".to_string() }
            Style::Doubled =>  { "╗".to_string() } 
        }
    }
    
    pub fn vertical(s: Style) -> String{
        match s {
            Style::Normal  =>  { "│".to_string() }
            Style::Thick   =>  { "┃".to_string() }
            Style::Doubled =>  { "║".to_string() } 
        }
    }
    
    pub fn t_left(s: Style) -> String{
        match s {
            Style::Normal  =>  { "┤".to_string() }
            Style::Thick   =>  { "┫".to_string() }
            Style::Doubled =>  { "╣".to_string() } 
        }
    }
    
    pub fn lower_right(s: Style) -> String{
        match s {
            Style::Normal  =>  { "┘".to_string() }
            Style::Thick   =>  { "┛".to_string() }
            Style::Doubled =>  { "╝".to_string() } 
        }
    }
    
    pub fn t_up(s: Style) -> String{
        match s {
            Style::Normal  =>  { "┴".to_string() }
            Style::Thick   =>  { "┻".to_string() }
            Style::Doubled =>  { "╩".to_string() } 
        }
    }
    
    pub fn lower_left(s: Style) -> String{
        match s {
            Style::Normal  =>  { "└".to_string() }
            Style::Thick   =>  { "┗".to_string() }
            Style::Doubled =>  { "╚".to_string() } 
        }
    }
    
    pub fn t_right(s: Style) -> String{
        match s {
            Style::Normal  =>  { "├".to_string() }
            Style::Thick   =>  { "┣".to_string() }
            Style::Doubled =>  { "╠".to_string() } 
        }
    }
    
    pub fn cross(s: Style) -> String{
        match s {
            Style::Normal  =>  { "┼".to_string() }
            Style::Thick   =>  { "╋".to_string() }
            Style::Doubled =>  { "╬".to_string() } 
        }
    }
    
    pub fn empty() -> String{
        " ".to_string()
    }
    
    
    
    
    
    
    
    
}


























#[cfg(test)]
mod tests {
    use super::*;
    
    
    #[test]
    fn t001_make() {
        const STYLE: Style = Style::Thick;        
        let mut str1: String = String::new();
        
        let corner1 = Char::upper_left(STYLE);
        let hor1 = Char::horizontal(STYLE);
        let corner2 = Char::upper_right(STYLE);
        
        str1.push_str(&corner1);
        str1.push_str(&hor1);
        str1.push_str(&hor1);
        str1.push_str(&corner2);
        
        assert_eq!(str1,"┏━━┓".to_string());
    }
    

    #[test]
    fn t002_target() {
        const STYLE: Style = Style::Doubled;        
        let mut str1: String = String::new();
        
        let corner1 = Char::upper_left(STYLE);
        let hor = Char::horizontal(STYLE);
        let td = Char::t_down(STYLE);
        let corner2 = Char::upper_right(STYLE);
        let tl = Char::t_left(STYLE);
        let corner3 = Char::lower_right(STYLE);
        let tu = Char::t_up(STYLE);
        let corner4 = Char::lower_left(STYLE);
        let tr = Char::t_right(STYLE);
        let cross = Char::cross(STYLE);
        
        
        str1.push_str(&corner1);
        str1.push_str(&hor);
        str1.push_str(&td);
        str1.push_str(&hor);
        str1.push_str(&corner2);
        str1.push_str("\n");
        str1.push_str(&tr);
        str1.push_str(&hor);
        str1.push_str(&cross);
        str1.push_str(&hor);
        str1.push_str(&tl);
        str1.push_str("\n");
        str1.push_str(&corner4);
        str1.push_str(&hor);
        str1.push_str(&tu);
        str1.push_str(&hor);
        str1.push_str(&corner3);
        
        println!("{}",str1);
        
        let target = format!("{}{}{}{}{}\n{}{}{}{}{}\n{}{}{}{}{}",
        &corner1, &hor, &td, &hor, &corner2, 
        &tr, &hor, &cross, &hor, &tl,
        &corner4, &hor, tu, &hor, &corner3  );
        
        println!("{}",target);
        
        assert_eq!(target, "
╔═╦═╗
╠═╬═╣
╚═╩═╝
        ".trim());

    } // end of t002
    

    #[test]
    fn t003_fmt() {
        let s: String = Style::Doubled.to_string();
        
        assert_eq!(s,"Doubled".to_string());
    }
    
    
    #[test]
    fn t004_parse() {
        let input = "Normal".to_string();
        let ff = Style::from_str(&input);

        // assert_eq!(1,1);
        assert_eq!(ff.unwrap(),Style::Normal);
    }
    
    
    #[test]
    fn t005_parse() {
        let input2 = "Normol".to_string();
        let ff = Style::from_str(&input2);

        assert_eq!(ff.is_err(),true);
    }















} // end of tests


































