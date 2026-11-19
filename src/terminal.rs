use std::io::{self, Stderr, stderr};

use crossterm::terminal::{self, EnterAlternateScreen, LeaveAlternateScreen};
use ratwrap::ratatui::{
    CompletedFrame, Frame, Terminal, backend::CrosstermBackend, layout::Size,
};

pub struct TerminalHandler {
    terminal: Terminal<CrosstermBackend<Stderr>>,
}

impl TerminalHandler {
    pub fn new() -> Result<Self, io::Error> {
        crossterm::execute!(io::stderr(), EnterAlternateScreen)?;
        terminal::enable_raw_mode()?;

        let panic_hook = std::panic::take_hook();
        std::panic::set_hook(Box::new(move |panic_info| {
            let _ = Self::reset();
            panic_hook(panic_info);
        }));

        let terminal = Terminal::new(CrosstermBackend::new(stderr()))?;

        Ok(Self { terminal })
    }

    pub fn size(&self) -> Result<Size, io::Error> {
        self.terminal.size()
    }

    pub fn draw<F>(
        &mut self,
        callback: F,
    ) -> Result<CompletedFrame<'_>, io::Error>
    where
        F: FnOnce(&mut Frame),
    {
        self.terminal.draw(callback)
    }

    fn reset() -> Result<(), io::Error> {
        crossterm::execute!(io::stderr(), LeaveAlternateScreen)?;
        terminal::disable_raw_mode()?;
        Ok(())
    }
}

impl Drop for TerminalHandler {
    fn drop(&mut self) {
        if let Err(e) = TerminalHandler::reset() {
            eprintln!("Failed to restore terminal: {e}");
            eprintln!("Run `reset` to recover.")
        }
    }
}
