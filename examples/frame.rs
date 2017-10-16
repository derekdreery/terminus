extern crate terminus;
#[macro_use] extern crate error_chain;

use terminus::*;
use std::io::Write;
use std::os::raw::c_ushort;

fn sleep() {
    ::std::thread::sleep(::std::time::Duration::from_millis(2000));
}

fn clear<T: Write + Send>(term: &mut Terminal<T>) -> Result<()> {
    let dims = term.dimensions()?;
    for _ in 0..dims.rows {
        writeln!(term)?;
    }
    Ok(())
}

fn cls<T: Write + Send>(term: &mut Terminal<T>) -> Result<()> {
    let dims = term.dimensions()?;
    for row in 0..dims.rows {
        term.set_position(Position { row, column: 0 })?;
        term.delete_line()?;
    }
    term.flush()?;
    Ok(())
}

fn draw_frame<T: Write + Send>(term: &mut Terminal<T>, text: &str) -> Result<()> {
    cls(term)?;
    let dims = term.dimensions()?;
    let fg = term.foreground_color()?;
    let bg = term.background_color()?;
    term.set_position(Position { row: 0, column: 0 })?;
    // invert colors to draw border
    term.set_foreground_color(bg)?;
    term.set_background_color(fg)?;
    for _ in 0..dims.columns {
        write!(term, " ")?;
    }
    for row in 1..(dims.rows-1) {
        term.set_position(Position { row, column: 0})?;
        write!(term, " ")?;
        term.set_position(Position { row, column: dims.columns - 1 })?;
        write!(term, " ")?;
    }
    term.set_position(Position { row: dims.rows - 1, column: 0 })?;
    for _ in 0..dims.columns {
        write!(term, " ")?;
    }
    // Draw text
    let start = (dims.columns - text.len() as c_ushort) / 2;
    term.set_foreground_color(fg)?;
    term.set_background_color(bg)?;
    term.set_position(Position { row: dims.rows / 2, column: start })?;
    write!(term, "{}", text);
    term.set_position(Position { row: dims.rows - 1, column: 0 })?;
    term.flush();
    sleep();
    Ok(())
}

fn run() -> Result<()> {
    let mut term = stdout()?;
    // We can test for the capabilities we know we will need up front, and change how we run the app
    // depending on what the console supports. In this case, we just bail if we don't have what we 
    // need
    let caps = &[Capability::Position, Capability::Dimensions];
    if ! term.has_capabilities(caps) {
        let mut missing_caps = Vec::new();
        for cap in caps {
            if ! term.has_capability(cap) {
                missing_caps.push(cap);
            }
        }
        bail!("cannot run example, terminal doesn't have required capabilities: {:?}", missing_caps)
    }

    clear(&mut term)?;
    draw_frame(&mut term, "Ignore the following it's not real")?;
    draw_frame(&mut term, "Hi")?;
    draw_frame(&mut term, "")?;
    draw_frame(&mut term, "We've updated your computer")?;
    draw_frame(&mut term, "")?;
    draw_frame(&mut term, "All your files are right where you've left them")?;
    draw_frame(&mut term, "")?;
    draw_frame(&mut term, "We have some new features that we are excited about")?;
    draw_frame(&mut term, "")?;
    writeln!(term)?;
    term.reset()
}

quick_main!(run);