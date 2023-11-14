use crate::Lang;
use crate::{insert_to_hash_map, treesitter_backend::get_query_from_each_language, OpenedFile};
use std::collections::HashMap;
use tree_sitter::{Query, QueryCursor, Tree};

use super::query::TokenData;

pub struct MonacoHighlights;

impl MonacoHighlights {
    pub fn emit(tokens: &TokenData) -> Vec<u32> {
        let mut data = vec![];
        let highlight_tokens = tokens.get_highlights_data();
        let mut prev_line = 0;
        let mut prev_col = 0;

        for token in highlight_tokens {
            let identifier = token.identifier();
            let range = token.range();
            let length = token.length();

            println!(
                "index{}, current_line{} prev_line{}",
                &identifier,
                range.start_row(),
                prev_line
            );

            let delta_start_row: u32 = range.start_row() - prev_line;
            let delta_start_col = if range.start_row() == prev_line {
                range.start_col() - prev_col
            } else {
                range.start_col()
            };

            data.push(delta_start_row); // Delta start row from previous match
            data.push(delta_start_col); // Delta start col from previous match
            data.push(length); // The length of the match
            data.push(identifier); // token index
            data.push(0); // Modifier currently 0 because all is in index
            prev_line = range.start_row();
            prev_col = range.start_col();
        }
        data
    }
}

#[cfg(test)]
mod test {

    use crate::treesitter_backend::query::QueryIter;
    use crate::{treesitter_backend::parser::ParserHelper, FileManager};

    use crate::treesitter_backend::theme::ThemeConfig;

    use super::MonacoHighlights;

    #[test]
    fn see_the_node_children() {
        let mut parser_helper = ParserHelper::default();
        let mut file_manager = FileManager::new();
        let query_iter = QueryIter::default();
        let id = file_manager.load_file("../../test_home/test.js").unwrap();
        let source_code = file_manager.read_source_code_in_bytes(&id.0).unwrap();
        let file_mutex = file_manager._get_file(&id.0);
        let file_language = &file_manager.get_file_language(&id.0).clone();
        parser_helper.append_tree(&id.0, file_mutex.unwrap().clone());
        file_manager.update_source_code(&id.0, &source_code);
        parser_helper.update_tree(&id.0, None);
        parser_helper.parse(&id.0, &file_language.clone().unwrap(), &source_code);
        let token_data = query_iter.iter_query(
            &parser_helper.get_tree(&id.0),
            &file_language.clone().unwrap(),
            &source_code,
        );

        let result = MonacoHighlights::emit(&token_data);

        println!("{:#1?}", result);
    }
    // assert_eq!(
    //     result,
    //     vec![
    //         0, 0, 8, 18, 0, 0, 1, 5, 4, 0, 0, 0, 1, 17, 0, 0, 0, 1, 17, 0, 0, 1, 1, 17, 0, 0,
    //         0, 1, 17, 0, 2, 7, 5, 4, 0, 0, 0, 1, 17, 0, 0, 0, 1, 17, 0, 0, 0, 1, 15, 0
    //     ]
    // )

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
