use ratatui::
{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style, Modifier},
    widgets::{Block, Borders, Paragraph, Wrap, BorderType, Clear},
    text::{Span, Line},
    prelude::Alignment,
    Frame,
};

use crate::syst::infos::get_system_info;
use super::app::App;
use super::utils::{format_network_rate, format_network_total, info_line};

//STYLE
const TITLE_STYLE: Style = Style::new()
    .fg(Color::Rgb(158,206,106))
    .add_modifier(Modifier::BOLD);

const BORDER_STYLE: Style = Style::new()
    .fg(Color::Rgb(86,95,137))
    .add_modifier(Modifier::BOLD);

const MIN_SYS_INFO_HEIGHT: u16 = 7;
const MIN_CPU_INFO_HEIGHT: u16 = 15;
//const MIN_NET_INFO_HEIGHT: u16 = 15;

pub fn draw(frame: &mut Frame, app: &mut App)
{
    //Window size
    let term_size = frame.size();

    //Split horizontal term by 2 50/50
    let horizontal_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .margin(1)
        .constraints([
            Constraint::Percentage(50),
                    Constraint::Percentage(50),
    ])
    .split(term_size);

    //Create column split; showing 2 blocks
    let left_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(MIN_SYS_INFO_HEIGHT),
                    Constraint::Min(MIN_CPU_INFO_HEIGHT),
    ])
    .split(horizontal_chunks[0]);

    //Impl
    system_info(frame, left_chunks[0]);
    cpu_info(frame, app, left_chunks[1]);
    network_info(frame, app, horizontal_chunks[1]);
}

//Calling syst info, UI creation
fn system_info(frame: &mut Frame, area: Rect)
{
    let sys_info = Block::default()
    .title(Line::from(vec![
        Span::raw("╭─"),
                      Span::styled(" System Information ", TITLE_STYLE),
                      Span::raw("─╮"),
    ]))
    .title_alignment(Alignment::Center)
    .borders(Borders::ALL)
    .border_type(BorderType::Rounded)
    .border_style(BORDER_STYLE);

    let system_info = get_system_info();
    let text = vec![
        info_line("Hostname", system_info.host_name.as_str(), Color::Rgb(247, 118, 142)),
        info_line("OS", system_info.os_name.as_str(), Color::Rgb(158, 206, 106)),
        info_line("CPU Architecture", &format!("{:?}", system_info.cpu_architecture), Color::Rgb(224, 175, 104)),
        info_line("Kernel Version", system_info.kernel_version.as_str(), Color::Rgb(187, 154, 247)),
        info_line("Total Memory", &format!("{} MB", system_info.total_memory / 1024 / 1024), Color::Rgb(239, 111, 111)),
    ];

    let sys_paragraph = Paragraph::new(text)
    .block(sys_info)
    .alignment(Alignment::Left)
    .wrap(Wrap { trim: true });

    frame.render_widget(Clear, area);
    frame.render_widget(sys_paragraph, area);
}

fn cpu_info(frame: &mut Frame, app: &mut App, area: Rect) {
    //Reducing windows depending at cpu size
    let cpu_n = app.cpu_monitor.get_cpu_info().len();
    let normal_size = (cpu_n * 2 + 2) as u16;
    let block_area = Rect::new(
        area.x,
        area.y,
        area.width,
        normal_size.min(area.height)
    );

    let cpu_info = Block::default()
    .title(Line::from(vec![
        Span::raw("╭─"),
                      Span::styled(" CPU Usage ", TITLE_STYLE),
                      Span::raw("─╮"),
    ]))
    .title_alignment(Alignment::Center)
    .borders(Borders::ALL)
    .border_type(BorderType::Rounded)
    .border_style(BORDER_STYLE);

    let mut info_lines: Vec<Line> = Vec::new();
    for cpu in app.cpu_monitor.get_cpu_info() {
        let usage_percentage = cpu.usage;
        let (symbol, color) = match usage_percentage as f64 {
            metric if metric > 85.0 => ("▲", Color::Rgb(247, 118, 142)),
            metric if metric > 50.0 => ("►", Color::Rgb(224, 175, 104)),
            _ => ("●", Color::Rgb(158, 206, 106)),
        };
        let bar_width = 20;
        let filled_width = ((usage_percentage as f64 / 100.0) * bar_width as f64).round() as usize;
        let bar = format!(
            "[{}{}]",
            "█".repeat(filled_width),
                          "░".repeat(bar_width - filled_width)
        );
        info_lines.push(Line::from(vec![
            Span::raw(format!("│ Core {:2} ", cpu.index)),
                                   Span::styled(symbol.to_string(), Style::default().fg(color)),
                                   Span::styled(
                                       format!(" {} ", bar),
                                           Style::default().fg(color)
                                   ),
                                   Span::styled(
                                       format!("{:>5.1}%", usage_percentage),
                                           Style::default().fg(color).add_modifier(Modifier::BOLD)
                                   ),
        ]));
        info_lines.push(Line::from(vec![
            Span::raw("│  "),
                                   Span::styled(
                                       format!("{} - {} MHz", cpu.name, cpu.frequency),
                                           Style::default().fg(Color::Rgb(169, 177, 214))
                                   ),
        ]));
    }
    let cpu_block = Paragraph::new(info_lines)
    .block(cpu_info)
    .alignment(Alignment::Left)
    .wrap(Wrap { trim: true });
    frame.render_widget(Clear, block_area);
    frame.render_widget(cpu_block, block_area);
}

fn network_info(frame: &mut Frame, app: &mut App, area: Rect)
{
    let network_n = app.network_data.len();
    let network_info = Block::default()
    .title(Line::from(vec![
        Span::raw("╭─"),
                      Span::styled(" Network Activity ", TITLE_STYLE),
                      Span::raw("─╮"),
    ]))
    .title_alignment(Alignment::Center)
    .borders(Borders::ALL)
    .border_type(BorderType::Rounded)
    .border_style(BORDER_STYLE);

    let mut text = Vec::new();

    for network in &app.network_data
    {
        text.push(Line::from(vec![
            Span::raw("├"),
                             Span::raw("─".repeat(3)),
                             Span::styled(
                                 format!(" {} ", network.interface),
                                     Style::default().fg(Color::Rgb(187, 154, 247)).add_modifier(Modifier::BOLD)
                             ),
                             Span::raw("─".repeat(20)),
                             Span::raw("┤"),
        ]));
        text.push(Line::from(vec![
            Span::raw("│ "),
                             Span::styled("MAC: ".to_string(), Style::default().fg(Color::Rgb(169, 177, 214))),
                             Span::styled(
                                 network.mac_address.to_string(),
                                          Style::default().fg(Color::Rgb(158, 206, 106))
                             ),
        ]));
        text.push(Line::from(vec![
            Span::raw("│ "),
                             Span::styled("IP: ".to_string(), Style::default().fg(Color::Rgb(169, 177, 214))),
                             Span::styled(
                                 network.ip_network.to_string(),
                                          Style::default().fg(Color::Rgb(158, 206, 106))
                             ),
        ]));
        text.push(Line::from(vec![Span::raw("├─ Transfer Rates ─┤")]));
        text.push(Line::from(vec![
            Span::raw("│ "),
                             Span::styled("↓ RX: ", Style::default().fg(Color::Rgb(158, 206, 106))),
                             Span::styled(
                                 format_network_rate(network.rx_rate),
                                     Style::default().fg(Color::Rgb(187, 154, 247))
                             ),
                             Span::raw("  "),
                             Span::styled("↑ TX: ", Style::default().fg(Color::Rgb(224, 175, 104))),
                             Span::styled(
                                 format_network_rate(network.tx_rate),
                                     Style::default().fg(Color::Rgb(187, 154, 247))
                             ),
        ]));
        text.push(Line::from(vec![Span::raw("├─ Total Transfer ─┤")]));

        text.push(Line::from(vec![
            Span::raw("│ "),
                             Span::styled("↓ Total: ", Style::default().fg(Color::Rgb(158, 206, 106))),
                             Span::styled(
                                 format_network_total(network.total_received),
                                     Style::default().fg(Color::Rgb(187, 154, 247))
                             ),
        ]));
        text.push(Line::from(vec![
            Span::raw("│ "),
                             Span::styled("↑ Total: ", Style::default().fg(Color::Rgb(224, 175, 104))),
                             Span::styled(
                                 format_network_total(network.total_transmitted),
                                     Style::default().fg(Color::Rgb(187, 154, 247))
                             ),
        ]));
        text.push(Line::from(vec![
            Span::raw("├"),
                             Span::raw("─".repeat(30)),
                             Span::raw("┤"),
        ]));
    }

    let network_paragraph = Paragraph::new(text)
    .block(network_info)
    .alignment(Alignment::Left)
    .wrap(Wrap { trim: true });

    frame.render_widget(Clear, area);
    frame.render_widget(network_paragraph, area);
}
