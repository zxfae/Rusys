use ratatui::
{
    layout::{Constraint, Direction, Layout},
    style::{Color, Style, Modifier},
    widgets::{Block, Borders, Paragraph, Wrap, BorderType},
    text::{Span, Line},
    prelude::Alignment,
    Frame,
};
use crate::syst::infos::get_system_info;
use super::app::App;

pub fn draw(frame: &mut Frame, app: &mut App)
{
    let main_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .margin(1)
        .constraints([
            Constraint::Percentage(60),
                    Constraint::Percentage(40),
        ])
        .split(frame.size());

    let left_chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(0)
        .constraints([
            Constraint::Percentage(20),
                    Constraint::Percentage(25),
                    Constraint::Percentage(25),
                    Constraint::Percentage(30),
        ])
        .split(main_chunks[0]);

    system_info(frame, left_chunks[0]);
    cpu_info(frame, app, left_chunks[1]);
    network_info(frame, app, left_chunks[2]);

    let right_block = Block::default()
        .title("-___╔══ System Monitor ══╗___-")
        .title_alignment(Alignment::Center)
        //Block informations
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(Color::White).add_modifier(Modifier::BOLD))
        .style(Style::default().bg(Color::Reset));

    frame.render_widget(right_block, main_chunks[1]);
}

//Automatic format
fn format_network_rate(rate: f64) -> String
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
//When
fn format_network_total(bytes: u64) -> String
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

//Calling syst info, UI creation
fn system_info(frame: &mut Frame, area: ratatui::layout::Rect)
{
    let sys_info = Block::default()
    .title("-___╔══ System Information ══╗___-")
    .title_alignment(Alignment::Center)
    .borders(Borders::ALL)
    .border_type(BorderType::Double)
    .border_style(Style::default().fg(Color::Green).add_modifier(Modifier::BOLD))
    .style(Style::default().bg(Color::Reset));

    let system_info = get_system_info();
    let text = vec![
        Line::from(vec![
            Span::styled("│ ", Style::default().fg(Color::White)),
                   Span::styled("Hostname: ", Style::default().fg(Color::Gray).add_modifier(Modifier::BOLD)),
                   Span::styled(system_info.host_name, Style::default().fg(Color::LightRed).add_modifier(Modifier::BOLD)),
        ]),
        Line::from(vec![
            Span::styled("│ ", Style::default().fg(Color::White)),
                   Span::styled("OS: ", Style::default().fg(Color::Gray).add_modifier(Modifier::BOLD)),
                   Span::styled(system_info.os_name, Style::default().fg(Color::LightGreen)),
        ]),
        Line::from(vec![
            Span::styled("│ ", Style::default().fg(Color::White)),
                   Span::styled("CPU Architecture: ", Style::default().fg(Color::Gray).add_modifier(Modifier::BOLD)),
                   Span::styled(format!("{:?}", system_info.cpu_architecture), Style::default().fg(Color::Yellow)),
        ]),
        Line::from(vec![
            Span::styled("│ ", Style::default().fg(Color::White)),
                   Span::styled("Kernel Version: ", Style::default().fg(Color::Gray).add_modifier(Modifier::BOLD)),
                   Span::styled(system_info.kernel_version, Style::default().fg(Color::LightMagenta)),
        ]),
        Line::from(vec![
            Span::styled("│ ", Style::default().fg(Color::White)),
                   Span::styled("Total Memory: ", Style::default().fg(Color::Gray).add_modifier(Modifier::BOLD)),
                   Span::styled(
                       format!("{} MB", system_info.total_memory / 1024 / 1024),
                           Style::default().fg(Color::LightRed),
                   ),
        ]),
    ];

    let sys_paragraph = Paragraph::new(text)
    .block(sys_info)
    .alignment(Alignment::Left)
    .wrap(Wrap { trim: true });
    frame.render_widget(sys_paragraph, area);
}

//Calling cpu info, UI creation
fn cpu_info(frame: &mut Frame, app: &mut App, area: ratatui::layout::Rect)
{
    //Formatting block
    let cpu_info = Block::default()
        .title("-___╔══ CPU Information ══╗___-")
        .title_alignment(Alignment::Center)
        .borders(Borders::ALL)
        .border_type(BorderType::Thick)
        .border_style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))
        .style(Style::default().bg(Color::Reset));

    //Create line by lines
    let mut infoby_lines: Vec<Line> = Vec::new();
    //By cpu
    for cpu in app.cpu_monitor.get_cpu_info()
    {
        let (color_indicator, use_style) = match cpu.usage
        {
            //Metric value considere color displaying
            metric if metric > 85.0 => ("⚠️", Style::default().fg(Color::Red).add_modifier(Modifier::BOLD)),
            metric if metric > 50.0 => ("⚡", Style::default().fg(Color::Yellow)),
            _ => ("✓", Style::default().fg(Color::Green)),
        };

        let use_span = Span::styled(
            format!("{:.1}%", cpu.usage),
                use_style
        );

        infoby_lines.push(Line::from(vec![
            Span::raw(format!("│ Core {:2} ", cpu.index)),
                              Span::raw(color_indicator),
                              Span::raw(format!(" {} - {} - ", cpu.vendor_id, cpu.name)),
                              use_span,
                              Span::raw(format!(" - {}\n", cpu.frequency)),
        ]));
    }

    //BlockCPU displaying with parameters
    let cpu_block = Paragraph::new(infoby_lines)
        .block(cpu_info)
        .alignment(Alignment::Left)
        .wrap(Wrap { trim: true });

    frame.render_widget(cpu_block, area);
}

//Calling network info, UI creation
fn network_info(frame: &mut Frame, app: &mut App, area: ratatui::layout::Rect)
{
    let network_info = Block::default()
    .title("-___╔══ Network Information ══╗___-")
    .title_alignment(Alignment::Center)
    .borders(Borders::ALL)
    .border_type(BorderType::Thick)
    .border_style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))
    .style(Style::default().bg(Color::Reset));

    let mut text = Vec::new();

    for network in &app.network_data {
        text.push(Line::from(vec![
            Span::styled("│ ", Style::default().fg(Color::White)),
                             Span::styled(
                                 format!("Interface: {}", network.interface),
                                     Style::default().fg(Color::Gray).add_modifier(Modifier::BOLD)
                             ),
                             Span::styled(" - MAC: ", Style::default().fg(Color::White)),
                             Span::styled(
                                 &network.mac_address,
                                 Style::default().fg(Color::LightBlue).add_modifier(Modifier::BOLD)
                             ),
                             Span::styled(" - IP: ", Style::default().fg(Color::White)),
                             Span::styled(
                                 &network.ip_network,
                                 Style::default().fg(Color::LightBlue).add_modifier(Modifier::BOLD)
                             ),
        ]));

        text.push(Line::from(vec![
            Span::styled("│   ", Style::default().fg(Color::White)),
                             Span::styled("RX: ", Style::default().fg(Color::Green)),
                             Span::styled(
                                 format_network_rate(network.rx_rate),
                                     Style::default().fg(Color::LightGreen)
                             ),
                             Span::styled(" | TX: ", Style::default().fg(Color::Yellow)),
                             Span::styled(
                                 format_network_rate(network.tx_rate),
                                     Style::default().fg(Color::LightYellow)
                             ),
        ]));

        text.push(Line::from(vec![
            Span::styled("│   ", Style::default().fg(Color::White)),
                             Span::styled("Total RX: ", Style::default().fg(Color::Green)),
                             Span::styled(
                                 format_network_total(network.total_received),
                                     Style::default().fg(Color::LightGreen)
                             ),
                             Span::styled(" | Total TX: ", Style::default().fg(Color::Yellow)),
                             Span::styled(
                                 format_network_total(network.total_transmitted),
                                     Style::default().fg(Color::LightYellow)
                             ),
        ]));

        text.push(Line::from(vec![
            Span::styled("│", Style::default().fg(Color::White)),
        ]));
    }

    let network_paragraph = Paragraph::new(text)
    .block(network_info)
    .alignment(Alignment::Left)
    .wrap(Wrap { trim: true });

    frame.render_widget(network_paragraph, area);
}
