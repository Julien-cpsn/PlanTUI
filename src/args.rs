use clap::Parser;
use once_cell::sync::Lazy;
use std::path::PathBuf;

pub const ARGS: Lazy<Args> = Lazy::new(|| {
    let args = Args::parse();

    if let Some(path) = &args.input {
        if !path.exists() {
            panic!("Input \"{}\" does not exist.", path.display());
        }

        if !path.is_file() {
            panic!("Input \"{}\" is not a file.", path.display());
        }
    }

    args
});

#[derive(Parser)]
pub struct Args {
    #[arg(short, long)]
    pub input: Option<PathBuf>,
    
    #[arg(short, long, default_value_t = false)]
    pub light_mode: bool,
}