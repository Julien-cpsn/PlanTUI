use ratatui::layout::Rect;
use ratatui::prelude::Position;
use ratatui::widgets::Paragraph;

pub struct TextInput<'a> {
    pub text: String,
    // y/height, x/width
    pub cursor_position: (u16, u16),
    pub render_fn: Box<dyn Fn(&str) -> Paragraph<'a> + 'a>
}

impl TextInput<'_> {
    pub fn move_cursor_up(&mut self) {
        self.cursor_position.0 = self.cursor_position.0.saturating_sub(1);
        self.cursor_position.1 = self.clamp_cursor_width(self.cursor_position.0, self.cursor_position.1);
    }

    pub fn move_cursor_down(&mut self) {
        let cursor_moved_down = self.cursor_position.0.saturating_add(1);

        if cursor_moved_down < self.text.lines().count() as u16 {
            self.cursor_position.0 = cursor_moved_down;
        }

        self.cursor_position.1 = self.clamp_cursor_width(self.cursor_position.0, self.cursor_position.1);
    }

    pub fn move_cursor_left(&mut self) {
        let cursor_moved_left = self.cursor_position.1.saturating_sub(1);
        self.cursor_position.1 = self.clamp_cursor_width(self.cursor_position.0, cursor_moved_left);
    }

    pub fn move_cursor_right(&mut self) {
        let cursor_moved_right = self.cursor_position.1.saturating_add(1);
        self.cursor_position.1 = self.clamp_cursor_width(self.cursor_position.0, cursor_moved_right);
    }

    pub fn move_cursor_line_start(&mut self) {
        self.cursor_position.1 = 0;
    }

    pub fn move_cursor_line_end(&mut self) {
        let current_line = self.text.lines().nth(self.cursor_position.0 as usize).unwrap();
        self.cursor_position.0 = current_line.chars().count() as u16;
    }

    pub fn enter_char(&mut self, new_char: char) {
        let byte_index = self.get_absolute_byte_index(self.cursor_position.0, self.cursor_position.1);
        self.text.insert(byte_index, new_char);
        self.move_cursor_right();
    }

    #[allow(unused)]
    pub fn enter_str(&mut self, string: &str) {
        for char in string.chars() {
            self.enter_char(char)
        }
    }

    pub fn delete_char_backward(&mut self) {
        // Can't delete if we're at the very beginning
        if self.cursor_position.0 == 0 && self.cursor_position.1 == 0 {
            return;
        }

        // If we're at the beginning of a line (but not the first line)
        if self.cursor_position.1 == 0 {
            // Get the length of the previous line before joining
            let prev_line_len = self.text.lines().nth(self.cursor_position.0 as usize - 1)
                .map(|line| line.chars().count())
                .unwrap_or(0);

            // Find the newline character before current line and remove it
            let current_line_start = self.get_absolute_byte_index(self.cursor_position.0, 0);
            if current_line_start > 0 {
                self.text.remove(current_line_start - 1);
            }

            // Move cursor to where the lines joined
            self.cursor_position.0 -= 1;
            self.cursor_position.1 = prev_line_len as u16;
        } else {
            // We're in the middle of a line, delete the previous character
            let byte_index = self.get_absolute_byte_index(self.cursor_position.0, self.cursor_position.1 - 1);
            self.text.remove(byte_index);
            self.move_cursor_left();
        }
    }

    pub fn delete_char_forward(&mut self) {
        let current_line = self.text.lines().nth(self.cursor_position.0 as usize).unwrap_or("");

        // If we're at the end of a line
        if self.cursor_position.1 as usize >= current_line.chars().count() {
            // If this is the last line, nothing to delete
            if self.cursor_position.0 as usize >= self.text.lines().count() - 1 {
                return;
            }

            // Join the next line with this line by removing the newline
            let byte_index = self.get_absolute_byte_index(self.cursor_position.0 + 1, 0);
            if byte_index > 0 {
                self.text.remove(byte_index - 1); // Remove the newline character
            }
        } else {
            // We're in the middle of a line, delete the character at cursor
            let byte_index = self.get_absolute_byte_index(self.cursor_position.0, self.cursor_position.1);
            if byte_index < self.text.len() {
                self.text.remove(byte_index);
            }
        }
    }

    // Helper method to get absolute byte index for any line/column position
    fn get_absolute_byte_index(&self, line: u16, col: u16) -> usize {
        let mut byte_index = 0;

        // Add bytes for all previous lines (including their newlines)
        for (i, text_line) in self.text.lines().enumerate() {
            if i >= line as usize {
                break;
            }
            byte_index += text_line.len() + 1; // +1 for newline
        }

        // Add bytes for characters in the current line up to the column
        if let Some(current_line) = self.text.lines().nth(line as usize) {
            byte_index += current_line.char_indices()
                .nth(col as usize)
                .map_or(current_line.len(), |(idx, _)| idx);
        }

        // Ensure we don't exceed text length
        byte_index.min(self.text.len())
    }

    pub fn clamp_cursor_width(&self, y: u16, x: u16) -> u16 {
        let max_line_size = self.text
            .lines()
            .nth(y as usize)
            .unwrap()
            .chars()
            .count();

        x.clamp(0, max_line_size as u16)
    }

    pub fn calculate_scroll_offset(&self, viewport_height: u16, viewport_width: u16) -> (u16, u16) {
        let cursor_y = self.cursor_position.0;
        let cursor_x = self.cursor_position.1;
        let total_lines = self.text.lines().count() as u16;

        // Calculate vertical offset to center cursor
        let half_height = viewport_height / 2;
        let vertical_offset = if cursor_y >= half_height {
            let desired_offset = cursor_y - half_height;
            // Don't scroll past the end of the text
            let max_offset = total_lines.saturating_sub(viewport_height);
            desired_offset.min(max_offset)
        }
        else {
            0
        };

        // Calculate horizontal offset to center cursor
        let half_width = viewport_width / 2;
        let horizontal_offset = if cursor_x > half_width + 1 {
            cursor_x - half_width
        }
        else {
            0
        };

        (vertical_offset, horizontal_offset)
    }

    pub fn get_cursor_screen_position(&self, area: Rect, vertical_offset: u16, horizontal_offset: u16) -> Option<Position> {
        let cursor_y = self.cursor_position.0;
        let cursor_x = self.cursor_position.1;

        // Calculate the cursor position relative to the scrolled viewport
        let screen_y = cursor_y.saturating_sub(vertical_offset);
        let screen_x = cursor_x.saturating_sub(horizontal_offset);

        // Check if cursor is visible within the viewport
        if screen_y < area.height && screen_x < area.width {
            // Calculate absolute screen position
            let absolute_x = area.x + screen_x;
            let absolute_y = area.y + screen_y;

            // Ensure cursor is within text area bounds
            if absolute_x < area.x + area.width && absolute_y < area.y + area.height {
                Some(Position::new(absolute_x, absolute_y))
            }
            else {
                None
            }
        }
        else {
            None
        }
    }

    #[allow(unused)]
    pub fn reset_cursor(&mut self) {
        self.cursor_position = (0, 0);
    }


    #[allow(unused)]
    pub fn reset_input(&mut self) {
        self.text.clear();
        self.reset_cursor();
    }
}