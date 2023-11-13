use crate::{treesitter_backend::get_query_from_each_language, OpenedFile};
use serde::{Deserialize, Serialize};
use specta::Type;
use std::error::Error;
use std::fmt::Display;
use std::fs::File;
use std::io::BufReader;
use std::sync::Mutex;
use tree_sitter::{Point, Query, QueryCursor, Range, Tree};

use super::get_tree_sitter_language;

impl OpenedFile {
    pub fn highlight(
        &mut self,
        tree: &Option<Tree>,
        ranged_source_code: Vec<u8>,
    ) -> Result<Vec<u32>, Box<dyn Error>> {
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

#[cfg(test)]
mod test {

    use crate::{treesitter_backend::parser::ParserHelper, FileManager};

    use super::ThemeConfig;

    #[test]
    fn see_the_node_children() {
        let mut parser_helper = ParserHelper::default();
        let mut file_manager = FileManager::new();
        let id = file_manager.load_file("../../test_home/test.js").unwrap();
        let source_code = file_manager.read_source_code_in_bytes(&id.0).unwrap();
        let file_mutex = file_manager._get_file(&id.0);
        if let Ok(file_mutex) = file_mutex {
            parser_helper.append_tree(&id.0, file_mutex.clone());
            let mut file = file_mutex.lock().unwrap();
            file.update_source_code(&source_code);
            parser_helper.update_tree(&id.0, None);
            parser_helper.parse(&id.0, &file.language.clone().unwrap(), &source_code);
            let result = file
                .highlight(&parser_helper.get_tree(&id.0), source_code)
                .unwrap();
        }
        // assert_eq!(
        //     result,
        //     vec![
        //         0, 0, 8, 18, 0, 0, 1, 5, 4, 0, 0, 0, 1, 17, 0, 0, 0, 1, 17, 0, 0, 1, 1, 17, 0, 0,
        //         0, 1, 17, 0, 2, 7, 5, 4, 0, 0, 0, 1, 17, 0, 0, 0, 1, 17, 0, 0, 0, 1, 15, 0
        //     ]
        // )
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
