use anyhow::Result;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyModifiers};
use std::time::Duration;

use super::state::App;

pub fn handle_events(app: &mut App) -> Result<()> {
    if event::poll(Duration::from_millis(250))? {
        if let Event::Key(key) = event::read()? {
            handle_key(app, key);
        }
    }
    Ok(())
}

fn handle_key(app: &mut App, key: KeyEvent) {
    match key.code {
        KeyCode::Char('q') => app.quit(),
        KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => app.quit(),
        KeyCode::Tab => app.next_panel(),
        KeyCode::BackTab => app.prev_panel(),
        KeyCode::Char('1') => app.active_panel = super::state::ActivePanel::Git,
        KeyCode::Char('2') => app.active_panel = super::state::ActivePanel::CiCd,
        KeyCode::Char('3') => app.active_panel = super::state::ActivePanel::Tasks,
        KeyCode::Char('4') => app.active_panel = super::state::ActivePanel::Quality,
        _ => {}
    }
}
