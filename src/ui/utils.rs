use ratatui::
{
    style::{Color, Style, Modifier},
    text::{Span, Line},
};

//metrics converter
pub fn format_network_rate(rate: f64) -> String
{
    let rate_bytes = rate;
    if rate_bytes >= 1_000_000_000.0 {
        format!("{:.2} GB/s", rate_bytes / 1_000_000_000.0)
    } else if rate_bytes >= 1_000_000.0 {
        format!("{:.2} MB/s", rate_bytes / 1_000_000.0)
    } else if rate_bytes >= 1_000.0 {
        format!("{:.2} KB/s", rate_bytes / 1_000.0)
    } else {
        format!("{:.0} B/s", rate_bytes)
    }
}

pub fn format_network_total(bytes: u64) -> String
{
    let bytes = bytes as f64;
    if bytes >= 1_000_000_000_000.0 {
        format!("{:.2} TB", bytes / 1_000_000_000_000.0)
    } else if bytes >= 1_000_000_000.0 {
        format!("{:.2} GB", bytes / 1_000_000_000.0)
    } else if bytes >= 1_000_000.0 {
        format!("{:.2} MB", bytes / 1_000_000.0)
    } else if bytes >= 1_000.0 {
        format!("{:.2} KB", bytes / 1_000.0)
    } else {
        format!("{:.0} B", bytes)
    }
}

pub fn info_line(label: &str, value: &str, color: Color) -> Line<'static> {
    Line::from(vec![
        Span::raw("│ "),
               Span::styled(
                   format!("{}: ", label),
                       Style::default()
                       .fg(Color::Rgb(169, 177, 214))
                       .add_modifier(Modifier::BOLD)
               ),
               Span::styled(
                   value.to_string(),
                            Style::default().fg(color).add_modifier(Modifier::BOLD)
               ),
    ])
}
