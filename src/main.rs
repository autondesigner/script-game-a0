#![allow(warnings)]

use std::io::*;

use crossterm::{
    cursor::MoveTo,
    event::{read, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    style::{Color, Print, ResetColor, SetBackgroundColor, SetForegroundColor},
    terminal::*,
    Result,
};

fn event_loop(stdout: &mut Stdout) -> Result<()> {
    loop {
        // `read()` blocks until an `Event` is available
        match read()? {
            Event::Key(event) => {
                if event.code == KeyCode::Esc {
                    return Ok(());
                }
            }
            Event::Mouse(event) => execute!(
                stdout,
                Clear(ClearType::All),
                MoveTo(0, 0),
                Print(format!("Mouse event {:?}", event)),
            )?,
            Event::Resize(width, height) => execute!(
                stdout,
                Clear(ClearType::All),
                MoveTo(0, 0),
                Print(format!("Resize event {} {}", width, height)),
            )?,
        }
    }
}

fn main() -> Result<()> {
    let mut stdout = stdout();
    // using the macro
    enable_raw_mode()?;

    execute!(
        stdout,
        EnableMouseCapture,
        EnterAlternateScreen,
        //SetForegroundColor(Color::White),
        //SetBackgroundColor(Color::Black),
        //Clear(ClearType::All),
        MoveTo(0, 0),
        //Print("Hello script game a0"),
    )?;

    event_loop(&mut stdout)?;
    execute!(
        stdout,
        ResetColor,
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    disable_raw_mode()?;
    Ok(())
}
