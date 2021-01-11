use crate::camera::*;
use crate::cause::*;
use crate::centered_camera::*;
use crate::flow::*;
use crate::space::*;
use crossterm::{
    cursor::{Hide, MoveTo, Show},
    event::{poll, read, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEvent},
    execute, queue,
    style::{Color, Print, ResetColor, SetBackgroundColor, SetForegroundColor},
    terminal::*,
    Result,
};
use std::io::*;
use std::thread::sleep;
use std::time::{Duration, Instant};

pub struct Game {
    out: Stdout,
    space: Space,
    terminal_size: [usize; 2],
    iteration_time: Duration,
    iteration_instant: Instant,
    poll_time: Duration,
    event_handling_duration: Duration,
    event_handling_surplus: Duration,
    min_terminal_size: [usize; 2],
    camera: CenteredCamera,
}

impl Default for Game {
    fn default() -> Game {
        let space = Space::default();
        let out = stdout();
        let terminal_size = [0, 0];
        let iteration_time = Duration::from_millis(500);
        let iteration_instant = Instant::now();
        let poll_time = Duration::from_millis(1);
        let event_handling_duration = Duration::from_millis(8);
        let event_handling_surplus = Duration::from_secs(0);
        let camera = CenteredCamera::default();
        let min_terminal_size = [camera.height(), camera.width()];
        Game {
            camera,
            min_terminal_size,
            event_handling_surplus,
            event_handling_duration,
            poll_time,
            iteration_instant,
            out,
            space,
            terminal_size,
            iteration_time,
        }
    }
}

impl Game {
    fn handle_resize(&mut self, height: u16, width: u16) -> Result<Flow> {
        self.terminal_size = [height as usize, width as usize];
        execute!(self.out, Hide,)?;
        if self.terminal_too_small() {
            return Ok(Flow::Pause(Cause::TerminalTooSmall));
        }
        Ok(Flow::Continue)
    }
    fn handle_key_event(&mut self, event: KeyEvent) -> Flow {
        if event.code == KeyCode::Esc {
            return Flow::Exit;
        }
        Flow::Continue
    }
    fn read_and_handle_event(&mut self) -> Result<Flow> {
        match read()? {
            Event::Key(event) => return Ok(self.handle_key_event(event)),
            Event::Mouse(event) => (),
            Event::Resize(width, height) => return self.handle_resize(height, width),
        }
        Ok(Flow::Continue)
    }
    fn must_handle_events(&mut self) -> bool {
        let elapsed = self.iteration_instant.elapsed();
        if elapsed >= self.event_handling_duration {
            self.event_handling_surplus = elapsed - self.event_handling_duration;
            return false;
        }
        true
    }
    fn handle_events(&mut self) -> Result<Flow> {
        if poll(self.poll_time)? {
            return self.read_and_handle_event();
        }
        Ok(Flow::Continue)
    }
    fn height_index() -> usize {
        0
    }
    fn width_index() -> usize {
        1
    }
    fn min_terminal_width(&self) -> usize {
        self.min_terminal_size[Game::width_index()]
    }
    fn terminal_width(&self) -> usize {
        self.terminal_size[Game::width_index()]
    }
    fn min_terminal_height(&self) -> usize {
        self.min_terminal_size[Game::height_index()]
    }
    fn terminal_height(&self) -> usize {
        self.terminal_size[Game::height_index()]
    }
    fn terminal_too_small(&self) -> bool {
        if self.terminal_height() < self.min_terminal_height()
            || self.terminal_width() < self.min_terminal_width()
        {
            return true;
        }
        false
    }
    fn draw_too_small_message(&mut self) -> Result<()> {
        let height = self.terminal_height();
        let min_height = self.min_terminal_height();
        let width = self.terminal_width();
        let min_width = self.min_terminal_width();
        execute!(
            self.out,
            MoveTo(0, 0),
            Clear(ClearType::All),
            Print("Terminal too small"),
            MoveTo(0, 1),
            Print(format!("height {} minimum {}", height, min_height)),
            MoveTo(0, 2),
            Print(format!("width {} minimum {}", width, min_width)),
        )
    }
    fn draw(&mut self) -> Result<()> {
        if self.terminal_too_small() {
            panic!("terminal too small for drawing");
        }
        execute!(self.out, Hide,)?;
        let space_height = self.space.height();
        let space_width = self.space.width();
        let address = self.camera.position(space_height, space_width);
        let camera_height = self.camera.height();
        let camera_width = self.camera.width();
        for row in 0..camera_height {
            for column in 0..camera_width {
                let character = self.space.draw(row, column);
                queue!(
                    self.out,
                    MoveTo(column as u16, row as u16),
                    Print(character),
                )?;
            }
        }
        self.out.flush()?;
        Ok(())
    }
    fn wait_for_larger_screen(&mut self) -> Result<Flow> {
        self.draw_too_small_message()?;
        loop {
            match read()? {
                Event::Resize(width, height) => match self.handle_resize(height, width) {
                    Ok(flow) => match flow {
                        Flow::Continue => return Ok(Flow::Continue),
                        Flow::Pause(cause) => match cause {
                            Cause::TerminalTooSmall => self.draw_too_small_message()?,
                            _ => Game::extraordinary_pause(cause),
                        },
                        Flow::Exit => Game::extraordinary_exit(),
                    },
                    Err(err) => return Err(err),
                },
                Event::Key(event) => {
                    if event.code == KeyCode::Esc {
                        return Ok(Flow::Exit);
                    }
                }
                _ => (),
            }
        }
    }
    fn handle_pause(&mut self, cause: Cause) -> Result<Flow> {
        match cause {
            Cause::TerminalTooSmall => return self.wait_for_larger_screen(),
            _ => Game::extraordinary_pause(cause),
        }
        Ok(Flow::Continue)
    }
    fn main_loop(&mut self) -> Result<()> {
        loop {
            self.draw()?;
            self.iteration_instant = Instant::now();
            while self.must_handle_events() {
                match self.handle_events() {
                    Ok(flow) => match flow {
                        Flow::Continue => continue,
                        Flow::Pause(cause) => match self.handle_pause(cause) {
                            Ok(flow) => match flow {
                                Flow::Continue => continue,
                                Flow::Pause(cause) => panic!("recursive pause"),
                                Flow::Exit => return Ok(()),
                            },
                            Err(err) => return Err(err),
                        },
                        Flow::Exit => return Ok(()),
                    },
                    Err(err) => return Err(err),
                }
            }
            /*
            let surplus = self.event_handling_surplus;
            execute!(
                self.out,
                MoveTo(0, 2),
                Clear(ClearType::CurrentLine),
                Print(format!("event_handling_surplus {:?}", surplus)),
            )?
            */
        }
    }
    fn extraordinary_pause(cause: Cause) {
        panic!(format!("extraordinary pause with cause {:?}", cause))
    }
    fn extraordinary_exit() {
        panic!("extraordinary exit")
    }
    fn initialize(&mut self) -> Result<Flow> {
        enable_raw_mode()?;
        execute!(
            self.out,
            EnableMouseCapture,
            EnterAlternateScreen,
            Hide,
            MoveTo(0, 0),
        )?;
        let terminal_size = size()?;
        self.terminal_size = [terminal_size.1 as usize, terminal_size.0 as usize];
        if self.terminal_too_small() {
            match self.wait_for_larger_screen() {
                Ok(flow) => match flow {
                    Flow::Continue => (),
                    Flow::Pause(cause) => Game::extraordinary_pause(cause),
                    Flow::Exit => return Ok(Flow::Exit),
                },
                Err(err) => return Err(err),
            }
        }
        Ok(Flow::Continue)
    }
    fn finalize(&mut self) -> Result<()> {
        execute!(
            self.out,
            ResetColor,
            LeaveAlternateScreen,
            DisableMouseCapture
        )?;
        disable_raw_mode()?;
        Ok(())
    }
    pub fn start() -> Result<()> {
        let mut game = Game::default();
        match game.initialize() {
            Ok(flow) => match flow {
                Flow::Continue => (),
                Flow::Exit => return Ok(()),
                Flow::Pause(cause) => Game::extraordinary_pause(cause),
            },
            Err(err) => return Err(err),
        }
        game.main_loop()?;
        game.finalize()
    }
}
