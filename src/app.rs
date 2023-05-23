use std::fmt;
#[derive(Clone, Debug, PartialEq)]
pub enum App {
    Sui,
    None,
}

impl<'a> From<&'a str> for App {
    fn from(value: &'a str) -> Self {
        let c = value.as_bytes();
        match c {
            b"sui" => Self::Sui,
            _ => Self::None,
        }
    }
}
impl fmt::Display for App {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let t = match *self {
            Self::Sui => "sui",
            Self::None => "None",
        };
        write!(f, "{}", t)
    }
}