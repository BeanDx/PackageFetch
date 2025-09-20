use crate::app::App;
use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Frame,
};

pub fn render_ascii_ui(app: &App) {
    let stats = app.get_package_stats();
    
    // Output block dimensions
    let total_box_width = 59;
    let inner_content_width = total_box_width - 4;
    
    // Helper function for text centering
    let center_text = |text: &str, width: usize| {
        let text_len = text.chars().count();
        if text_len >= width {
            return text.to_string();
        }
        let padding = width - text_len;
        let left_padding = padding / 2;
        let right_padding = padding - left_padding;
        format!("{}{}{}", " ".repeat(left_padding), text, " ".repeat(right_padding))
    };
    
    // Helper function for formatting content lines
    let format_line_content = |content: &str| {
        format!("| {:<width$} |", content, width = inner_content_width)
    };
    
    // Header output
    println!("+{}", "-".repeat(total_box_width - 2));
    println!("|{}|", center_text("PackageFetch", inner_content_width + 2));
    println!("|{}|", " ".repeat(inner_content_width + 2));
    
    // "Recently Installed" section
    println!("{}", format_line_content("[Recently Installed]"));
    for package in &app.recent_packages {
        println!("{}", format_line_content(&package.name));
    }
    if app.recent_packages.is_empty() {
        println!("{}", format_line_content("No recent packages"));
    }
    println!("|{}|", " ".repeat(inner_content_width + 2));
    
    // "Package Statistics" section
    println!("{}", format_line_content("[Package Statistics]"));
    println!("{}", format_line_content(&format!("Total packages: {}", stats.total)));
    
    if stats.pacman > 0 {
        println!("{}", format_line_content(&format!("Pacman packages: {}", stats.pacman)));
    }
    if stats.aur > 0 {
        println!("{}", format_line_content(&format!("AUR packages: {}", stats.aur)));
    }
    if stats.apt > 0 {
        println!("{}", format_line_content(&format!("APT packages: {}", stats.apt)));
    }
    if stats.flatpak > 0 {
        println!("{}", format_line_content(&format!("Flatpak packages: {}", stats.flatpak)));
    }
    println!("|{}|", " ".repeat(inner_content_width + 2));
    
    // "Outdated Packages" section
    println!("{}", format_line_content("[Outdated Packages]"));
    if !app.outdated_packages.is_empty() {
        println!("{}", format_line_content(&format!("Total outdated: {}", stats.outdated)));
        for package in app.outdated_packages.iter().take(5) {
            println!("{}", format_line_content(&package.name));
        }
        if app.outdated_packages.len() > 5 {
            println!("{}", format_line_content("..."));
        }
    } else {
        println!("{}", format_line_content("All packages are up to date!"));
    }
    
    // Footer output
    println!("+{}", "-".repeat(total_box_width - 2));
}

pub fn render_tui_layout<B: Backend>(f: &mut Frame<B>, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints(
            [
                Constraint::Length(3),  // Title
                Constraint::Min(10),    // Main content
                Constraint::Length(3),  // Footer
            ]
            .as_ref(),
        )
        .split(f.size());

    // Title
    let title = Paragraph::new("PackageFetch")
        .style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))
        .alignment(tui::layout::Alignment::Center)
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(title, chunks[0]);

    // Main content
    let main_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
        .split(chunks[1]);

    // Left side - Package Statistics
    let stats = app.get_package_stats();
    let stats_text = vec![
        Spans::from(vec![
            Span::raw("Total packages: "),
            Span::styled(format!("{}", stats.total), Style::default().fg(Color::Green)),
        ]),
        Spans::from(vec![
            Span::raw("Pacman: "),
            Span::styled(format!("{}", stats.pacman), Style::default().fg(Color::Blue)),
        ]),
        Spans::from(vec![
            Span::raw("AUR: "),
            Span::styled(format!("{}", stats.aur), Style::default().fg(Color::Yellow)),
        ]),
        Spans::from(vec![
            Span::raw("Outdated: "),
            Span::styled(format!("{}", stats.outdated), Style::default().fg(Color::Red)),
        ]),
    ];

    let stats_widget = Paragraph::new(stats_text)
        .block(Block::default().title("Statistics").borders(Borders::ALL));
    f.render_widget(stats_widget, main_chunks[0]);

    // Right side - Recent Packages
    let recent_items: Vec<ListItem> = app
        .recent_packages
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
        .collect();

    let recent_widget = List::new(recent_items)
        .block(Block::default().title("Recent Packages").borders(Borders::ALL));
    f.render_widget(recent_widget, main_chunks[1]);

    // Footer
    let footer = Paragraph::new("Press 'q' to quit, 'r' to refresh")
        .style(Style::default().fg(Color::Gray))
        .alignment(tui::layout::Alignment::Center);
    f.render_widget(footer, chunks[2]);
}