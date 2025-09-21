use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{
    io,
    time::{Duration, Instant},
};
use tui::{
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Frame, Terminal,
};

use crate::app::App;
use crate::fetch::{format_size, detect_system};

pub fn run_tui<B: Backend>(terminal: &mut Terminal<B>, mut app: App) -> io::Result<()> {
    let mut last_tick = Instant::now();
    let tick_rate = Duration::from_millis(250);

    loop {
        terminal.draw(|f| ui(f, &app))?;

        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));

        if crossterm::event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') => return Ok(()),
                    KeyCode::Char('r') => app.update(),
                    KeyCode::Esc => return Ok(()),
                    _ => {}
                }
            }
        }

        if last_tick.elapsed() >= tick_rate {
            last_tick = Instant::now();
        }
    }
}

fn ui<B: Backend>(f: &mut Frame<B>, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints(
            [
                Constraint::Length(3),  // Title
                Constraint::Min(20),    // Main content
                Constraint::Length(3),  // Footer
            ]
            .as_ref(),
        )
        .split(f.size());

    // Title with icon
    let title = Paragraph::new("PackageFetch")
        .style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))
        .alignment(tui::layout::Alignment::Center)
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(title, chunks[0]);

    // Main content - Left side single, Right side split
    let main_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(40), Constraint::Percentage(60)].as_ref())
        .split(chunks[1]);

    // Left side - Extended Package Statistics
    let stats = app.get_package_stats();
    let uptodate_packages = stats.total.saturating_sub(stats.outdated);
    let outdated_percentage = if stats.total > 0 {
        (stats.outdated as f64 / stats.total as f64) * 100.0
    } else {
        0.0
    };

    // Calculate additional statistics
    let pacman_percentage = if stats.total > 0 {
        (stats.pacman as f64 / stats.total as f64) * 100.0
    } else {
        0.0
    };
    let aur_percentage = if stats.total > 0 {
        (stats.aur as f64 / stats.total as f64) * 100.0
    } else {
        0.0
    };

    // Detect system type
    let system_commands = detect_system();
    let is_arch = system_commands.iter().any(|(cmd, _)| *cmd == "pacman");
    let is_debian = system_commands.iter().any(|(cmd, _)| *cmd == "dpkg");
    let is_fedora = system_commands.iter().any(|(cmd, _)| *cmd == "rpm");

    let mut stats_text = vec![
        Spans::from(vec![
            Span::raw("Total packages: "),
            Span::styled(format!("{}", stats.total), Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)),
        ]),
        Spans::from(""),
    ];

    // Show package types based on detected system
    if is_arch {
        stats_text.push(Spans::from(vec![
            Span::raw("Pacman: "),
            Span::styled(format!("{}", stats.pacman), Style::default().fg(Color::Blue)),
            Span::raw(" ("),
            Span::styled(format!("{:.1}%", pacman_percentage), Style::default().fg(Color::Gray)),
            Span::raw(")"),
        ]));
        stats_text.push(Spans::from(vec![
            Span::raw("AUR: "),
            Span::styled(format!("{}", stats.aur), Style::default().fg(Color::Yellow)),
            Span::raw(" ("),
            Span::styled(format!("{:.1}%", aur_percentage), Style::default().fg(Color::Gray)),
            Span::raw(")"),
        ]));
    }

    if is_debian {
        stats_text.push(Spans::from(vec![
            Span::raw("APT: "),
            Span::styled(format!("{}", stats.apt), Style::default().fg(Color::Magenta)),
        ]));
    }

    if is_fedora {
        stats_text.push(Spans::from(vec![
            Span::raw("DNF: "),
            Span::styled(format!("{}", stats.dnf), Style::default().fg(Color::Red)),
        ]));
    }

    // Show Flatpak only if it's available
    if stats.flatpak > 0 {
        stats_text.push(Spans::from(vec![
            Span::raw("Flatpak: "),
            Span::styled(format!("{}", stats.flatpak), Style::default().fg(Color::Cyan)),
        ]));
    }

    stats_text.extend(vec![
        Spans::from(""),
        Spans::from(vec![
            Span::raw("Up to date: "),
            Span::styled(format!("{}", uptodate_packages), Style::default().fg(Color::Green)),
        ]),
        Spans::from(vec![
            Span::raw("Outdated: "),
            Span::styled(format!("{}", stats.outdated), Style::default().fg(Color::Red)),
        ]),
        Spans::from(""),
        Spans::from(vec![
            Span::raw("System health: "),
            Span::styled(
                format!("{:.1}%", 100.0 - outdated_percentage),
                Style::default().fg(if outdated_percentage < 10.0 { Color::Green } else if outdated_percentage < 25.0 { Color::Yellow } else { Color::Red }),
            ),
        ]),
        Spans::from(""),
        Spans::from(vec![
            Span::raw("System: "),
            Span::styled(if is_arch { "Arch Linux" } else if is_debian { "Debian/Ubuntu" } else if is_fedora { "Fedora Linux" } else { "Unknown" }, Style::default().fg(Color::Blue)),
        ]),
        Spans::from(vec![
            Span::raw("Kernel: "),
            Span::styled("6.16.7-zen1-1-zen", Style::default().fg(Color::Gray)),
        ]),
    ]);

    // Add separator
    stats_text.push(Spans::from(""));
    stats_text.push(Spans::from(vec![
        Span::styled("─".repeat(20), Style::default().fg(Color::Gray)),
    ]));

    // Add disk information
    stats_text.push(Spans::from(""));
    stats_text.push(Spans::from(vec![
        Span::styled("Disk Usage", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
    ]));

    for disk in &app.disk_info {
        let usage_color = if disk.usage_percentage < 70.0 {
            Color::Green
        } else if disk.usage_percentage < 90.0 {
            Color::Yellow
        } else {
            Color::Red
        };

        stats_text.push(Spans::from(vec![
            Span::raw(format!("{}: ", disk.mount_point)),
            Span::styled(
                format!("{:.1}%", disk.usage_percentage),
                Style::default().fg(usage_color),
            ),
            Span::raw(format!(" ({}/{})", format_size(disk.used), format_size(disk.total))),
        ]));
    }

    let stats_widget = Paragraph::new(stats_text)
        .block(
            Block::default()
                .title(Spans::from(vec![
                    Span::styled("Package Statistics", Style::default().fg(Color::Green).add_modifier(Modifier::BOLD))
                ]))
                .borders(Borders::ALL)
        );
    f.render_widget(stats_widget, main_chunks[0]);

    // Right side - Split into two parts
    let right_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(60), Constraint::Percentage(40)].as_ref())
        .split(main_chunks[1]);

    // Top right - Outdated Packages
    let outdated_items: Vec<ListItem> = if app.outdated_packages.is_empty() {
        vec![ListItem::new(Spans::from(vec![
            Span::styled("All packages are up to date!", Style::default().fg(Color::Green)),
        ]))]
    } else {
        app.outdated_packages
            .iter()
            .take(10) // Show more outdated packages
            .map(|pkg| {
                ListItem::new(Spans::from(vec![
                    Span::raw(&pkg.name),
                    Span::styled(
                        format!(" [{}]", pkg.source),
                        Style::default().fg(Color::Gray),
                    ),
                ]))
            })
            .collect()
    };

    let outdated_widget = List::new(outdated_items)
        .block(
            Block::default()
                .title(Spans::from(vec![
                    Span::styled("Outdated Packages", Style::default().fg(Color::Red).add_modifier(Modifier::BOLD))
                ]))
                .borders(Borders::ALL)
        );
    f.render_widget(outdated_widget, right_chunks[0]);

    // Bottom right - Recent Packages
    let recent_items: Vec<ListItem> = if app.recent_packages.is_empty() {
        vec![ListItem::new(Spans::from(vec![
            Span::styled("No recent packages found", Style::default().fg(Color::Gray)),
        ]))]
    } else {
        app.recent_packages
            .iter()
            .map(|pkg| {
                ListItem::new(Spans::from(vec![
                    Span::raw(&pkg.name),
                    Span::styled(
                        format!(" [{}]", pkg.source),
                        Style::default().fg(Color::Gray),
                    ),
                ]))
            })
            .collect()
    };

    let recent_widget = List::new(recent_items)
        .block(
            Block::default()
                .title(Spans::from(vec![
                    Span::styled("Recent Packages", Style::default().fg(Color::Blue).add_modifier(Modifier::BOLD))
                ]))
                .borders(Borders::ALL)
        );
    f.render_widget(recent_widget, right_chunks[1]);

    // Footer
    let footer = Paragraph::new("Press 'q' to quit, 'r' to refresh")
        .style(Style::default().fg(Color::Gray))
        .alignment(tui::layout::Alignment::Center);
    f.render_widget(footer, chunks[2]);
}

pub fn setup_terminal() -> io::Result<Terminal<CrosstermBackend<io::Stdout>>> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let terminal = Terminal::new(backend)?;
    Ok(terminal)
}

pub fn restore_terminal(
    terminal: &mut Terminal<CrosstermBackend<io::Stdout>>,
) -> io::Result<()> {
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;
    Ok(())
}