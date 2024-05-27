use std::{
    io,
    time::{Duration, Instant},
};

use anyhow::Result;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{prelude::*, widgets::*};

pub struct App {}

impl App {
    pub fn new() -> Result<Self> {
        Ok(Self {})
    }

    pub fn run(&mut self) -> Result<()> {
        // setup terminal
        enable_raw_mode()?;
        let mut stdout = io::stdout();
        execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
        let backend = CrosstermBackend::new(stdout);
        let mut term = Terminal::new(backend)?;

        let _ = self.event_loop(&mut term);

        // restore terminal
        disable_raw_mode()?;
        execute!(
            term.backend_mut(),
            LeaveAlternateScreen,
            DisableMouseCapture
        )?;
        Ok(term.show_cursor()?)
    }

    fn event_loop(&mut self, term: &mut Terminal<CrosstermBackend<io::Stdout>>) -> Result<()> {
        term.draw(|f| self.ui(f))?;

        let tick_rate = Duration::from_millis(250);
        let mut last_tick = Instant::now();
        loop {
            let timeout = tick_rate.saturating_sub(last_tick.elapsed());
            if crossterm::event::poll(timeout)? {
                if let Event::Key(key) = event::read()? {
                    if key.code == KeyCode::Char('q') {
                        return Ok(());
                    }
                }
            }
            if last_tick.elapsed() >= tick_rate {
                last_tick = Instant::now();
            }
        }
    }

    fn ui(&mut self, frame: &mut Frame) {
        let area = frame.size();

        let vertical = Layout::vertical([Constraint::Percentage(10), Constraint::Percentage(90)]);
        let [input, display] = vertical.areas(area);

        self.render_text(frame, input, vec!["Please select one cli to run".into()]);
        self.render_text(frame, display, vec!["just".into(), "rg".into()]);
    }

    fn render_text(&mut self, frame: &mut Frame, area: Rect, text: Vec<String>) {
        let text: Vec<_> = text
            .iter()
            .map(|s| text::Line::from(format!("{s}")))
            .collect();
        let block = Block::default().borders(Borders::ALL).title(Span::styled(
            "CLI",
            Style::default()
                .fg(Color::Magenta)
                .add_modifier(Modifier::BOLD),
        ));
        let paragraph = Paragraph::new(text).block(block).wrap(Wrap { trim: true });
        frame.render_widget(paragraph, area);
    }
}
