// Copyright 2013-2015 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Windows console handling

// FIXME (#13400): this is only a tiny fraction of the Windows console api

extern crate kernel32;
extern crate winapi;

use cap::Capability;
use std::io::prelude::*;
use std::io;
use std::ptr;

use super::Attr;
use {Error, ErrorKind, Result};
use cap::Color;
use Dimensions;

/// A Terminal implementation which uses the Win32 Console API.
pub struct WinConsole<T> {
    /// The buf is not actually used for terminal commands - they just call into windows to get
    /// the terminal for the currently running process. buf is used to write out text
    buf: T,
    /// Remember the original foreground/background, for reset
    def_foreground: Color,
    /// Remember the original foreground/background, for reset
    def_background: Color,
    foreground: Color,
    background: Color,
}

fn color_to_bits(color: Color) -> u16 {
    // magic numbers from mingw-w64's wincon.h
    // #define FOREGROUND_BLUE 0x1
    // #define FOREGROUND_GREEN 0x2
    // #define FOREGROUND_RED 0x4
    // #define FOREGROUND_INTENSITY 0x8
    // #define BACKGROUND_BLUE 0x10
    // #define BACKGROUND_GREEN 0x20
    // #define BACKGROUND_RED 0x40
    // #define BACKGROUND_INTENSITY 0x80

    match color {
        Color::Black => 0,
        Color::Blue => 0x1,
        Color::Green => 0x2,
        Color::Red => 0x4,
        Color::Yellow => 0x2 | 0x4,
        Color::Magenta => 0x1 | 0x4,
        Color::Cyan => 0x1 | 0x2,
        Color::White => 0x1 | 0x2 | 0x4,
        Color::BrightBlue => 0x1 | 0x8,
        Color::BrightGreen => 0x2 | 0x8,
        Color::BrightRed => 0x4 | 0x8,
        Color::BrightYellow => 0x2 | 0x4 | 0x8,
        Color::BrightMagenta => 0x1 | 0x4 | 0x8,
        Color::BrightCyan => 0x1 | 0x2 | 0x8,
        Color::BrightWhite => 0x1 | 0x2 | 0x4 | 0x8,
    }
}

fn bits_to_color(bits: u16) -> Color {
    match bits {
        0 | 0x8 => Color::Black,
        0x1 => Color::Blue,
        0x2 => Color::Green,
        0x4 => Color::Red,
        0x6 => Color::Yellow,
        0x5 => Color::Magenta,
        0x3 => Color::Cyan,
        0x7 => Color::White,
        0x9 => Color::Blue,
        0xA => Color::Green,
        0xC => Color::Red,
        0xE => Color::Yellow,
        0xD => Color::Magenta,
        0xB => Color::Cyan,
        0xF => Color::White,
        _ => unreachable!(),
    }
}

// Just get a handle to the current console buffer whatever it is
fn conout() -> io::Result<winapi::HANDLE> {
    let name = b"CONOUT$\0";
    let handle = unsafe {
        kernel32::CreateFileA(name.as_ptr() as *const i8,
                              winapi::GENERIC_READ | winapi::GENERIC_WRITE,
                              winapi::FILE_SHARE_WRITE,
                              ptr::null_mut(),
                              winapi::OPEN_EXISTING,
                              0,
                              ptr::null_mut())
    };
    if handle == winapi::INVALID_HANDLE_VALUE {
        Err(io::Error::last_os_error())
    } else {
        Ok(handle)
    }
}

// This test will only pass if it is running in an actual console, probably
#[test]
fn test_conout() {
    assert!(conout().is_ok())
}

impl<T: Write + Send> WinConsole<T> {
    fn apply(&mut self) -> io::Result<()> {
        let out = try!(conout());
        let _unused = self.buf.flush();
        let mut accum: winapi::WORD = 0;
        accum |= color_to_bits(self.foreground);
        accum |= color_to_bits(self.background) << 4;
        unsafe {
            kernel32::SetConsoleTextAttribute(out, accum);
        }
        Ok(())
    }

    /// Returns `Err` whenever the terminal cannot be created for some
    /// reason.
    pub fn new(out: T) -> io::Result<WinConsole<T>> {
        let fg;
        let bg;
        let handle = try!(conout());
        unsafe {
            let mut buffer_info = ::std::mem::uninitialized();
            if kernel32::GetConsoleScreenBufferInfo(handle, &mut buffer_info) != 0 {
                fg = bits_to_color(buffer_info.wAttributes);
                bg = bits_to_color(buffer_info.wAttributes >> 4);
            } else {
                return Err(io::Error::last_os_error());
            }
        }
        Ok(WinConsole {
            buf: out,
            def_foreground: fg,
            def_background: bg,
            foreground: fg,
            background: bg,
        })
    }
}

impl<T: Write> Write for WinConsole<T> {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.buf.write(buf)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.buf.flush()
    }
}

impl<T: Write + Send> WinConsole<T> {

    pub fn attr(&self, attr: Attr) -> Result<bool> {
        match attr {
            Attr::ForegroundColor(f) => {
                Ok(self.foreground == f)
            }
            Attr::BackgroundColor(b) => {
                Ok(self.background == b)
            }
            _ => bail!(ErrorKind::NotSupported(attr.into())),
        }
    }

    pub fn set_attr(&mut self, attr: Attr) -> Result<()> {
        match attr {
            Attr::ForegroundColor(f) => {
                self.foreground = f;
                try!(self.apply());
                Ok(())
            }
            Attr::BackgroundColor(b) => {
                self.background = b;
                try!(self.apply());
                Ok(())
            }
            _ => bail!(ErrorKind::NotSupported(attr.into())),
        }
    }

    /// has_capability for windows console
    // it claims support for underscore and reverse video, but I can't get
    // it to do anything -cmr
    pub fn has_capability(&self, cap: Capability) -> bool {
        match cap {
            Capability::ForegroundColor | Capability::BackgroundColor => true,
            _ => false
        }
    }

    pub fn foreground_color(&self) -> Color {
        self.foreground
    }

    pub fn background_color(&self) -> Color {
        self.background
    }

    pub fn reset(&mut self) -> Result<()> {
        self.foreground = self.def_foreground;
        self.background = self.def_background;
        try!(self.apply());

        Ok(())
    }

    pub fn cursor_up(&mut self) -> Result<()> {
        let _unused = self.buf.flush();
        let handle = try!(conout());
        unsafe {
            let mut buffer_info = ::std::mem::uninitialized();
            if kernel32::GetConsoleScreenBufferInfo(handle, &mut buffer_info) != 0 {
                let (x, y) = (buffer_info.dwCursorPosition.X,
                              buffer_info.dwCursorPosition.Y);
                if y == 0 {
                    // Even though this might want to be a CursorPositionInvalid, on Unix there
                    // is no checking to see if the cursor is already on the first line.
                    // I'm not sure what the ideal behavior is, but I think it'd be silly to have
                    // cursor_up fail in this case.
                    Ok(())
                } else {
                    let pos = winapi::COORD {
                        X: x,
                        Y: y - 1,
                    };
                    if kernel32::SetConsoleCursorPosition(handle, pos) != 0 {
                        Ok(())
                    } else {
                        Err(io::Error::last_os_error().into())
                    }
                }
            } else {
                Err(io::Error::last_os_error().into())
            }
        }
    }

    pub fn delete_line(&mut self) -> Result<()> {
        let _unused = self.buf.flush();
        let handle = try!(conout());
        unsafe {
            let mut buffer_info = ::std::mem::uninitialized();
            if kernel32::GetConsoleScreenBufferInfo(handle, &mut buffer_info) == 0 {
                return Err(io::Error::last_os_error().into());
            }
            let pos = buffer_info.dwCursorPosition;
            let size = buffer_info.dwSize;
            let num = (size.X - pos.X) as winapi::DWORD;
            let mut written = 0;
            if kernel32::FillConsoleOutputCharacterW(handle, 0, num, pos, &mut written) == 0 {
                return Err(io::Error::last_os_error().into());
            }
            if kernel32::FillConsoleOutputAttribute(handle, 0, num, pos, &mut written) == 0 {
                return Err(io::Error::last_os_error().into());
            }
            // Similar reasoning for not failing as in cursor_up -- it doesn't even make
            // sense to
            // me that these APIs could have written 0, unless the terminal is width zero.
            Ok(())
        }
    }

    pub fn carriage_return(&mut self) -> Result<()> {
        let _unused = self.buf.flush();
        let handle = try!(conout());
        unsafe {
            let mut buffer_info = ::std::mem::uninitialized();
            if kernel32::GetConsoleScreenBufferInfo(handle, &mut buffer_info) != 0 {
                let winapi::COORD { X: x, Y: y } = buffer_info.dwCursorPosition;
                if x == 0 {
                    Ok(()) // I changed this from `term` - should it change back
                } else {
                    let pos = winapi::COORD {
                        X: 0,
                        Y: y,
                    };
                    if kernel32::SetConsoleCursorPosition(handle, pos) != 0 {
                        Ok(())
                    } else {
                        Err(io::Error::last_os_error().into())
                    }
                }
            } else {
                Err(io::Error::last_os_error().into())
            }
        }
    }

    pub fn dims(&self) -> Result<Dimensions> {
        let handle = try!(conout());
        unsafe {
            let mut buffer_info = ::std::mem::uninitialized();
            if kernel32::GetConsoleScreenBufferInfo(handle, &mut buffer_info) != 0 {
                Ok(Dimensions {
                    rows: (buffer_info.srWindow.Bottom - buffer_info.srWindow.Top + 1) as u16,
                    columns: (buffer_info.srWindow.Right - buffer_info.srWindow.Left + 1) as u16,
                })
            } else {
                Err(io::Error::last_os_error().into())
            }
        }
    }

    pub fn get_ref<'a>(&'a self) -> &'a T {
        &self.buf
    }

    pub fn get_mut<'a>(&'a mut self) -> &'a mut T {
        &mut self.buf
    }

    pub fn into_inner(self) -> T
        where Self: Sized
    {
        self.buf
    }
}

