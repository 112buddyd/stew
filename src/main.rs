mod app;
mod tui;

use color_eyre::Result;

fn main() -> Result<()> {
    color_eyre::install()?;
    let mut terminal = tui::init()?;
    let app_result = app::App::default().run(&mut terminal);
    if let Err(err) = tui::restore() {
        eprintln!(
            "Failed to restore terminal. Run `reset` or restart your terminal to recover: {}",
            err
        );
    }
    app_result
}
