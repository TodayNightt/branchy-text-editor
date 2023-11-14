use std::collections::HashMap;

use tree_sitter::{Point, Range};
use tree_sitter::{Query, QueryCursor, Tree};

use crate::{insert_to_hash_map, Lang};

use super::get_query_from_each_language;
use super::get_tree_sitter_language;

#[derive(Debug, Clone, Copy)]
pub struct RangePoint(u32, u32, u32, u32);
impl RangePoint {
    pub fn start_row(&self) -> u32 {
        self.0
    }
    pub fn start_col(&self) -> u32 {
        self.1
    }
    pub fn end_row(&self) -> u32 {
        self.2
    }
    pub fn end_col(&self) -> u32 {
        self.3
    }
}
impl From<Range> for RangePoint {
    fn from(value: Range) -> Self {
        let start_point = value.start_point;
        let end_point = value.end_point;
        Self(
            start_point.row as u32,
            start_point.column as u32,
            end_point.row as u32,
            end_point.column as u32,
        )
    }
}

pub struct QueryData {
    query: Query,
    locals_index: Option<u32>,
    injection_index: Option<u32>,
}

#[derive(Debug, Clone, Copy)]
pub struct Token {
    identifier: u32,
    length: u32,
    range: RangePoint,
}

impl Token {
    fn new(index: u32, range: Range) -> Self {
        Self {
            identifier: index,
            length: (range.end_byte - range.start_byte) as u32,
            range: RangePoint::from(range),
        }
    }
    pub fn identifier(&self) -> u32 {
        self.identifier
    }
    pub fn length(&self) -> u32 {
        self.length
    }
    pub fn range(&self) -> RangePoint {
        self.range.clone()
    }
}

#[derive(Debug, Default)]
pub struct TokenData {
    highlights: Vec<Token>,
}

impl TokenData {
    pub fn get_highlights_data(&self) -> Vec<Token> {
        self.highlights.clone()
    }
}
pub struct QueryIter {
    queries: HashMap<Lang, QueryData>,
}

impl Default for QueryIter {
    fn default() -> Self {
        let mut queries: HashMap<Lang, QueryData> = HashMap::new();
        insert_to_hash_map!(query queries, Lang::Javascript);
        insert_to_hash_map!(query queries, Lang::Rust);
        insert_to_hash_map!(query queries, Lang::Typescript);
        insert_to_hash_map!(query queries, Lang::Json);
        Self { queries }
    }
}

impl QueryIter {
    pub fn get_legend(&self, lang: &Lang) -> Vec<String> {
        let query_data = self.queries.get(lang).unwrap();
        let query = &query_data.query;
        query.capture_names().to_vec()
    }
    pub fn iter_query(
        &self,
        tree: &Option<Tree>,
        language: &Lang,
        ranged_source_code: &Vec<u8>,
    ) -> TokenData {
        let mut highlights_data: Vec<Token> = vec![];

        if let Some(tree) = tree.as_ref() {
            let query_data = self.queries.get(&language).unwrap();
            let query = &query_data.query;
            let mut query_cursor = QueryCursor::new();

            let _name = query.capture_names().to_vec();

            let matches =
                query_cursor.captures(query, tree.root_node(), ranged_source_code.as_slice());
            let mut prev_range = Range {
                start_byte: 0,
                end_byte: 0,
                start_point: Point::default(),
                end_point: Point::default(),
            };
            for matches in matches {
                for capture in matches.0.captures {
                    let range = capture.node.range();
                    if range.eq(&prev_range) {
                        continue;
                    }
                    highlights_data.push(Token::new(capture.index, range));
                    prev_range = range;
                }
            }
        }

        TokenData {
            highlights: highlights_data,
        }
    }
}

#[cfg(test)]
mod test {
    use crate::{treesitter_backend::parser::ParserHelper, FileManager};

    use super::QueryIter;

    #[test]
    fn test_query() {
        let mut parser_helper = ParserHelper::default();
        let mut file_manager = FileManager::new();
        let query_analyser = QueryIter::default();
        let id = file_manager.load_file("../../test_home/test.js").unwrap();
        let source_code = file_manager.read_source_code_in_bytes(&id.0).unwrap();
        let file_mutex = file_manager._get_file(&id.0);
        if let Ok(file_mutex) = file_mutex {
            parser_helper.append_tree(&id.0, file_mutex.clone());
            let mut file = file_mutex.lock().unwrap();
            file.update_source_code(&source_code);
            parser_helper.update_tree(&id.0, None);
            parser_helper.parse(&id.0, &file.language.clone().unwrap(), &source_code);
            let result = query_analyser.iter_query(
                &parser_helper.get_tree(&id.0),
                &file.language.clone().unwrap(),
                &source_code,
            );
            println!("{:#2?}", result.get_highlights_data());
        }
    }
}
