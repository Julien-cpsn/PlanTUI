use crate::app::App;
use ratatui::{init, restore};

mod app;
mod ui;
mod event;
mod files;
mod args;
mod render;
mod widgets;
mod logic;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let mut app = App::new()?;

    let terminal = init();
    
    app
        .run(terminal)
        .await?;

    restore();

    Ok(())
}
