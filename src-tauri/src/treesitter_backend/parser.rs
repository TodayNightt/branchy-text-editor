use std::collections::HashMap;

use crate::{
    error::{MutexLockError, NotFoundError},
    insert_to_hash_map,
    language::Lang,
};

use derivative::Derivative;
use serde::{Deserialize, Serialize};
use specta::Type;
use tree_sitter::{InputEdit, Parser, Point, Tree};

use super::get_tree_sitter_language;

#[derive(Derivative, Serialize, Deserialize, Type)]
#[derivative(Debug)]
pub struct ChangesRange {
    pub start_byte: u32,
    pub old_end_byte: u32,
    pub new_end_byte: u32,
    pub start_position: CustomPoint,
    pub old_end_position: CustomPoint,
    pub new_end_position: CustomPoint,
}

impl Into<InputEdit> for ChangesRange {
    fn into(self) -> InputEdit {
        InputEdit {
            start_byte: self.start_byte as usize,
            old_end_byte: self.old_end_byte as usize,
            new_end_byte: self.new_end_byte as usize,
            start_position: self.start_position.into(),
            old_end_position: self.old_end_position.into(),
            new_end_position: self.new_end_position.into(),
        }
    }
}

#[derive(Derivative, Serialize, Deserialize, Type)]
#[derivative(Debug)]
pub struct CustomPoint {
    pub row: u32,
    pub column: u32,
}

impl Into<Point> for CustomPoint {
    fn into(self) -> Point {
        Point::new(self.row as usize, self.column as usize)
    }
}

impl CustomPoint {
    pub fn new(row: u32, column: u32) -> Self {
        Self { row, column }
    }
}

pub struct ParserHelper {
    parsers: HashMap<Lang, Parser>,
    trees: HashMap<u32, Option<Tree>>,
}
impl Default for ParserHelper {
    fn default() -> Self {
        let mut parsers: HashMap<Lang, Parser> = HashMap::new();
        insert_to_hash_map!(parser parsers, Lang::Javascript);
        insert_to_hash_map!(parser parsers, Lang::Rust);
        insert_to_hash_map!(parser parsers, Lang::Html);
        insert_to_hash_map!(parser parsers, Lang::Json);
        insert_to_hash_map!(parser parsers, Lang::Java);
        Self {
            parsers: parsers,
            trees: HashMap::default(),
        }
    }
}

impl ParserHelper {
    pub fn currently_supported_language(&self) -> Vec<Lang> {
        self.parsers.keys().map(|lang| lang.to_owned()).collect()
    }

    pub fn parser_exist(&self, language: &Lang) -> bool {
        self.parsers.contains_key(language)
    }

    pub fn remove_tree(&mut self, id: &u32) {
        self.trees.remove_entry(id);
    }

    pub fn append_tree(
        &mut self,
        id: &u32,
        file_language: Option<Lang>,
    ) -> Result<(), MutexLockError> {
        if file_language.is_some() {
            let id = id.clone();
            self.trees.insert(id, None);
        }

        Ok(())
    }
    pub fn update_tree(
        &mut self,
        id: &u32,
        input_edit: Option<ChangesRange>,
    ) -> Result<(), NotFoundError> {
        let tree_option = self
            .trees
            .get_mut(id)
            .ok_or_else(|| NotFoundError::TreeNotFoundError(id.clone()))?;
        if let Some(mut tree) = tree_option.take() {
            // Update the tree if tree = Some(tree) AND input = Some(input_edit)
            if let Some(input_edit) = input_edit {
                println!("{:?}", input_edit);
                tree.edit(&input_edit.into());
            }
            *tree_option = Some(tree);
        }

        Ok(())
    }

    pub fn get_tree(&self, id: &u32) -> Result<Option<Tree>, NotFoundError> {
        let tree = self
            .trees
            .get(id)
            .ok_or_else(|| NotFoundError::TreeNotFoundError(id.clone()))?;
        Ok(tree.clone())
    }

    pub fn parse(
        &mut self,
        id: &u32,
        language: &Lang,
        source_code: &Vec<u8>,
    ) -> Result<(), NotFoundError> {
        let old_tree = self
            .trees
            .get(id)
            .ok_or_else(|| NotFoundError::TreeNotFoundError(id.clone()))?;
        // FixMe : This will crash the program if self.language is not support or None
        let parser = self
            .parsers
            .get_mut(language)
            .ok_or_else(|| NotFoundError::ParserNotFoundError(language.to_string()))?;

        // parse the source_code
        let new_tree = parser.parse(source_code, old_tree.as_ref());

        // if let Some(tree) = old_tree.as_ref() {
        //     println!("Old tree{}", tree.root_node().to_sexp())
        // }
        // if let Some(tree) = new_tree.as_ref() {
        //     println!("New tree{:#?}", &tree.root_node().to_sexp());
        // }

        self.trees
            .entry(id.to_owned())
            .and_modify(|tree| *tree = new_tree);

        Ok(())
    }

    // pub fn load_parser(&mut self, lang: Lang, language: Language) {
    //     let mut parser = Parser::new();
    //     let _ = parser.set_language(language);
    //     self.parsers.insert(lang, parser);
    // }

    pub fn get_tree_sexp(&self, id: &u32) -> Result<String, NotFoundError> {
        let item = self
            .trees
            .get(id)
            .ok_or_else(|| NotFoundError::TreeNotFoundError(id.clone()))?;
        if let Some(tree) = item {
            Ok(tree.root_node().to_sexp().to_owned())
        } else {
            Ok("None".to_string())
        }
    }
}

#[cfg(test)]
mod test {

    use crate::FileManager;

    use super::{ChangesRange, CustomPoint, ParserHelper};
    #[test]
    fn check_parsing_a_tree() {
        let mut parser_helper = ParserHelper::default();
        let mut file_manager = FileManager::new();
        let id = file_manager.load_file("../../test_home/test.js").unwrap();
        let source_code = file_manager.read_source_code_in_bytes(&id.0).unwrap();
        let file_language = file_manager.get_file_language(&id.0).unwrap();
        parser_helper
            .append_tree(&id.0, file_language.clone())
            .unwrap();
        file_manager
            .update_source_code_for_file(&id.0, &source_code)
            .unwrap();
        parser_helper.update_tree(&id.0, None).unwrap();
        parser_helper
            .parse(
                &id.0,
                &file_language.clone().unwrap(),
                &file_manager.get_source_code_from_file(&id.0).unwrap(),
            )
            .unwrap();

        assert_eq!(parser_helper.get_tree_sexp(&id.0).unwrap(),"(program (function_declaration name: (identifier) parameters: (formal_parameters) body: (statement_block (for_statement initializer: (lexical_declaration (variable_declarator name: (identifier) value: (number))) condition: (expression_statement (binary_expression left: (identifier) right: (number))) increment: (update_expression argument: (identifier)) body: (statement_block)))) (expression_statement (call_expression function: (identifier) arguments: (arguments))))");
    }

    #[test]
    fn check_parsing_a_tree_with_changes() {
        let mut parser_helper = ParserHelper::default();
        let mut file_manager = FileManager::new();
        let id = file_manager.load_file("../../test_home/test.js").unwrap();
        let source_code = file_manager.read_source_code_in_bytes(&id.0).unwrap();
        let file_language = file_manager.get_file_language(&id.0).unwrap();
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
        let source_code = b"console.log(Hello World);".to_vec();

        let input_edit = Some(ChangesRange {
            start_byte: 8,
            old_end_byte: 8,
            new_end_byte: 14,
            start_position: CustomPoint::new(4, 0),
            old_end_position: CustomPoint::new(5, 8),
            new_end_position: CustomPoint::new(5, 14),
        });
        file_manager
            .update_source_code_for_file(&id.0, &source_code)
            .unwrap();
        parser_helper.update_tree(&id.0, input_edit).unwrap();
        parser_helper
            .parse(&id.0, &file_language.clone().unwrap(), &source_code)
            .unwrap();
    }
}
