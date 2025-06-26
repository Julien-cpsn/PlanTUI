use std::fs;
use crate::app::App;
use crate::args::ARGS;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use plantuml_parser::{PlantUmlLine, PlantUmlLineKind};
use ratatui::prelude::{Line, Stylize};

pub const DEFAULT_DIAGRAM: &str = r"@startuml
title MyDiagram
'comment

Alice -> Bob: Hello
@enduml";

impl App {
    pub fn save_pmu_file(&mut self) -> anyhow::Result<()> {
        let diagram = &self.text_input.text;
        let temp_path = self.input_file_path.with_extension("~");

        let mut input_file = File::options()
            .create(true)
            .write(true)
            .read(true)
            .truncate(true)
            .open(&temp_path)
            .expect("Failed to open input file");

        input_file.write_all(diagram.as_bytes())?;
        
        fs::copy(&temp_path, &self.input_file_path)?;
        fs::remove_file(&temp_path)?;

        Ok(())
    }
}

pub fn get_input_file_path(data_dir: &PathBuf) -> anyhow::Result<PathBuf> {
    match &ARGS.input {
        None => {
            let path = data_dir.join("temp");
            fs::remove_file(&path).ok();
            Ok(path)
        },
        Some(path) => Ok(path.clone()),
    }
}

pub fn syntax_highlighting(input: &str) -> Vec<Line> {
    let mut lines = vec![];
    let mut in_comment_block = false;

    for line in input.lines() {
        if let Ok((_rest, (text_line, plantuml_line))) = PlantUmlLine::parse(line.into()) {
            let mut line = match plantuml_line.kind() {
                PlantUmlLineKind::Start(_) => Line::raw(plantuml_line.raw_str().to_string()).blue(),
                PlantUmlLineKind::End(_) => Line::raw(plantuml_line.raw_str().to_string()).blue(),
                PlantUmlLineKind::BlockCommentOpen(_) => {
                    in_comment_block = true;
                    Line::raw(text_line.to_string())
                },
                PlantUmlLineKind::BlockCommentClose(_) => {
                    in_comment_block = false;
                    Line::raw(text_line.to_string()).dim()
                },
                PlantUmlLineKind::InComment(_) => Line::raw(text_line.to_string()).dim(),
                PlantUmlLineKind::Include(_) => Line::raw(plantuml_line.raw_str().to_string()).yellow(),
                PlantUmlLineKind::Title(_) => Line::raw(plantuml_line.raw_str().to_string()).cyan(),
                PlantUmlLineKind::Header(_) => Line::raw(plantuml_line.raw_str().to_string()).magenta(),
                PlantUmlLineKind::Footer(_) => Line::raw(plantuml_line.raw_str().to_string()).magenta(),
                PlantUmlLineKind::Empty => Line::raw(text_line.to_string()).gray(),
                PlantUmlLineKind::Others => Line::raw(text_line.to_string())
            };

            if in_comment_block {
                line = line.dim()
            }
            
            lines.push(line);
        }
    }
    
    return lines;
}