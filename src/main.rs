mod monitoring;
mod network;
mod syst;
mod ui;

use ui::ratatui;

fn main() -> std::io::Result<()> {
    ratatui::init()
}
