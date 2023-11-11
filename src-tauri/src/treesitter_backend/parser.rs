use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use crate::{Lang, OpenedFile};

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

macro_rules! insert_to_hash_map {
    ($parsers: ident,$lang : expr) => {
        let mut parser = Parser::new();
        let _ = parser.set_language(get_tree_sitter_language(&$lang));
        $parsers.insert($lang, parser);
    };
}

pub struct ParserHelper {
    parsers: HashMap<Lang, Parser>,
    trees: HashMap<u32, Option<Tree>>,
}
impl Default for ParserHelper {
    fn default() -> Self {
        let mut parsers: HashMap<Lang, Parser> = HashMap::new();
        insert_to_hash_map!(parsers, Lang::Javascript);
        insert_to_hash_map!(parsers, Lang::Rust);
        insert_to_hash_map!(parsers, Lang::Html);
        Self {
            parsers: parsers,
            trees: HashMap::default(),
        }
    }
}

impl ParserHelper {
    pub fn append_tree(&mut self, id: &u32, file: Arc<Mutex<OpenedFile>>) {
        let file = file.lock().unwrap();
        if file.language.is_some() {
            let id = id.clone();
            self.trees.insert(id, None);
        }
    }
    pub fn update_tree(&mut self, id: &u32, input_edit: Option<ChangesRange>) {
        //FIXME : Handle the case that the tree is non-existence
        let tree_option = self.trees.get_mut(id).unwrap();
        if let Some(mut tree) = tree_option.take() {
            // Update the tree if tree = Some(tree) AND input = Some(input_edit)
            if let Some(input_edit) = input_edit {
                tree.edit(&input_edit.into());
            }
            *tree_option = Some(tree);
        }
    }

    pub fn get_tree(&self, id: &u32) -> Option<Tree> {
        let tree = self.trees.get(id).unwrap();
        println!("{:?}", &tree);
        tree.clone()
    }

    pub fn parse(&mut self, id: &u32, language: &Lang, source_code: &Vec<u8>) {
        let old_tree = self.trees.get(id).unwrap();
        // FixMe : This will crash the program if self.language is not support or None
        let parser = self.parsers.get_mut(language).unwrap();

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
    }

    // pub fn load_parser(&mut self, lang: Lang, language: Language) {
    //     let mut parser = Parser::new();
    //     let _ = parser.set_language(language);
    //     self.parsers.insert(lang, parser);
    // }

    pub fn get_tree_sexp(&self, id: &u32) -> String {
        let item = self.trees.get(id);
        if let Some(item) = item {
            if let Some(tree) = item {
                tree.root_node().to_sexp().to_owned()
            } else {
                "None".to_string()
            }
        } else {
            format!("Didn't found the item with the id : {}", id).to_string()
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
        let file_mutex = file_manager._get_file(&id.0);
        if let Ok(file_mutex) = file_mutex {
            parser_helper.append_tree(&id.0, file_mutex.clone());
            let mut file = file_mutex.lock().unwrap();
            file.update_source_code(&source_code);
            parser_helper.update_tree(&id.0, None);
            parser_helper.parse(&id.0, &file.language.clone().unwrap(), &file.source_code);
        }

        assert_eq!(parser_helper.get_tree_sexp(&id.0),"(program (function_declaration name: (identifier) parameters: (formal_parameters) body: (statement_block (for_statement initializer: (lexical_declaration (variable_declarator name: (identifier) value: (number))) condition: (expression_statement (binary_expression left: (identifier) right: (number))) increment: (update_expression argument: (identifier)) body: (statement_block)))) (expression_statement (call_expression function: (identifier) arguments: (arguments))))");
    }

    #[test]
    fn check_parsing_a_tree_with_changes() {
        let mut parser_helper = ParserHelper::default();
        let mut file_manager = FileManager::new();
        let id = file_manager.load_file("../../test_home/test.js").unwrap();
        let source_code = file_manager.read_source_code_in_bytes(&id.0).unwrap();
        let file_mutex = file_manager._get_file(&id.0).unwrap();
        parser_helper.append_tree(&id.0, file_mutex.clone());
        let mut file = file_mutex.lock().unwrap();
        file.update_source_code(&source_code);
        parser_helper.update_tree(&id.0, None);
        parser_helper.parse(&id.0, &file.language.clone().unwrap(), &source_code);
        let source_code = b"console.log(Hello World);".to_vec();

        let input_edit = Some(ChangesRange {
            start_byte: 8,
            old_end_byte: 8,
            new_end_byte: 14,
            start_position: CustomPoint::new(4, 0),
            old_end_position: CustomPoint::new(5, 8),
            new_end_position: CustomPoint::new(5, 14),
        });
        file.update_source_code(&source_code);
        parser_helper.update_tree(&id.0, input_edit);
        parser_helper.parse(&id.0, &file.language.clone().unwrap(), &source_code);
    }
}
