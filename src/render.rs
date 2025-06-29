use crate::app::{App, RenderOutput};
use async_process::Command;
use image::ImageReader;
use parking_lot::RwLock;
use plantuml_parser::PlantUmlFileData;
use ratatui_image::picker::Picker;
use std::env;
use std::path::PathBuf;
use std::process::Output;
use std::sync::Arc;
use std::time::Instant;
use anyhow::anyhow;
use once_cell::sync::Lazy;
use tokio_util::sync::CancellationToken;

const PLANTUML_COMMAND: Lazy<String> = Lazy::new(|| {
    let plantuml_env = env::var("PLANT_UML");
    let plantuml_command = match plantuml_env {
        Ok(command) if !command.is_empty() => command,
        _ => String::from("plantuml")
    };
    
    plantuml_command.trim().to_string()
});

impl App<'_> {
    pub async fn render_plantuml(&mut self) -> anyhow::Result<()> {
        if self.render_output.write().pending {
            self.cancellation_token.cancel();
        }

        let render_output_clone = self.render_output.clone();
        let data_dir = self.data_dir.clone();
        let input_file_path = self.input_file_path.clone();
        let picker = self.picker.clone();
        let text_input = self.text_input.text.clone();
        let dark_mode = self.dark_mode;
        let cancellation_token = self.cancellation_token.clone();

        tokio::spawn(async move {
            tokio::select! {
                _ = render_plantuml_task(
                    render_output_clone,
                    data_dir,
                    input_file_path,
                    picker,
                    text_input,
                    dark_mode
                ) => {},
                _ = cancellation_token.cancelled() => {},
            }
        });
        
        self.cancellation_token = CancellationToken::new();
        
        Ok(())
    }
}

async fn render_plantuml_task(
    render_output_clone: Arc<RwLock<RenderOutput>>,
    data_dir: PathBuf,
    input_file_path: PathBuf,
    picker: Option<Picker>,
    text_input: String,
    dark_mode: bool
) {
    render_output_clone.write().pending = true;

    let (output_format, extension) = match picker {
        None => ("-tutxt", "utxt"),
        Some(_) => ("-tpng", "png")
    };

    let mode = dark_mode_to_plantuml_mode(dark_mode);
    
    let initial_time = Instant::now();

    let render_command_output = render_command(output_format, mode, &data_dir, &input_file_path).await;
    
    {
        let mut render_output = render_output_clone.write();

        render_output.time = initial_time.elapsed().as_millis().to_string();

        match render_command_output {
            Ok(output) => {
                if output.stderr.len() == 0 {
                    let output_path = data_dir
                        .join(input_file_path.file_stem().unwrap())
                        .with_extension(extension);

                    render_output.image = None;

                    if let Ok(dyn_img) = ImageReader::open(&output_path).expect("Could not open output file").decode() {
                        if let Some(picker) = picker {
                            render_output.image = Some(picker.new_resize_protocol(dyn_img));
                        }
                    }
                    render_output.file_path = Some(output_path);
                    render_output.render_error = None;
                }
                else {
                    if let Err(parse_error) = PlantUmlFileData::parse_from_str(text_input) {
                        let stringed_error: String = match parse_error {
                            plantuml_parser::Error::Parse(err) => {
                                let err = err.to_string();
                                let comma_split = err.split(',').collect::<Vec<_>>();

                                if comma_split.len() >= 4 {
                                    let begin = comma_split[comma_split.len() - 3];
                                    let end = comma_split[comma_split.len() - 2];
                                    format!("Parse error\n{},{}", &begin[1..], &end[..end.len() - 3])
                                }
                                else {
                                    err
                                }
                            },
                            plantuml_parser::Error::PathResolver(err) => err.to_string(),
                            plantuml_parser::Error::DiagramKindNotMatch(_, _) => "The diagram kind in the start keyword and the diagram kind in the end keyword are not match".to_string(),
                            plantuml_parser::Error::ContentUnclosed(_) | plantuml_parser::Error::IsNotBlockComment => "An end keyword is not found in PlantUmlContent".to_string(),
                            plantuml_parser::Error::Unreachable(err) => err
                        };

                        render_output.render_error = Some(stringed_error);
                    }
                    else {
                        render_output.render_error = Some(String::from_utf8_lossy(&output.stderr).to_string());
                    }

                    render_output.image = None;
                    render_output.file_path = None;
                }
            },
            Err(_) => render_output.file_path = None
        }

        render_output.pending = false;
    }
}

pub async fn render_command(output_format: &str, mode: &str, output_dir_path: &PathBuf, input_file_path: &PathBuf) -> anyhow::Result<Output> {
    match Command::new(&*PLANTUML_COMMAND)
        .args(vec![
            output_format,
            "-nbthread", "auto",
            mode,
            "-failfast2",
            "-output", output_dir_path.as_os_str().to_str().unwrap(),
            input_file_path.as_os_str().to_str().unwrap(),
        ])
        .output()
        .await {
        Ok(output) => Ok(output),
        Err(err) => Err(anyhow!(err))
    }
}

pub fn dark_mode_to_plantuml_mode(dark_mode: bool) -> &'static str {
    match dark_mode {
        true => "-darkmode",
        false => ""
    }
}