use std::path;
use crate::files::pmu::PlantUmlExtensions;
use clap::Parser;
use once_cell::sync::Lazy;
use std::path::PathBuf;
use crate::files::utils::expand_tilde;

pub const ARGS: Lazy<Args> = Lazy::new(|| {
    let mut args = Args::parse();

    if let Some(input_path) = args.input.as_mut() {
        if !input_path.exists() {
            panic!("Input \"{}\" does not exist.", input_path.display());
        }

        if !input_path.is_file() {
            panic!("Input \"{}\" is not a file.", input_path.display());
        }

        *input_path = expand_tilde(input_path);
        
        if let Ok(new_input_path) = path::absolute(&input_path) {
            *input_path = new_input_path;
        }
    }

    if let Some(command) = args.command.as_mut() {
        match command {
            Command::Render { output, .. } => {
                if !output.exists() {
                    panic!("Render \"{}\" does not exist.", output.display());
                }

                if !output.is_dir() {
                    panic!("Render \"{}\" is not a directory.", output.display());
                }

                *output = expand_tilde(output);

                if let Ok(new_output_path) = path::absolute(&output) {
                    *output = new_output_path;
                }
            }
        }
    }

    args
});

#[derive(Parser)]
pub struct Args {
    #[clap(subcommand)]
    pub command: Option<Command>,

    /// PlantUML file to edit
    pub input: Option<PathBuf>,

    /// Activate light mode instead of dark mode
    #[arg(global = true, short, long, default_value_t = false)]
    pub light_mode: bool,

    /// Activate dark mode instead of light mode
    #[arg(global = true, short, long, overrides_with = "light_mode", default_value_t = false)]
    pub dark_mode: bool,
}

#[derive(clap::Subcommand)]
pub enum Command {
    /// Render the input to the output
    Render {
        /// Output file directory path
        output: PathBuf,

        /// Extension wanted for the output
        #[arg(short, long, value_enum, default_value_t = PlantUmlExtensions::Png)]
        extension: PlantUmlExtensions,
    },
}