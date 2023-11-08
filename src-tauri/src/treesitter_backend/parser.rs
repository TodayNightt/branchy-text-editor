use crate::OpenedFile;

use derivative::Derivative;
use serde::{Deserialize, Serialize};
use specta::Type;
use tree_sitter::{InputEdit, Parser, Point};

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

impl OpenedFile {
    pub fn update_tree(&mut self, input_edit: Option<ChangesRange>) {
        let mut old_tree = self.tree.lock().unwrap();
        // Update the tree if tree = Some(tree) AND input = Some(input_edit)
        if let Some(input_edit) = input_edit {
            if let Some(mut tree) = old_tree.take() {
                tree.edit(&input_edit.into());
                *old_tree = Some(tree);
            }
        }
    }
    pub fn parse(&mut self) {
        let mut old_tree = self.tree.lock().unwrap();
        let mut parser = Parser::new();

        // FixMe : This will crash the program if self.language is not support or None
        let _ = parser.set_language(get_tree_sitter_language(&self.language.clone().unwrap()));

        // parse the source_code
        let new_tree = parser.parse(&self.source_code, old_tree.as_ref());

        // if let Some(tree) = old_tree.as_ref() {
        //     println!("Old tree{}", tree.root_node().to_sexp())
        // }
        // if let Some(tree) = new_tree.as_ref() {
        //     println!("New tree{:#?}", tree.root_node().to_sexp());
        // }

        if let Some(tree) = new_tree {
            *old_tree = Some(tree);
        }
    }
}

// #[cfg(test)]
// mod test {

//     use crate::FileManager;

//     use super::{ChangesRange, CustomPoint};
//     // #[test]
//     // fn check_parsing_a_tree() {
//     //     let mut file_manager = FileManager::new();
//     //     let id = file_manager.load_file("../../test_home/test.js").unwrap();
//     //     let source_code = file_manager.read_source_code_in_bytes(&id.0).unwrap();
//     //     let mut parse_load = ParserLoader::default();
//     //     parse_load.load_parse(crate::Lang::Javascript, tree_sitter_javascript::language());

//     //     file_manager
//     //         ._get_file(&id.0)
//     //         .parse(&parse_load.parsers, source_code, None);
//     // }

//     #[test]
//     fn check_parsing_a_tree_with_changes() {
//         let mut file_manager = FileManager::new();
//         let id = file_manager.load_file("../../test_home/test.js").unwrap();
//         let source_code = file_manager.read_source_code_in_bytes(&id.0).unwrap();
//         let mut file = file_manager._get_file(&id.0);
//         file.update_tree(None);
//         file.parse(&source_code);

//         let source_code = b"console.log(Hello World);".to_vec();

//         let input_edit = Some(ChangesRange {
//             start_byte: 8,
//             old_end_byte: 8,
//             new_end_byte: 14,
//             start_position: CustomPoint::new(4, 0),
//             old_end_position: CustomPoint::new(5, 8),
//             new_end_position: CustomPoint::new(5, 14),
//         });
//         file.update_tree(input_edit);
//         file.parse(&source_code);
//     }
