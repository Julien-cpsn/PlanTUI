use crate::app::App;
use arboard::{Clipboard, ImageData};
use image::EncodableLayout;
use std::fs;

impl App {
    pub fn copy_to_clipboard(&self) -> anyhow::Result<()> {
        let mut clipboard = Clipboard::new()?;

        let render_output = self.render_output.read();

        if let Some(file_path) = &render_output.file_path {
            let content = fs::read(file_path)?;
            
            match image::load_from_memory(&content) {
                Ok(dyn_image) => {
                    let rgba_image = dyn_image.to_rgba8();

                    clipboard
                        .set_image(ImageData {
                            width: rgba_image.width() as usize,
                            height: rgba_image.height() as usize,
                            bytes: rgba_image.as_bytes().into()
                        })
                        .expect("Could not copy response image to clipboard");
                }
                Err(_) => clipboard.set_text(String::from_utf8_lossy(&content))?
            };
        }
        
        Ok(())
    }
}