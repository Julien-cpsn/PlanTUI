#[derive(Default)]
pub struct TextInput {
    pub text: String,
    // y/height, x/width
    pub cursor_position: (usize, usize)
}

#[allow(unused)]
const ELLIPSIS_LEFT: &str = "<";
#[allow(unused)]
const ELLIPSIS_RIGHT: &str = ">";

impl TextInput {
    pub fn move_cursor_up(&mut self) {
        self.cursor_position.0 = self.cursor_position.0.saturating_sub(1);
        self.cursor_position.1 = self.clamp_cursor_width(self.cursor_position.0, self.cursor_position.1);
    }

    pub fn move_cursor_down(&mut self) {
        let cursor_moved_down = self.cursor_position.0.saturating_add(1);

        if cursor_moved_down < self.text.lines().count() {
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
        let current_line = self.text.lines().nth(self.cursor_position.0).unwrap();
        self.cursor_position.0 = current_line.chars().count();
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
            let prev_line_len = self.text.lines().nth(self.cursor_position.0 - 1)
                .map(|line| line.chars().count())
                .unwrap_or(0);

            // Find the newline character before current line and remove it
            let current_line_start = self.get_absolute_byte_index(self.cursor_position.0, 0);
            if current_line_start > 0 {
                self.text.remove(current_line_start - 1);
            }

            // Move cursor to where the lines joined
            self.cursor_position.0 -= 1;
            self.cursor_position.1 = prev_line_len;
        } else {
            // We're in the middle of a line, delete the previous character
            let byte_index = self.get_absolute_byte_index(self.cursor_position.0, self.cursor_position.1 - 1);
            self.text.remove(byte_index);
            self.move_cursor_left();
        }
    }

    pub fn delete_char_forward(&mut self) {
        let current_line = self.text.lines().nth(self.cursor_position.0).unwrap_or("");

        // If we're at the end of a line
        if self.cursor_position.1 >= current_line.chars().count() {
            // If this is the last line, nothing to delete
            if self.cursor_position.0 >= self.text.lines().count() - 1 {
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
    fn get_absolute_byte_index(&self, line: usize, col: usize) -> usize {
        let mut byte_index = 0;

        // Add bytes for all previous lines (including their newlines)
        for (i, text_line) in self.text.lines().enumerate() {
            if i >= line {
                break;
            }
            byte_index += text_line.len() + 1; // +1 for newline
        }

        // Add bytes for characters in the current line up to the column
        if let Some(current_line) = self.text.lines().nth(line) {
            byte_index += current_line.char_indices()
                .nth(col)
                .map_or(current_line.len(), |(idx, _)| idx);
        }

        // Ensure we don't exceed text length
        byte_index.min(self.text.len())
    }

    pub fn clamp_cursor_width(&self, y: usize, x: usize) -> usize {
        let max_line_size = self.text
            .lines()
            .nth(y)
            .unwrap()
            .chars()
            .count();

        x.clamp(0, max_line_size)
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

    /*
    pub fn get_padded_text_and_cursor(&self, length: usize) -> (String, usize) {
        let text_char_count = self.text.chars().count();

        // if the text is shorter than the desired length
        if text_char_count <= length {
            return (self.text.clone(), self.cursor_position);
        }

        let ellipsis_char_count = ELLIPSIS_LEFT.chars().count();

        // Calculate the visible portion length
        let first_part_length = length - ellipsis_char_count;

        // If cursor is within the first visible portion
        if self.cursor_position <= first_part_length {
            // Take the first part of the text (in characters)
            let first_part: String = self.text.chars().take(first_part_length).collect();
            let text = format!("{}{}", first_part, ELLIPSIS_RIGHT);
            return (text, self.cursor_position);
        }

        // For cursor positions beyond the first visible portion
        let double_adjusted_length = length - 2 * ellipsis_char_count;
        let char_vec: Vec<char> = self.text.chars().collect();

        // Calculate the number of "pages" of text
        let nb_lengths_text = ((text_char_count - first_part_length) / double_adjusted_length) + 1;
        let nb_lengths_cursor = ((self.cursor_position - first_part_length) / double_adjusted_length) + 1;

        // Calculate the starting character index for the visible portion
        let start_index = first_part_length + (nb_lengths_cursor - 1) * double_adjusted_length;

        if nb_lengths_cursor == nb_lengths_text {
            // If cursor is in the last "page"
            let text = format!("{}{}", ELLIPSIS_LEFT, char_vec[start_index..].iter().collect::<String>());
            return (text, self.cursor_position + ellipsis_char_count - start_index);
        } else {
            // If cursor is in a middle "page"
            let end_index = std::cmp::min(start_index + double_adjusted_length, text_char_count);
            let visible_text: String = char_vec[start_index..end_index].iter().collect();
            let text = format!("{}{}{}", ELLIPSIS_LEFT, visible_text, ELLIPSIS_RIGHT);
            return (text, self.cursor_position + ellipsis_char_count - start_index);
        }
    }*/
}