use package_fetch::{App, ui::tui_app};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let app = App::new();
    let mut terminal = tui_app::setup_terminal()?;
    
    let res = tui_app::run_tui(&mut terminal, app);
    
    tui_app::restore_terminal(&mut terminal)?;
    res?;
    
    Ok(())
}