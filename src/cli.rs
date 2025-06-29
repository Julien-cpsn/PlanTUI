use crate::app::App;
use crate::args::{Command, ARGS};
use crate::render::{dark_mode_to_plantuml_mode, render_command};

impl App<'_> {
    pub async fn handle_command(&mut self, command: &Command) -> anyhow::Result<()> {
        match command {
            Command::Render { output, extension } => {
                render_command(
                    &extension.to_output_format(),
                    dark_mode_to_plantuml_mode(!ARGS.light_mode && ARGS.dark_mode),
                    output,
                    &self.input_file_path
                )
                    .await?;
            }
        }

        Ok(())
    }
}