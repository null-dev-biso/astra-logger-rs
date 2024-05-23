use crate::formatter::{LogFormatter, Logs};
use crate::scanner::LogStats;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{error::Error, fmt, io};
use tui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Span, Spans, Text},
    widgets::{Block, Borders, List, ListItem, Paragraph, Tabs, Wrap},
    Terminal,
};

// Структура для представления эллиптической кривой
struct EllipticCurve {
    a: f64,
    b: f64,
}

impl EllipticCurve {
    fn new(a: f64, b: f64) -> Self {
        EllipticCurve { a, b }
    }

    fn calculate_points(&self, x_min: f64, x_max: f64, step: f64) -> Vec<(f64, f64)> {
        let mut points = Vec::new();
        let mut x = x_min;

        while x <= x_max {
            let y2 = x * x * x + self.a * x + self.b;
            if y2 >= 0.0 {
                let y = y2.sqrt();
                points.push((x, y));
                points.push((x, -y));
            }
            x += step;
        }

        points
    }
}

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
    filter: Option<LogFormatter>,
    tab: usize,
    points: Vec<(f64, f64)>,
}

impl App {
    pub fn new(logs: Logs, stats: LogStats, filter: Option<LogFormatter>) -> Self {
        // Используем значения из stats для a и b
        let a = stats.error_messages as f64 / stats.total_messages as f64;
        let b = stats.warning_messages as f64 / stats.total_messages as f64;
        let curve = EllipticCurve::new(a, b);
        let points = curve.calculate_points(-5.0, 5.0, 0.1);
        App {
            logs,
            stats,
            filter,
            tab: 0,
            points,
        }
    }

    pub fn render(
        &mut self,
        terminal: &mut Terminal<CrosstermBackend<io::Stdout>>,
    ) -> Result<(), Box<dyn Error>> {
        let size = terminal.size()?;
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(3), Constraint::Min(80)].as_ref())
            .split(size);

        let titles = ["Logs", "Elliptic Curve"]
            .iter()
            .cloned()
            .map(Spans::from)
            .collect();

        let tabs = Tabs::new(titles)
            .block(Block::default().borders(Borders::ALL).title("Menu"))
            .select(self.tab)
            .style(Style::default().fg(Color::White))
            .highlight_style(
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD),
            );

        terminal.draw(|f| {
            f.render_widget(tabs, chunks[0]);
            match self.tab {
                0 => self.render_logs(f, chunks[1]),
                1 => self.render_curve(f, chunks[1]),
                _ => {}
            }
        })?;

        Ok(())
    }

    fn render_logs<B: tui::backend::Backend>(
        &self,
        f: &mut tui::Frame<B>,
        area: tui::layout::Rect,
    ) {
        let log_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(if self.filter.is_some() {
                [Constraint::Percentage(100)].as_ref()
            } else {
                [
                    Constraint::Percentage(25),
                    Constraint::Percentage(25),
                    Constraint::Percentage(25),
                    Constraint::Percentage(25),
                ]
                .as_ref()
            })
            .split(area);

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

        let info_list =
            List::new(info_items).block(Block::default().borders(Borders::ALL).title("Info Logs"));

        let warning_list = List::new(warning_items)
            .block(Block::default().borders(Borders::ALL).title("Warning Logs"));

        let error_list = List::new(error_items)
            .block(Block::default().borders(Borders::ALL).title("Error Logs"));

        let trace_list = List::new(trace_items)
            .block(Block::default().borders(Borders::ALL).title("Trace Logs"));

        let stats_list = Paragraph::new(Text::from(vec![
            Spans::from(format!("Total messages: {}", self.stats.total_messages)),
            Spans::from(format!("Info messages: {}", self.stats.info_messages)),
            Spans::from(format!("Warning messages: {}", self.stats.warning_messages)),
            Spans::from(format!("Error messages: {}", self.stats.error_messages)),
            Spans::from(format!("Trace messages: {}", self.stats.trace_messages)),
        ]))
        .block(Block::default().borders(Borders::ALL).title("Stats"))
        .wrap(Wrap { trim: true });

        if let Some(filter) = &self.filter {
            let filtered_list = match filter {
                LogFormatter::Info => info_list,
                LogFormatter::Warning => warning_list,
                LogFormatter::Error => error_list,
                LogFormatter::Trace => trace_list,
            };
            f.render_widget(filtered_list, log_chunks[0]);
        } else {
            f.render_widget(info_list, log_chunks[0]);
            f.render_widget(warning_list, log_chunks[1]);
            f.render_widget(error_list, log_chunks[2]);
            f.render_widget(trace_list, log_chunks[3]);
        }
        f.render_widget(stats_list, area);
    }

    fn render_curve<B: tui::backend::Backend>(
        &self,
        f: &mut tui::Frame<B>,
        area: tui::layout::Rect,
    ) {
        let datasets = vec![tui::widgets::Dataset::default()
            .name("Elliptic Curve")
            .marker(tui::symbols::Marker::Dot)
            .style(Style::default().fg(Color::Cyan))
            .data(&self.points)];

        let chart = tui::widgets::Chart::new(datasets)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("Elliptic Curve"),
            )
            .x_axis(
                tui::widgets::Axis::default()
                    .title("X Axis")
                    .style(Style::default().fg(Color::Gray))
                    .bounds([-5.0, 5.0])
                    .labels(vec![
                        Span::styled("-5", Style::default().add_modifier(Modifier::BOLD)),
                        Span::raw("0"),
                        Span::styled("5", Style::default().add_modifier(Modifier::BOLD)),
                    ]),
            )
            .y_axis(
                tui::widgets::Axis::default()
                    .title("Y Axis")
                    .style(Style::default().fg(Color::Gray))
                    .bounds([-10.0, 10.0])
                    .labels(vec![
                        Span::styled("-10", Style::default().add_modifier(Modifier::BOLD)),
                        Span::raw("0"),
                        Span::styled("10", Style::default().add_modifier(Modifier::BOLD)),
                    ]),
            );

        f.render_widget(chart, area);
    }
}

pub fn run_app(logs: Logs, stats: LogStats, filter: Option<LogFormatter>) -> eyre::Result<()> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::new(logs, stats, filter);

    loop {
        let _ = app.render(&mut terminal);

        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Char('q') => break,
                KeyCode::Char('t') => app.tab = (app.tab + 1) % 2,
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
