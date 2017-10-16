extern crate term;
#[macro_use]
extern crate error_chain;

mod error;
mod cap;
mod sys;

pub use error::*;
pub use cap::{Capability, Color};
use cap::CAPABILITIES;

use std::io;

/// A struct representing the dimensions of a terminal
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct Dimensions {
    pub rows: u16,
    pub columns: u16
}

/// A struct representing a position in the terminal window (e.g. cursor)
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct Position {
    pub row: u16,
    pub column: u16
}

/// A terminal providing extra functionality besides writing bytes.
/// 
/// The terminal is stateful - you set attribues, write some text, and reset attributes 
/// to their defaults with `reset`. Stateless libraries could be built on top of this.
pub struct Terminal<T> {
   inner: sys::Terminal<T>
}

impl<T: io::Write + Send> Terminal<T> {
    /// Create a new terminal from 
    #[inline]
    pub fn new(stream: T) -> Result<Self> {
        let inner = sys::Terminal::new(stream)?;
        Ok(Terminal { inner })
    }

    /// Check whether this terminal has a particular capability
    #[inline]
    pub fn has_capability(&self, cap: &Capability) -> bool {
        self.inner.has_capability(cap)
    }

    /// Check whether this terminal has a set of capabilities
    /// 
    /// # Example
    /// 
    /// ```ignore
    /// let caps = [Capability::Bold, Capability::Dim];
    /// // this will fail if the terminal does not support bold or dim
    /// assert!(term.has_capabilities(caps));
    /// ```
    #[inline]
    pub fn has_capabilities<'a, I, Iter>(&self, caps: I) -> bool 
        where I: IntoIterator<IntoIter=Iter, Item=&'a Capability>,
              Iter: Iterator<Item=&'a Capability>,
    {
        for cap in caps.into_iter() {
            if ! self.has_capability(cap) {
                return false;
            }
        }
        true
    }

    /// Get a list of capabilities supported on the current system
    /// 
    /// This is useful during development
    #[inline]
    pub fn capabilities(&self) -> Vec<Capability> {
        CAPABILITIES.iter()
            .filter(|cap| self.has_capability(cap))
            .map(|e| e.clone())
            .collect()
    }

    /// Reset the terminal attributes to their defaults (for most options, this is "off")
    #[inline]
    pub fn reset(&mut self) -> Result<()> {
        self.inner.reset()
    }

    /// Get the color that will be used to color text.
    #[inline]
    pub fn foreground_color(&self) -> Result<Color> {
        self.inner.foreground_color()
    }

    /// Set the terminal foreground color
    #[inline]
    pub fn set_foreground_color(&mut self, color: Color) -> Result<()> {
        self.inner.set_attr(sys::Attr::ForegroundColor(color))
    }

    /// Get the color that will be used to color the area behind text.
    #[inline]
    pub fn background_color(&self) -> Result<Color> {
        self.inner.background_color()
    }

    /// Set the terminal background color
    #[inline]
    pub fn set_background_color(&mut self, color: Color) -> Result<()> {
        self.inner.set_attr(sys::Attr::BackgroundColor(color))
    }

    /// Get whether text will be written in bold
    #[inline]
    pub fn bold(&mut self) -> Result<bool> {
        self.inner.bold()
    }

    /// Set bold text on or off
    #[inline]
    pub fn set_bold(&mut self, on: bool) -> Result<()> {
        self.inner.set_attr(sys::Attr::Bold(on))
    }

    /// Get whether text will be written with lower brightness
    #[inline]
    pub fn dim(&self) -> Result<bool> {
        bail!(ErrorKind::NotSupported(Capability::Dim))
    }

    /// Set writing dim text on or off
    #[inline]
    pub fn set_dim(&mut self, on: bool) -> Result<()> {
        self.inner.set_attr(sys::Attr::Dim(on))
    }

    /// Get whether text will be written in italics
    #[inline]
    pub fn italic(&self) -> Result<bool> {
        unimplemented!();
    }

    /// Set italic text on or off
    #[inline]
    pub fn set_italic(&mut self, on: bool) -> Result<()> {
        self.inner.set_attr(sys::Attr::Italic(on))
    }

    /// Get whether text will be written underlined
    #[inline]
    pub fn underline(&self) -> Result<bool> {
        unimplemented!();
    }

    /// Set writing underlined text on or off
    #[inline]
    pub fn set_underline(&mut self, on: bool) -> Result<()> {
        self.inner.set_attr(sys::Attr::Underline(on))
    }

    /// Get whether text will be written underlined
    #[inline]
    pub fn blink(&self) -> Result<bool> {
        unimplemented!();
    }

    /// Set writing underlined text on or off
    #[inline]
    pub fn set_blink(&mut self, on: bool) -> Result<()> {
        self.inner.set_attr(sys::Attr::Blink(on))
    }

    /// Get whether text will be written underlined
    #[inline]
    pub fn standout(&self) -> Result<bool> {
        unimplemented!();
    }

    /// Set writing underlined text on or off
    #[inline]
    pub fn set_standout(&mut self, on: bool) -> Result<()> {
        self.inner.set_attr(sys::Attr::Standout(on))
    }

    /// Get whether text will be written underlined
    #[inline]
    pub fn reverse(&self) -> Result<bool> {
        unimplemented!();
    }

    /// Set writing underlined text on or off
    #[inline]
    pub fn set_reverse(&mut self, on: bool) -> Result<()> {
        self.inner.set_attr(sys::Attr::Reverse(on))
    }

    /// Get whether text will be written underlined
    #[inline]
    pub fn secure(&self) -> Result<bool> {
        unimplemented!();
    }

    /// Set writing underlined text on or off
    #[inline]
    pub fn set_secure(&mut self, on: bool) -> Result<()> {
        self.inner.set_attr(sys::Attr::Secure(on))
    }
    
    /// Moves the cursor up one line
    #[inline]
    pub fn cursor_up(&mut self) -> Result<()> {
        self.inner.cursor_up()
    }

    /// Deletes the text from the cursor location to the end of the line
    #[inline]
    pub fn delete_line(&mut self) -> Result<()> {
        self.inner.delete_line()
    }

    /// Returns the cursor to the beginning of the current line
    #[inline]
    pub fn carriage_return(&mut self) -> Result<()> {
        self.inner.carriage_return()
    }

    /// Gets the current cursor position from top-left
    #[inline]
    pub fn position(&self) -> Result<Position> {
        self.inner.position()
    }

    /// Gets the current cursor position from top-left
    #[inline]
    pub fn set_position(&mut self, position: Position) -> Result<()> {
        self.inner.set_position(position)
    }

    /// Gets the dimensions of the terminal
    #[inline]
    pub fn dimensions(&self) -> Result<Dimensions> {
        self.inner.dimensions()
    }

    /// Gets an immutable reference to the wrapped stream
    #[inline]
    pub fn get_ref(&self) -> &T {
        self.inner.get_ref()
    }

    /// Gets a mutable reference to the wrapped stream
    #[inline]
    pub fn get_mut(&mut self) -> &mut T {
        self.inner.get_mut()
    }

    /// Destroy the terminus instance, recovering the wrapped stream
    #[inline]
    pub fn into_inner(self) -> T {
        self.inner.into_inner()
    }
}

impl<T: io::Write> io::Write for Terminal<T> {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.inner.write(buf)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.inner.flush()
    }
}

/// Create a terminal wrapping stdout
pub fn stdout() -> Result<Terminal<io::Stdout>> {
    Terminal::new(io::stdout())
}

/// Create a terminal wrapping stderr
pub fn stderr() -> Result<Terminal<io::Stderr>> {
    Terminal::new(io::stderr())
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
