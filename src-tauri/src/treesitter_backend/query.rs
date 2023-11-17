use std::collections::{HashMap, HashSet};
use std::sync::Arc;

use serde::{Deserialize, Serialize};
use specta::Type;
use tree_sitter::{Query, QueryCursor, Range, Tree};

use crate::{insert_to_hash_map, Lang};

use super::get_query_from_each_language;
use super::get_tree_sitter_language;
use super::highlighter::HighlightIter;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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

    pub fn within(&self, value: RangePoint) -> bool {
        value.start_row().gt(&self.start_row()) && value.end_row().lt(&self.end_row())
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
#[derive(Debug, Serialize, Deserialize, Type, Clone, Default)]
pub struct SemanticLegend {
    _token_types: Vec<String>,
    _token_modifier: Vec<String>,
}

impl SemanticLegend {
    pub fn create(name: Vec<String>) -> Self {
        Self {
            _token_types: name.to_owned(),
            _token_modifier: vec![],
        }
    }

    pub fn get_token_types(&self) -> Vec<String> {
        self._token_types.to_owned()
    }

    pub fn get_modifier(&self) -> Vec<String> {
        self._token_modifier.to_owned()
    }
}
#[derive(Debug)]
pub struct QueryData {
    query: Query,
    legend: Arc<SemanticLegend>,
    modified_legend: Arc<SemanticLegend>,
    _local_scope: Option<u32>,
    _local_reference: Option<u32>,
    _local_definition: Option<u32>,
    _injection_content: Option<u32>,
    _injection_language: Option<u32>,
}

#[derive(Debug, Clone, Copy)]
pub struct Token {
    token_type: u32,
    modifier: u32,
    length: u32,
    range: RangePoint,
}

impl Token {
    fn new(index: u32, range: Range) -> Self {
        Self {
            token_type: index,
            modifier: 0,
            length: (range.end_byte - range.start_byte) as u32,
            range: RangePoint::from(range),
        }
    }
    pub fn token_type(&self) -> u32 {
        self.token_type
    }

    pub fn modifier(&self) -> u32 {
        self.modifier
    }
    pub fn length(&self) -> u32 {
        self.length
    }
    pub fn range(&self) -> RangePoint {
        self.range.clone()
    }

    pub fn remap_token_n_modifier(&mut self, index: (u32, u32)) {
        self.token_type = index.0;
        println!("{}", index.1);
        self.modifier = index.1;
    }
}

macro_rules! filter_out_to_vec {
    ($tokens :expr,$index_option : expr) => {
        if let Some(index) = $index_option {
            $tokens
                .iter()
                .filter(|token| token.token_type().eq(index))
                .map(|token| token.to_owned())
                .collect()
        } else {
            vec![]
        }
    };
}
pub struct QueryManager(HashMap<Lang, QueryData>);

impl Default for QueryManager {
    fn default() -> Self {
        let mut queries: HashMap<Lang, QueryData> = HashMap::new();
        insert_to_hash_map!(query queries, Lang::Javascript);
        insert_to_hash_map!(query queries, Lang::Rust);
        insert_to_hash_map!(query queries, Lang::Typescript);
        insert_to_hash_map!(query queries, Lang::Json);
        insert_to_hash_map!(query queries, Lang::Html);
        Self(queries)
    }
}

impl QueryManager {
    pub fn get_legend(&self, lang: &Lang) -> Arc<SemanticLegend> {
        let query_data = self.0.get(&lang).unwrap();
        query_data.modified_legend.clone()
    }

    pub fn get_unmodified_legend(&self, lang: &Lang) -> Arc<SemanticLegend> {
        let query_data = self.0.get(&lang).unwrap();
        query_data.legend.clone()
    }
    pub fn iter_query(
        &self,
        tree: &Option<Tree>,
        language: &Lang,
        ranged_source_code: &Vec<u8>,
    ) -> Vec<Token> {
        let mut data: Vec<Token> = vec![];

        if let Some(tree) = tree.as_ref() {
            let query_data = self.0.get(&language).unwrap();
            let query = &query_data.query;
            let mut query_cursor = QueryCursor::new();

            let _name = query.capture_names().to_vec();

            let matches =
                query_cursor.captures(query, tree.root_node(), ranged_source_code.as_slice());
            // let mut prev_range = Range {
            //     start_byte: 0,
            //     end_byte: 0,
            //     start_point: Point::default(),
            //     end_point: Point::default(),
            // };
            for matches in matches {
                for capture in matches.0.captures {
                    let range = capture.node.range();
                    let index = capture.index;

                    let token = Token::new(index, range);

                    // if range.eq(&prev_range) {
                    //     continue;
                    // }

                    data.push(token);
                    // prev_range = range;
                }
            }
        }

        data
    }

    pub fn sort_layer(&self, tokens: Vec<Token>, lang: &Lang) -> HighlightIter {
        let query_data = self.0.get(&lang).unwrap();
        let local_scope_index = &query_data._local_scope;
        let local_reference_index = &query_data._local_reference;
        let local_definition_index = &query_data._local_definition;

        // NOTE : There must be a better way to implement it but due to time constrain
        // Case when locals_index exists
        // If local_scope_index is Some then filter it out to a vec else an empty vec
        let scope = filter_out_to_vec!(tokens, local_scope_index);

        let local_definition = filter_out_to_vec!(tokens, local_definition_index);

        let local_reference = filter_out_to_vec!(tokens, local_reference_index);

        // NOTE : Injection is hard!!!!!!
        // let injection_content_index = &query_data.injection_content;
        // let injection_language_index = &query_data.injection_language;
        // // If injection_content is Some then filter it out to a vec else an empty vec
        // let injection_content: Vec<Token> = if let Some(index) = injection_content_index {
        //     tokens
        //         .iter()
        //         .filter(|token| token.identifier.eq(index))
        //         .map(|token| token.to_owned())
        //         .collect()
        // } else {
        //     vec![]
        // };

        // let injection_language = if let Some(index) = injection_language_index {
        //     tokens
        //         .iter()
        //         .filter(|token| token.identifier().eq(index))
        //         .map(|token| token.to_owned())
        //         .collect()
        // } else {
        //     vec![]
        // };

        let highlights_data: Vec<Token> = tokens
            .into_iter()
            .filter(|token| {
                // If scope is not empty it means local_scope is Some and do not add the highlights_data
                if !scope.is_empty() && token.token_type().eq(&local_scope_index.unwrap()) {
                    return false;
                }

                if !local_reference.is_empty()
                    && token.token_type().eq(&local_reference_index.unwrap())
                {
                    return false;
                }

                if !local_definition.is_empty()
                    && token.token_type().eq(&local_definition_index.unwrap())
                {
                    return false;
                }

                // if !injection_content.is_empty()
                //     && token.identifier().eq(&injection_content_index.unwrap())
                // {
                //     return false;
                // }

                true
            })
            .map(|token| token.to_owned())
            .collect();

        HighlightIter::new(
            highlights_data,
            scope,
            local_definition,
            local_reference,
            query_data.legend.clone(),
            query_data.modified_legend.clone(),
        )
    }
}

#[cfg(test)]
mod test {
    use crate::{treesitter_backend::parser::ParserHelper, FileManager};

    use super::QueryManager;

    #[test]
    fn test_query() {
        let mut parser_helper = ParserHelper::default();
        let mut file_manager = FileManager::new();
        let query_analyser = QueryManager::default();
        let id = file_manager.load_file("../../test_home/test.js").unwrap();
        let source_code = file_manager.read_source_code_in_bytes(&id.0).unwrap();
        let file_mutex = file_manager._get_file(&id.0);
        let file_language = file_manager.get_file_language(&id.0);
        file_manager.update_source_code(&id.0, &source_code);
        if let Ok(file_mutex) = file_mutex {
            parser_helper.append_tree(&id.0, file_mutex.clone());
            parser_helper.update_tree(&id.0, None);
            parser_helper.parse(&id.0, &file_language.clone().unwrap(), &source_code);
            let tokens = query_analyser.iter_query(
                &parser_helper.get_tree(&id.0),
                &file_language.clone().unwrap(),
                &source_code,
            );

            let result = query_analyser.sort_layer(tokens, &file_language.clone().unwrap());

            // println!("{:?}", query_analyser.get_legend(&crate::Lang::Javascript));
            println!("{:#2?}", result);
        }
    }
}
