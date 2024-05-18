// log for tui.rs -> in graph trade function

use crate::formatter::{LogFormatter, Logs};
use crate::scanner::LogStats;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::fmt;
use std::{error::Error, io};
use tui::{
    backend::CrosstermBackend,
    layout::{Constraint, Corner, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, List, ListItem},
    Terminal,
};

impl fmt::Display for LogFormatter {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            LogFormatter::Info => write!(f, "INFO"),
            LogFormatter::Warning => write!(f, "WARNING"),
            LogFormatter::Error => write!(f, "ERROR"),
            LogFormatter::Trace => write!(f, "TRACE"),
        }
    }
}

pub struct App {
    logs: Logs,
    stats: LogStats,
}

impl App {
    pub fn new(logs: Logs, stats: LogStats) -> Self {
        App { logs, stats }
    }

    pub fn render(
        &mut self,
        terminal: &mut Terminal<CrosstermBackend<io::Stdout>>,
    ) -> Result<(), Box<dyn Error>> {
        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
            .split(terminal.size()?);

        let mut info_items: Vec<ListItem> = Vec::new();
        let mut warning_items: Vec<ListItem> = Vec::new();
        let mut error_items: Vec<ListItem> = Vec::new();
        let mut trace_items: Vec<ListItem> = Vec::new();

        for entry in &self.logs.entries {
            let level = match entry.level {
                LogFormatter::Info => Style::default().fg(Color::Blue),
                LogFormatter::Warning => Style::default().fg(Color::Yellow),
                LogFormatter::Error => Style::default().fg(Color::Red),
                LogFormatter::Trace => Style::default().fg(Color::White),
            };

            let message = Spans::from(vec![Span::styled(entry.message.clone(), level)]);
            let metadata = Spans::from(vec![
                Span::styled(format!("{}", entry.level), level),
                Span::raw(" "),
                Span::styled(
                    entry.date.clone(),
                    Style::default().add_modifier(Modifier::ITALIC),
                ),
            ]);

            let item = ListItem::new(vec![metadata, Spans::from(""), message]);

            match entry.level {
                LogFormatter::Info => info_items.push(item),
                LogFormatter::Warning => warning_items.push(item),
                LogFormatter::Error => error_items.push(item),
                LogFormatter::Trace => trace_items.push(item),
            }
        }

        let info_list = List::new(info_items)
            .block(Block::default().borders(Borders::ALL).title("Info Logs"))
            .start_corner(Corner::TopLeft);

        let warning_list = List::new(warning_items)
            .block(Block::default().borders(Borders::ALL).title("Warning Logs"))
            .start_corner(Corner::TopRight);

        let error_list = List::new(error_items)
            .block(Block::default().borders(Borders::ALL).title("Error Logs"))
            .start_corner(Corner::BottomLeft);

        let trace_list = List::new(trace_items)
            .block(Block::default().borders(Borders::ALL).title("Trace Logs"))
            .start_corner(Corner::BottomRight);

        let stats_list = List::new(vec![
            ListItem::new(vec![Spans::from(format!(
                "Total messages: {}",
                self.stats.total_messages
            ))]),
            ListItem::new(vec![Spans::from(format!(
                "Info messages: {}",
                self.stats.info_messages
            ))]),
            ListItem::new(vec![Spans::from(format!(
                "Warning messages: {}",
                self.stats.warning_messages
            ))]),
            ListItem::new(vec![Spans::from(format!(
                "Error messages: {}",
                self.stats.error_messages
            ))]),
            ListItem::new(vec![Spans::from(format!(
                "Trace messages: {}",
                self.stats.trace_messages
            ))]),
        ])
        .block(Block::default().borders(Borders::ALL).title("Stats"));

        terminal.draw(|f| {
            let rects = Layout::default()
                .direction(Direction::Horizontal)
                .constraints(
                    [
                        Constraint::Percentage(25),
                        Constraint::Percentage(25),
                        Constraint::Percentage(25),
                        Constraint::Percentage(25),
                    ]
                    .as_ref(),
                )
                .split(f.size());

            let stats_rect = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Percentage(80), Constraint::Percentage(20)].as_ref())
                .split(rects[0])[1];

            f.render_widget(info_list, rects[0]);
            f.render_widget(warning_list, rects[1]);
            f.render_widget(error_list, rects[2]);
            f.render_widget(trace_list, rects[3]);
            f.render_widget(stats_list, stats_rect);
        })?;

        Ok(())
    }
}

pub fn run_app(logs: Logs, stats: LogStats) -> eyre::Result<()> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::new(logs, stats);

    loop {
        let _ = app.render(&mut terminal);

        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Char('q') => break,
                _ => {}
            }
        }
    }

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;
    Ok(())
}
