use crate::app::{App, APP_NAME};
use ratatui::prelude::{Constraint, Layout, Line, Rect, Span, Stylize};
use ratatui::widgets::{Block, Borders, Paragraph, Wrap};
use ratatui::Frame;
use ratatui_image::StatefulImage;
use std::fs;
use throbber_widgets_tui::{Throbber, WhichUse, BRAILLE_DOUBLE};

impl App<'_> {
    pub fn ui(&mut self, frame: &mut Frame) {
        let [title_area, main_area] = Layout::vertical(vec![
            Constraint::Length(1),
            Constraint::Fill(1)
        ])
            .areas(frame.area());

        let file_name = self.input_file_path.file_name().unwrap().to_str().unwrap();
        let title = Line::from(vec![
            Span::raw(APP_NAME).italic().gray(),
            Span::raw(" {").dark_gray(),
            Span::raw(file_name),
            Span::raw("}").dark_gray(),
        ])
            .centered();

        let main_block = Block::bordered();
        let inner_main_area = main_block.inner(main_area);

        frame.render_widget(title, title_area);
        frame.render_widget(main_block, main_area);

        self.main_area(frame, inner_main_area);
    }

    pub fn main_area(&mut self, frame: &mut Frame, area: Rect) {
        let [text_area, render_area] = Layout::horizontal(vec![
            Constraint::Percentage(self.left_area_percentage),
            Constraint::Percentage(100-self.left_area_percentage),
        ])
            .areas(area);
        
        let mut text_input_par = (self.text_input.render_fn)(&self.text_input.text);
        let (vertical_offset, horizontal_offset) = self.text_input.calculate_scroll_offset(text_area.height, text_area.width);
        let cursor_position = self.text_input.get_cursor_screen_position(text_area, vertical_offset, horizontal_offset);
        
        text_input_par = text_input_par.scroll((
            vertical_offset,
            horizontal_offset,
        ));
        
        if let Some(cursor_position) = cursor_position {
            frame.set_cursor_position(cursor_position);
        }
        
        frame.render_widget(text_input_par, text_area);

        {
            let output_clone = self.render_output.clone();
            let mut output = output_clone.write();

            let render_area_block = Block::new()
                .borders(Borders::LEFT)
                .title_bottom(
                    Line::from(format!("{} ms", output.time))
                        .right_aligned()
                        .dim()
                );

            let inner_render_area = render_area_block.inner(render_area);
            frame.render_widget(render_area_block, render_area);

            if output.pending {
                self.render_throbber_state.calc_next();

                let throbber = Throbber::default()
                    .throbber_set(BRAILLE_DOUBLE)
                    .use_type(WhichUse::Spin)
                    .label("Rendering")
                    .to_line(&mut self.render_throbber_state)
                    .centered();

                let throbber_par = Paragraph::new(vec![
                    Line::default(),
                    throbber
                ]);

                frame.render_widget(throbber_par, inner_render_area);
            }
            else {
                match &output.file_path {
                    Some(path) => match &self.picker {
                        None => {
                            let content = fs::read_to_string(&path).unwrap();
                            let render_par = Paragraph::new(content);
                            frame.render_widget(render_par, inner_render_area);
                        },
                        Some(_) => {
                            if let Some(image) = output.image.as_mut() {
                                frame.render_stateful_widget(StatefulImage::default(), inner_render_area, image);
                            }
                        }
                    },
                    None => match &output.render_error {
                        None => {}
                        Some(render_error) => {
                            let render_error_par = Paragraph::new(render_error.clone())
                                .wrap(Wrap { trim: false })
                                .red();

                            frame.render_widget(render_error_par, inner_render_area);
                        }
                    }
                }
            }
        }
    }
}