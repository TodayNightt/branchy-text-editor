use crate::{
    treesitter_backend::get_query_from_each_language,
    OpenedFile,
};
use serde::{Deserialize, Serialize};
use specta::Type;
use std::error::Error;
use std::fmt::Display;
use std::fs::File;
use std::io::BufReader;
use std::sync::Mutex;
use tree_sitter::{Point, Query, QueryCursor, Range};

use super::get_tree_sitter_language;

impl OpenedFile {
    pub fn highlight(&mut self, ranged_source_code: Vec<u8>) -> Result<Vec<u32>, Box<dyn Error>> {
        let tree = self.tree.lock().unwrap();

        let mut data: Vec<u32> = vec![];

        let language = &self.language.clone().unwrap();

        if let Some(tree) = tree.as_ref() {
            if self.query.as_ref().is_none() {
                self.query = Some(Query::new(
                    get_tree_sitter_language(&language),
                    get_query_from_each_language(&language),
                )?)
                .into();
            }
            let query = self.query.as_ref().as_ref().unwrap();
            let mut query_cursor = QueryCursor::new();

            let mut prev_line = 0;
            let mut prev_col = 0;
            let matching =
                query_cursor.captures(&query, tree.root_node(), ranged_source_code.as_slice());

            let mut prev_range = Range {
                start_byte: 0,
                end_byte: 0,
                start_point: Point::default(),
                end_point: Point::default(),
            };
            let _name = query.capture_names().to_vec();
            for matches in matching {
                for capture in matches.0.captures {
                    let range = capture.node.range();
                    let start = range.start_point;
                    let end = range.end_point;

                    if range.eq(&prev_range) {
                        continue;
                    }

                    // println!(
                    //     "index{},{:?}, {:?}, current_line{} prev_line{}",
                    //     capture.index,
                    //     capture.node,
                    //     _name.get(capture.index as usize),
                    //     start.row,
                    //     prev_line
                    // );

                    let delta_start_row = (start.row - prev_line) as u32;
                    let delta_start_col = if start.row == prev_line {
                        start.column - prev_col
                    } else {
                        start.column
                    } as u32;
                    let length = (end.column - start.column) as u32;

                    data.push(delta_start_row); // Delta start row from previous match
                    data.push(delta_start_col); // Delta start col from previous match
                    data.push(length); // The length of the match
                    data.push(capture.index); // token index
                    data.push(0); // Modifier currently 0 because all is in index

                    prev_line = start.row;
                    prev_col = start.column;
                    prev_range = range;
                }
            }
        }

        Ok(data)
    }
}

#[derive(Deserialize, Serialize, Debug, Clone, Default, Type)]

pub struct Token {
    token: String,
    foreground: String,
}

impl Token {
    fn new(token: &str, foreground: &str) -> Self {
        Self {
            token: token.to_string(),
            foreground: foreground.to_string(),
        }
    }
}

#[derive(Deserialize, Serialize, Debug, Clone, Type)]
pub struct Theme {
    rules: Vec<Token>, // keyword: String,
                       // function: String,
                       // types: String,
                       // variable: String,
                       // number: String,
                       // string: String,
}

impl Default for Theme {
    fn default() -> Self {
        let mut rules = vec![];
        rules.push(Token::new("keyword", "#DD91FA"));
        rules.push(Token::new("function", "#71B1FF"));
        rules.push(Token::new("type", "#DCAE80"));
        rules.push(Token::new("variable", "#9CDCFE"));
        rules.push(Token::new("number", "#b4d5ff"));
        rules.push(Token::new("string", "#72c0ff"));
        rules.push(Token::new("comment", "#93d0ff"));
        rules.push(Token::new("class", "#D1B9FF"));
        Self { rules }
    }
}

#[derive(Deserialize, Serialize, Debug, Default, Clone, Type)]
pub struct EditorTheme {
    background: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Type)]
pub struct LanguageTheme {
    default: Theme,
    javascript: Option<Theme>,
    rust: Option<Theme>,
    java: Option<Theme>,
    html: Option<Theme>,
    css: Option<Theme>,
    python: Option<Theme>,
    ruby: Option<Theme>,
}

impl Default for LanguageTheme {
    fn default() -> Self {
        Self {
            default: Theme::default(),
            javascript: None,
            rust: None,
            java: None,
            html: None,
            css: None,
            python: None,
            ruby: None,
        }
    }
}

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct ThemeConfig {
    pub language: Mutex<LanguageTheme>,
    pub editor: Mutex<EditorTheme>,
}

impl ThemeConfig {
    fn load(path: &str) -> Result<Self, Box<dyn Error>> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);

        // Read the JSON contents of the file as an instance of `User`.
        Ok(serde_json::from_reader(reader)?)
    }

    fn save(&self) {
        let json = serde_json::to_string(self).expect("Cannot serialize");
        std::fs::write("../../test_home/config.json", json).expect("Cannot save config file");
    }
}

impl Display for ThemeConfig {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}", serde_json::to_string_pretty(&self).unwrap())
    }
}

#[cfg(test)]
mod test {
    use crate::FileManager;

    use super::ThemeConfig;

    #[test]
    fn see_the_node_children() {
        let mut file_manager = FileManager::new();
        let id = file_manager.load_file("../../test_home/test.js").unwrap();
        let source_code = file_manager.read_source_code_in_bytes(&id.0).unwrap();
        let binding = file_manager._get_file(&id.0).unwrap();
        let mut file = binding.lock().unwrap();
        file.update_source_code(&source_code);
        file.update_tree(None);
        file.parse();
        let result = file.highlight(source_code).unwrap();

        assert_eq!(
            result,
            vec![
                0, 0, 8, 18, 0, 0, 1, 5, 4, 0, 0, 0, 1, 17, 0, 0, 0, 1, 17, 0, 0, 1, 1, 17, 0, 0,
                0, 1, 17, 0, 2, 7, 5, 4, 0, 0, 0, 1, 17, 0, 0, 0, 1, 17, 0, 0, 0, 1, 15, 0
            ]
        )
    }

    #[test]
    fn check_loading_config_file() {
        let config = ThemeConfig::load("../../test_home/config.json");

        dbg!(&config);
        if let Ok(config) = config {
            println!("{}", config);
        }
    }

    #[test]
    fn save_config_file() {
        let config = ThemeConfig::default();

        config.save();
    }
}
