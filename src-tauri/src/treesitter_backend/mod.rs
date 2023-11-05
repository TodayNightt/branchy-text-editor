use tree_sitter::Language;

use crate::Lang;

pub mod highlighter;
pub mod parser;

fn get_tree_sitter_language(lang: &Lang) -> Language {
    match lang {
        Lang::Javascript => tree_sitter_javascript::language(),
        Lang::Typescript => tree_sitter_javascript::language(),
        Lang::Rust => tree_sitter_rust::language(),
        _ => tree_sitter_javascript::language(),
    }
}
