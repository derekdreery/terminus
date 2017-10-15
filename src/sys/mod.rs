//! Platform-specific implementation of a terminal

use cap::Capability;
use error::{ErrorKind, Result};
use Dimensions;

#[cfg(windows)]
mod win;
#[cfg(windows)]
use self::win::WinConsole;

use std::io;
use cap::Color;

/// Terminal attributes that it is possible to set
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Attr {
    Bold(bool),
    Dim(bool),
    Italic(bool),
    Underline(bool),
    Blink(bool),
    Standout(bool),
    Reverse(bool),
    Secure(bool),
    ForegroundColor(Color),
    BackgroundColor(Color)
}

// allow attributes to be converted into capabilities, for error handling

impl Into<Capability> for Attr {
    fn into(self) -> Capability {
        match self {
            Attr::Bold(_) => Capability::Bold,
            Attr::Dim(_) => Capability::Dim,
            Attr::Italic(_) => Capability::Italic,
            Attr::Underline(_) => Capability::Underline,
            Attr::Blink(_) => Capability::Blink,
            Attr::Standout(_) => Capability::Standout,
            Attr::Reverse(_) => Capability::Reverse,
            Attr::Secure(_) => Capability::Secure,
            Attr::ForegroundColor(_) => Capability::ForegroundColor,
            Attr::BackgroundColor(_) => Capability::BackgroundColor
        }
    }
}

/// Inner terminal with platform-specific implementations
pub enum Terminal<T> {
    //TerminfoTerminal(TerminfoTerminal),
    #[cfg(windows)]
    WinConsole(WinConsole<T>)
}

impl<T: io::Write + Send> Terminal<T> {
    pub fn new(stream: T) -> Result<Self> {
        Ok(Terminal::WinConsole(WinConsole::new(stream)?))
    }
}

impl<T: io::Write + Send> Terminal<T> {

    /// Gets the foreground color
    pub fn foreground_color(&self) -> Result<Color> {
        match self {
            &Terminal::WinConsole(ref console) => Ok(console.foreground_color()),
        }
    }

    /// Gets the background color
    pub fn background_color(&self) -> Result<Color> {
        match self {
            &Terminal::WinConsole(ref console) => Ok(console.background_color()),
        }
    }

    /// Set the state of an attribute
    pub fn set_attr(&mut self, attr: Attr) -> Result<()> {
        match self {
            &mut Terminal::WinConsole(ref mut console) => console.set_attr(attr),
        }
    }

    /// Check for support for an attribute
    pub fn has_capability(&self, cap: Capability) -> bool {
        match self {
            &Terminal::WinConsole(ref console) => console.has_capability(cap)
        }
    }

    pub fn reset(&mut self) -> Result<()> {
        match self {
            &mut Terminal::WinConsole(ref mut console) => console.reset(),
        }
    }

    /// Moves the cursor up one line
    pub fn cursor_up(&mut self) -> Result<()> {
        unimplemented!();
    }

    /// Deletes the text from the cursor location to the end of the line
    pub fn delete_line(&mut self) -> Result<()> {
        unimplemented!();
    }

    /// Gets the dimensions of the terminal
    pub fn dimensions(&self) -> Result<Dimensions> {
        unimplemented!();
    }
}

impl<T: io::Write> io::Write for Terminal<T> {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        match self {
            &mut Terminal::WinConsole(ref mut console) => console.write(buf),
        }
    }

    fn flush(&mut self) -> io::Result<()> {
        match self {
            &mut Terminal::WinConsole(ref mut console) => console.flush(),
        }
    }
}
