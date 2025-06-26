use crate::app::App;
use crokey::crossterm::event;
use crokey::crossterm::event::{Event, KeyCode, KeyEvent};
use crokey::OneToThree::One;
use crokey::{key, KeyCombination};
use std::time::Duration;

const TICK_RATE: Duration = Duration::from_millis(200);

impl App {
    pub async fn handle_events(&mut self) -> anyhow::Result<()> {
        if event::poll(TICK_RATE)? {
            if let Ok(event) = event::read() {
                if let Event::Key(key) = event {
                    let missed_input = self.handle_event(key)?;

                    if !missed_input && !self.should_quit {
                        self.save_pmu_file()?;
                        self.render_plantuml().await?;
                    }
                }
            }
        }

        Ok(())
    }

    fn handle_event(&mut self, key_event: KeyEvent) -> anyhow::Result<bool> {
        let key_combination = KeyCombination::from(key_event);
        let mut missed_input = false;
        let mut should_render = false;

        match key_combination {
            key!(ctrl-c) => self.should_quit = true,
            key!(ctrl-Y) | key!(ctrl-shift-Y) => self.copy_to_clipboard()?,

            key!(alt-left) => self.shrink_left_area(),
            key!(alt-right) => self.expand_left_area(),
            key!(ctrl-shift-D) => self.dark_mode = !self.dark_mode,

            key!(delete) => {
                self.text_input.delete_char_forward();
                should_render = true;
            },
            key!(backspace) => {
                self.text_input.delete_char_backward();
                should_render = true;
            },
            key!(enter) => {
                self.text_input.enter_char('\n');
                self.text_input.move_cursor_down();
                should_render = true;
            },
            key!(home) => self.text_input.move_cursor_line_start(),
            key!(end) => self.text_input.move_cursor_line_end(),

            key!(up) => self.text_input.move_cursor_up(),
            key!(down) => self.text_input.move_cursor_down(),
            key!(left) => self.text_input.move_cursor_left(),
            key!(right) => self.text_input.move_cursor_right(),

            KeyCombination { codes: One(KeyCode::Char(char)), .. } => {
                self.text_input.enter_char(char);
                should_render = true;
            },

            _ => missed_input = true,
        }

        Ok(missed_input || !should_render)
    }
}