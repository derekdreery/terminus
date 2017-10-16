//! Platform-specific implementation of a terminal

use cap::Capability;
use error::{ErrorKind, Result};
use {Position, Dimensions};

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
    pub fn has_capability(&self, cap: &Capability) -> bool {
        match self {
            &Terminal::WinConsole(ref console) => console.has_capability(*cap)
        }
    }

    /// Reset the terminal to default values
    pub fn reset(&mut self) -> Result<()> {
        match self {
            &mut Terminal::WinConsole(ref mut console) => console.reset(),
        }
    }

    /// True if bold is set, false if not
    pub fn bold(&self) -> Result<bool> {
        match self {
            &Terminal::WinConsole(_) => {
                bail!(ErrorKind::NotSupported(Capability::Bold));
            }
        }
    }

    /// Moves the cursor up one line
    pub fn cursor_up(&mut self) -> Result<()> {
        match self {
            &mut Terminal::WinConsole(ref mut console) => console.cursor_up(),
        }
    }

    /// Deletes the text from the cursor location to the end of the line
    pub fn delete_line(&mut self) -> Result<()> {
        match self {
            &mut Terminal::WinConsole(ref mut console) => console.delete_line(),
        }
    }

    /// Return to the beginning of the current line
    pub fn carriage_return(&mut self) -> Result<()> {
        match self {
            &mut Terminal::WinConsole(ref mut console) => console.carriage_return(),
        }
    }

    /// Gets the current position of the cursor
    pub fn position(&self) -> Result<Position> {
        match self {
            &Terminal::WinConsole(ref console) => console.position(),
        }
    }

    /// Sets the position of the cursor
    pub fn set_position(&mut self, pos: Position) -> Result<()> {
        match self {
            &mut Terminal::WinConsole(ref mut console) => console.set_position(pos),
        }
    }

    /// Gets the dimensions of the terminal
    pub fn dimensions(&self) -> Result<Dimensions> {
        match self {
            &Terminal::WinConsole(ref console) => console.dimensions(),
        }
    }

    /// Get immutable reference to underlying stream
    pub fn get_ref(&self) -> &T {
        match self {
            &Terminal::WinConsole(ref console) => console.get_ref(),
        }
    }

    /// Get mutable reference to underlying stream
    pub fn get_mut(&mut self) -> &mut T {
        match self {
            &mut Terminal::WinConsole(ref mut console) => console.get_mut(),
        }
    }

    /// Get mutable reference to underlying stream
    pub fn into_inner(self) -> T {
        match self {
            Terminal::WinConsole(console) => console.into_inner(),
        }
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
