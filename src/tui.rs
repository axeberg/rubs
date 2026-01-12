//! Minimal TUI for interactive passphrase generation.

use crate::PassphraseInfo;
use crossterm::{
    ExecutableCommand,
    event::{self, Event, KeyCode, KeyEventKind},
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::{
    Frame, Terminal,
    layout::{Constraint, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span, Text},
    widgets::{Block, Borders, Paragraph, Wrap},
};
use std::collections::HashSet;
use std::io::{self, stdout};

pub struct App {
    wordlist: HashSet<String>,
    bits: u32,
    current: PassphraseInfo,
}

impl App {
    pub fn new(wordlist: HashSet<String>, bits: u32) -> Self {
        let current = crate::generate(bits, &wordlist);
        Self {
            wordlist,
            bits,
            current,
        }
    }

    fn regenerate(&mut self) {
        self.current = crate::generate(self.bits, &self.wordlist);
    }
}

pub fn run(wordlist: HashSet<String>, bits: u32) -> io::Result<()> {
    enable_raw_mode()?;
    stdout().execute(EnterAlternateScreen)?;

    let mut terminal = Terminal::new(ratatui::backend::CrosstermBackend::new(stdout()))?;
    let mut app = App::new(wordlist, bits);

    loop {
        terminal.draw(|frame| draw(frame, &app))?;

        if let Event::Key(key) = event::read()?
            && key.kind == KeyEventKind::Press
        {
            match key.code {
                KeyCode::Char('q') | KeyCode::Esc => break,
                KeyCode::Char(' ') | KeyCode::Enter | KeyCode::Char('r') => {
                    app.regenerate();
                }
                _ => {}
            }
        }
    }

    disable_raw_mode()?;
    stdout().execute(LeaveAlternateScreen)?;
    Ok(())
}

fn draw(frame: &mut Frame, app: &App) {
    let area = frame.area();

    let chunks = Layout::vertical([
        Constraint::Length(3), // Title
        Constraint::Min(5),    // Passphrase
        Constraint::Length(3), // Stats
        Constraint::Length(2), // Help
    ])
    .split(area);

    // Title
    let title = Paragraph::new("rubs - passphrase generator")
        .style(
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        )
        .block(Block::default().borders(Borders::BOTTOM));
    frame.render_widget(title, chunks[0]);

    // Passphrase display
    let passphrase = app.current.passphrase();
    let passphrase_widget = Paragraph::new(Text::from(vec![
        Line::from(""),
        Line::from(Span::styled(
            &passphrase,
            Style::default()
                .fg(Color::Green)
                .add_modifier(Modifier::BOLD),
        )),
    ]))
    .wrap(Wrap { trim: false })
    .block(
        Block::default()
            .title(" Passphrase ")
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Blue)),
    );
    frame.render_widget(passphrase_widget, chunks[1]);

    // Statistics
    let stats = format!(
        "{} words | {:.1} bits | {} word vocabulary ({:.1} bits/word)",
        app.current.words.len(),
        app.current.total_bits(),
        app.current.wordlist_size,
        app.current.bits_per_word
    );
    let stats_widget = Paragraph::new(stats)
        .style(Style::default().fg(Color::Yellow))
        .block(Block::default().borders(Borders::TOP));
    frame.render_widget(stats_widget, chunks[2]);

    // Help
    let help = Paragraph::new(" [Space/Enter/r] Generate new  [q/Esc] Quit")
        .style(Style::default().fg(Color::DarkGray));
    frame.render_widget(help, chunks[3]);
}
