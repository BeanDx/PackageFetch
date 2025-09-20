use tui::{
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Frame,
};

use crate::app::PackageStats;

pub struct PackageTable {
    pub packages: Vec<String>,
}

pub struct PackageStatsWidget {
    pub stats: PackageStats,
}

pub struct PackageGraph {
    pub data: Vec<u64>,
}

impl PackageTable {
    pub fn new(packages: Vec<String>) -> Self {
        Self { packages }
    }

    pub fn render<B: tui::backend::Backend>(&self, f: &mut Frame<B>, area: tui::layout::Rect) {
        let items: Vec<ListItem> = self
            .packages
            .iter()
            .map(|pkg| {
                ListItem::new(Spans::from(vec![
                    Span::raw(pkg),
                ]))
            })
            .collect();

        let list = List::new(items)
            .block(Block::default().title("Packages").borders(Borders::ALL));
        f.render_widget(list, area);
    }
}

impl PackageStatsWidget {
    pub fn new(stats: PackageStats) -> Self {
        Self { stats }
    }

    pub fn render<B: tui::backend::Backend>(&self, f: &mut Frame<B>, area: tui::layout::Rect) {
        let stats_text = vec![
            Spans::from(vec![
                Span::raw("Total packages: "),
                Span::styled(
                    format!("{}", self.stats.total),
                    Style::default().fg(Color::Green).add_modifier(Modifier::BOLD),
                ),
            ]),
            Spans::from(""),
            Spans::from(vec![
                Span::raw("Pacman: "),
                Span::styled(
                    format!("{}", self.stats.pacman),
                    Style::default().fg(Color::Blue),
                ),
            ]),
            Spans::from(vec![
                Span::raw("AUR: "),
                Span::styled(
                    format!("{}", self.stats.aur),
                    Style::default().fg(Color::Yellow),
                ),
            ]),
            Spans::from(vec![
                Span::raw("APT: "),
                Span::styled(
                    format!("{}", self.stats.apt),
                    Style::default().fg(Color::Magenta),
                ),
            ]),
            Spans::from(vec![
                Span::raw("Flatpak: "),
                Span::styled(
                    format!("{}", self.stats.flatpak),
                    Style::default().fg(Color::Cyan),
                ),
            ]),
            Spans::from(""),
            Spans::from(vec![
                Span::raw("Outdated: "),
                Span::styled(
                    format!("{}", self.stats.outdated),
                    Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
                ),
            ]),
        ];

        let stats_widget = Paragraph::new(stats_text)
            .block(Block::default().title("Package Statistics").borders(Borders::ALL));
        f.render_widget(stats_widget, area);
    }
}

impl PackageGraph {
    pub fn new(data: Vec<u64>) -> Self {
        Self { data }
    }

    pub fn render<B: tui::backend::Backend>(&self, f: &mut Frame<B>, area: tui::layout::Rect) {
        let max_value = self.data.iter().max().unwrap_or(&1);
        let height = area.height as usize;
        
        let mut lines = Vec::new();
        for i in 0..height {
            let value = if i < self.data.len() {
                self.data[i]
            } else {
                0
            };
            
            let bar_length = if *max_value > 0 {
                (value as f64 / *max_value as f64 * (area.width as f64 - 2.0)) as usize
            } else {
                0
            };
            
            let bar = "█".repeat(bar_length);
            let empty = "░".repeat((area.width as usize).saturating_sub(bar_length + 2));
            
            lines.push(Spans::from(format!("{}{}", bar, empty)));
        }

        let graph_widget = Paragraph::new(lines)
            .block(Block::default().title("Package Growth").borders(Borders::ALL));
        f.render_widget(graph_widget, area);
    }
}