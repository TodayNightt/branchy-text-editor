use std::{collections::HashMap, sync::Arc};

use super::query::{RangePoint, SemanticLegend, Token};

pub struct MonacoHighlights;

#[derive(Debug, Default)]
pub struct HighlightIter {
    highlights: Vec<Token>,
    scopes: Vec<Token>,
    local_definition: Vec<Token>,
    local_reference: Vec<Token>,
    legend_based_on: Arc<SemanticLegend>,
    legend_modified_to: Arc<SemanticLegend>,
}

impl HighlightIter {
    pub fn new(
        highlights: Vec<Token>,
        scopes: Vec<Token>,
        local_definition: Vec<Token>,
        local_reference: Vec<Token>,
        legend_based_on: Arc<SemanticLegend>,
        legend_modified_to: Arc<SemanticLegend>,
    ) -> Self {
        Self {
            highlights,
            scopes,
            local_definition,
            local_reference,
            legend_based_on,
            legend_modified_to,
        }
    }
    pub fn analyse_layer(&mut self) -> Highlights {
        let mut result = vec![];
        let mut _current_scope = None;
        let mut cached_index = HashMap::new();

        for token in &self.highlights {
            let range = token.range();

            // NOTE : This check is very inefficient
            // Call the method to see whether the range in inside a scope
            _current_scope = self.check_whether_in_scope(range);
            let mut new_token = *token;

            // If is inside a scope
            if _current_scope.is_some_and(|scope| scope.range().within(range)) {
                // Look for local_reference of local_definition if any
                let local_reference_token = self
                    .local_reference
                    .iter()
                    .find(|reference| reference.range().eq(&range));
                let local_definition_token = self
                    .local_definition
                    .iter()
                    .find(|definition| definition.range().eq(&range));

                // If either both exists replace it with the old token
                if local_reference_token.is_some() {
                    new_token = local_reference_token.unwrap().to_owned();
                } else if local_definition_token.is_some() {
                    new_token = local_definition_token.unwrap().to_owned()
                }
            }

            let identifier = new_token.token_type();

            // If the cached_index does not contain the key, call remaping and cache it
            cached_index
                .entry(identifier)
                .or_insert_with(|| self.remaping(identifier));
            let new_index = cached_index.get(&identifier).unwrap();

            new_token.remap_token_n_modifier(new_index.to_owned());
            result.push(new_token);
        }
        Highlights(result)
    }

    fn check_whether_in_scope(&self, range: RangePoint) -> Option<Token> {
        if !self.scopes.is_empty() {
            for scope in &self.scopes {
                if scope.range().within(range) {
                    return Some(scope.to_owned());
                }
            }
        }
        None
    }

    fn remaping(&self, identifier_index: u32) -> (u32, u32) {
        let original_legend = self.get_unmodified_legend().get_token_types();
        let modified_legend = self.get_modified_legend();
        let modified_token_type = modified_legend.get_token_types();
        let modified_modifier = modified_legend.get_modifier();
        let identifier = original_legend
            .get(identifier_index as usize)
            .unwrap()
            .to_owned();

        let spilt_identifier: Vec<&str> = identifier.split('.').collect();
        let first = spilt_identifier.first();
        let last = spilt_identifier.last();

        let token_type_index = modified_token_type
            .iter()
            .enumerate()
            .find(|val| val.1.eq(first.unwrap()))
            .unwrap()
            .0 as u32;
        let mut modifier_index = 0;

        if last.is_some() && first.unwrap().ne(last.unwrap()) {
            modifier_index = modified_modifier
                .iter()
                .enumerate()
                .find(|val| val.1.eq(last.unwrap()))
                .unwrap()
                .0 as u32;
        }

        (token_type_index, modifier_index)
    }

    pub fn get_highlights_data(&self) -> Vec<Token> {
        self.highlights.clone()
    }
    pub fn get_scopes_data(&self) -> Vec<Token> {
        self.scopes.clone()
    }

    pub fn get_unmodified_legend(&self) -> Arc<SemanticLegend> {
        self.legend_based_on.clone()
    }
    pub fn get_modified_legend(&self) -> Arc<SemanticLegend> {
        self.legend_modified_to.clone()
    }
}

#[derive(Debug)]
pub struct Highlights(Vec<Token>);

impl MonacoHighlights {
    pub fn emit(highlights: &Highlights) -> Vec<u32> {
        let mut data = vec![];
        let mut prev_line = 0;
        let mut prev_col = 0;

        for token in &highlights.0 {
            let range = token.range();
            let length = token.length();
            let token_type_index = token.token_type();
            let modifier_index = token.modifier();

            // println!(
            //     "index{}, current_line{} prev_line{}",
            //     &identifier_index,
            //     range.start_row(),
            //     prev_line
            // );

            let delta_start_row: u32 = range.start_row() - prev_line;
            let delta_start_col = if range.start_row() == prev_line {
                range.start_col() - prev_col
            } else {
                range.start_col()
            };

            data.push(delta_start_row); // Delta start row from previous match
            data.push(delta_start_col); // Delta start col from previous match
            data.push(length); // The length of the match
            data.push(token_type_index); // token index
            data.push(modifier_index); // Modifier currently 0 because all is in index
            prev_line = range.start_row();
            prev_col = range.start_col();
        }
        data
    }
}

#[cfg(test)]
mod test {

    use crate::treesitter_backend::query::QueryManager;
    use crate::{FileManager, treesitter_backend::parser::ParserHelper};

    use super::MonacoHighlights;

    #[test]
    fn see_the_node_children() {
        let mut parser_helper = ParserHelper::default();
        let mut file_manager = FileManager::new();
        let query_iter = QueryManager::default();
        let id = file_manager.load_file("../../test_home/test.js").unwrap();
        let source_code = file_manager.read_source_code_in_bytes(&id.0).unwrap();
        let file_language = &file_manager.get_file_language(&id.0).unwrap().clone();
        parser_helper
            .append_tree(&id.0, file_language.clone())
            .unwrap();
        file_manager
            .update_source_code_for_file(&id.0, &source_code)
            .unwrap();
        parser_helper.update_tree(&id.0, None).unwrap();
        parser_helper
            .parse(&id.0, &file_language.clone().unwrap(), &source_code)
            .unwrap();
        let tokens = query_iter
            .iter_query(
                &parser_helper.get_tree(&id.0).unwrap(),
                &file_language.clone().unwrap(),
                &source_code,
            )
            .unwrap();

        let mut token_data = query_iter
            .sort_layer(tokens, &file_language.clone().unwrap())
            .unwrap();
        // println!("{:#1?}", &token_data);

        let highlights = token_data.analyse_layer();
        // println!("{:#1?}", &highlights);

        let _result = MonacoHighlights::emit(&highlights);
    }
}
