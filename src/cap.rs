//! The color/terminal capability enums and associated helper methods

use std::fmt;

/// All available capabilities, can be filtered to get a list of supported capabilities
pub const CAPABILITIES: &'static [Capability] = 
    &[Capability::Bold, Capability::Dim, Capability::Italic, Capability::Underline, 
      Capability::Blink, Capability::Standout, Capability::Reverse, Capability::Secure, 
      Capability::ForegroundColor, Capability::BackgroundColor, Capability::Reset, 
      Capability::Position, Capability::Dimensions];

/// The primary colors used in a terminal.
/// 
/// They correspond to 1 bit each for read, green, blue, and a bit for bright.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum Color {
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
    BrightRed,
    BrightGreen,
    BrightYellow,
    BrightBlue,
    BrightMagenta,
    BrightCyan,
    BrightWhite
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &Color::Black => write!(f, "black"),
            &Color::Red => write!(f, "red"),
            &Color::Green => write!(f, "green"),
            &Color::Yellow => write!(f, "yellow"),
            &Color::Blue => write!(f, "blue"),
            &Color::Magenta => write!(f, "magenta"),
            &Color::Cyan => write!(f, "cyan"),
            &Color::White => write!(f, "white"),
            &Color::BrightRed => write!(f, "bright red"),
            &Color::BrightGreen => write!(f, "bright green"),
            &Color::BrightYellow => write!(f, "bright yellow"),
            &Color::BrightBlue => write!(f, "bright blue"),
            &Color::BrightMagenta => write!(f, "bright magenta"),
            &Color::BrightCyan => write!(f, "bright cyan"),
            &Color::BrightWhite => write!(f, "bright white"),
        }
    }
}

/// Potential terminal capabilities
/// 
/// These capabilities can be tested for, letting you know what a terminal is capable of on a
/// particular platform. 
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum Capability {
    /// Text can be bold
    Bold,
    /// Text can be dim
    Dim,
    /// Text can be italic
    Italic,
    /// Text can be underlined
    Underline,
    /// Text can blink on and off
    Blink,
    /// Text can be standout
    Standout,
    /// Text can be printed in reverse
    Reverse,
    /// Can accept secure text (not displayed on screen - for passwords)
    Secure,
    /// Can change the text color
    ForegroundColor,
    /// Can change the background color
    BackgroundColor,
    /// Whether the terminal can be reset to defaults
    Reset,
    /// Whether the terminal can give and set the cursor position
    Position,
    /// Whether we can find out the screen dimensions
    Dimensions,
}

impl fmt::Display for Capability {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &Capability::Bold => write!(f, "bold"),
            &Capability::Dim => write!(f, "dim"),
            &Capability::Italic => write!(f, "italic"),
            &Capability::Underline => write!(f, "underline"),
            &Capability::Blink => write!(f, "blink"),
            &Capability::Standout => write!(f, "standout"),
            &Capability::Reverse => write!(f, "reverse"),
            &Capability::Secure => write!(f, "secure"),
            &Capability::ForegroundColor => write!(f, "foreground color"),
            &Capability::BackgroundColor => write!(f, "background color"),
            &Capability::Reset => write!(f, "reset"),
            &Capability::Position => write!(f, "position"),
            &Capability::Dimensions => write!(f, "dimensions"),
        }
    }
}
