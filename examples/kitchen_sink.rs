extern crate terminus;
#[macro_use]
extern crate error_chain;

use terminus::{Color, Result};

use std::io::Write;

fn run() -> Result<()> {
    let mut term = terminus::stdout()?;
    writeln!(term, "normal")?;
    term.set_background_color(Color::Red)?;
    writeln!(term, "normal?")?;
    term.reset()?;
    term.set_foreground_color(Color::BrightCyan)?;
    let color = term.foreground_color()?;
    writeln!(term, "{:?}", color)?;
    term.set_foreground_color(Color::Cyan)?;
    let color = term.foreground_color()?;
    writeln!(term, "{:?}", color)?;
    term.reset()?;
    Ok(())
}

quick_main!(run);