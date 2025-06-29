use crate::args::ARGS;
use crate::files::data_dir::get_data_dir;
use crate::files::pmu::{get_input_file_path, pmu_to_paragraph, DEFAULT_DIAGRAM};
use crate::widgets::text_input::TextInput;
use parking_lot::RwLock;
use ratatui::prelude::Backend;
use ratatui::Terminal;
use ratatui_image::picker::Picker;
use ratatui_image::protocol::StatefulProtocol;
use std::fs;
use std::path::PathBuf;
use std::sync::Arc;
use throbber_widgets_tui::ThrobberState;
use tokio_util::sync::CancellationToken;

pub const APP_NAME: &str = "PlanTUI";

pub struct App<'a> {
    pub should_quit: bool,
    pub data_dir: PathBuf,

    pub dark_mode: bool,
    pub left_area_percentage: u16,

    // Input
    pub input_file_path: PathBuf,
    pub text_input: TextInput<'a>,

    // Output
    pub render_output: Arc<RwLock<RenderOutput>>,
    pub cancellation_token: CancellationToken,
    pub render_throbber_state: ThrobberState,
    pub picker: Option<Picker>
}

pub struct RenderOutput {
    pub pending: bool,
    pub file_path: Option<PathBuf>,
    pub image: Option<StatefulProtocol>,
    pub render_error: Option<String>,
    pub time: String,
}

impl<'a> App<'a> {
    pub fn new() -> anyhow::Result<App<'a>> {
        let data_dir = get_data_dir();
        let input_file_path = get_input_file_path(&data_dir)?;

        let text_input = match fs::exists(&input_file_path)? {
            true => fs::read_to_string(&input_file_path)?,
            false => DEFAULT_DIAGRAM.to_string()
        };

        Ok(App {
            should_quit: false,
            data_dir,
            dark_mode: !ARGS.light_mode,
            left_area_percentage: 50,
            input_file_path,
            text_input: TextInput {
                text: text_input,
                cursor_position: (0, 0),
                render_fn: Box::new(pmu_to_paragraph),
            },
            render_output: Arc::new(RwLock::new(RenderOutput {
                pending: false,
                file_path: None,
                image: None,
                render_error: None,
                time: String::new(),
            })),
            cancellation_token: CancellationToken::new(),
            render_throbber_state: ThrobberState::default(),
            picker: Picker::from_query_stdio().ok(),
        })
    }

    pub async fn run<T: Backend>(&mut self, mut terminal: Terminal<T>) -> anyhow::Result<()> {
        self.save_pmu_file()?;
        self.render_plantuml().await?;
        terminal.draw(|frame| self.ui(frame))?;

        while !self.should_quit {
            self.handle_events().await?;
            terminal.draw(|frame| self.ui(frame))?;
        }

        Ok(())
    }
}