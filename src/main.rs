use crate::app::App;
use ratatui::{init, restore};
use crate::args::ARGS;

mod app;
mod ui;
mod event;
mod files;
mod args;
mod render;
mod widgets;
mod logic;
mod cli;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let mut app = App::new()?;

    
    if let Some(command) = &ARGS.command {
        app.handle_command(command).await?;
    }
    else {
        let terminal = init();

        app
            .run(terminal)
            .await?;

        restore();
    }
    
    Ok(())
}
