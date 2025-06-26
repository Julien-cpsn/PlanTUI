use crate::app::App;

impl App {
    pub fn shrink_left_area(&mut self) {
        if self.left_area_percentage > 0 {
            self.left_area_percentage -= 10;
        }
    }

    pub fn expand_left_area(&mut self) {
        if self.left_area_percentage < 100 {
            self.left_area_percentage += 10;
        }
    }
}