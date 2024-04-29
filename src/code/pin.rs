use std::fmt::Display;

#[derive (PartialEq, Eq, Debug, Copy, Clone, Hash)]
#[allow(dead_code)]
pub enum PinColour {
    Blue,
    Green,
    Yellow,
    Orange,
    Red,
    Purple,
    Black,
    White,
    Brown,
    
    Empty
}

impl Display for PinColour {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PinColour::Blue     => write!(f, stringify!(Blue)),
            PinColour::Green    => write!(f, stringify!(Green)),
            PinColour::Yellow   => write!(f, stringify!(Yellow)),
            PinColour::Orange   => write!(f, stringify!(Orange)),
            PinColour::Red      => write!(f, stringify!(Red)),
            PinColour::Purple   => write!(f, stringify!(Purple)),
            PinColour::Black    => write!(f, stringify!(Black)),
            PinColour::White    => write!(f, stringify!(White)),
            PinColour::Brown    => write!(f, stringify!(Brown)),
            PinColour::Empty    => write!(f, stringify!(Empty)),
            
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_equality() {
        assert_eq!(PinColour::Blue, PinColour::Blue);
        assert_ne!(PinColour::Blue, PinColour::Green);
    }
}
