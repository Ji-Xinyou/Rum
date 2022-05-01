use termion::color;

#[derive(PartialEq, Clone, Copy)]
pub enum Type {
    None,
    Number,
    Match, // => for search
    String,
    Character,
    Comment,
    MultilineComment,
    PrimaryKeywords,
    SecondaryKeywords,
}

const COLOR_NUMBER: color::Rgb = color::Rgb(220, 163, 163);
const COLOR_MATCH: color::Rgb = color::Rgb(38, 139, 210);
const COLOR_STRING: color::Rgb = color::Rgb(211, 54, 130);
const COLOR_CHAR: color::Rgb = color::Rgb(108, 113, 196);
const COLOR_COMMENT: color::Rgb = color::Rgb(133, 153, 0);
const COLOR_PRIMARYKW: color::Rgb = color::Rgb(181, 137, 0);
const COLOR_SECONDARYKW: color::Rgb = color::Rgb(42, 161, 152);
const COLOR_NOCOLOR: color::Rgb = color::Rgb(255, 255, 255);

impl Type {
    pub fn to_color(self) -> impl color::Color {
        match self {
           Type::Number => COLOR_NUMBER,
           Type::Match => COLOR_MATCH,
           Type::String => COLOR_STRING,
           Type::Character => COLOR_CHAR,
           Type::Comment | Type::MultilineComment => COLOR_COMMENT,
           Type::PrimaryKeywords => COLOR_PRIMARYKW,
           Type::SecondaryKeywords => COLOR_SECONDARYKW,
           _ => COLOR_NOCOLOR,
        }
    }
}