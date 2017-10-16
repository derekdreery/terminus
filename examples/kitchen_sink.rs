extern crate terminus;
#[macro_use]
extern crate error_chain;

use terminus::{Terminal, Color, Result};

use std::io::Write;
use std::thread;
use std::time;

fn spin<T: Write + Send>(t: &mut Terminal<T>) -> Result<()> {
    let tm = time::Duration::from_millis(100);
    for _ in 0..5 {
        write!(t, "/")?;
        thread::sleep(tm);
        t.carriage_return();
        write!(t, "|")?;
        thread::sleep(tm);
        t.carriage_return();
        write!(t, "\\")?;
        thread::sleep(tm);
        t.carriage_return();
        write!(t, "-")?;
        thread::sleep(tm);
        t.carriage_return();
    }
    writeln!(t)?;
    Ok(())
}

fn run() -> Result<()> {
    let mut term = terminus::stdout()?;
    let caps = term.capabilities();
    writeln!(term, "Capabilities: {:?}", caps)?;
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
    let mut term = terminus::stderr()?;
    write!(term, "to stderr")?;
    ::std::thread::sleep(::std::time::Duration::from_secs(1));
    term.carriage_return()?;
    writeln!(term, "overwrite")?;
    spin(&mut term)?;
    term.reset()?;
    Ok(())
}

quick_main!(run);