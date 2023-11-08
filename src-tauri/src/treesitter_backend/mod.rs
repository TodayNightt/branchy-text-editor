use tree_sitter::Language;

use crate::Lang;

pub mod highlighter;
pub mod parser;

pub fn get_tree_sitter_language(lang: &Lang) -> Language {
    match lang {
        Lang::Javascript => tree_sitter_javascript::language(),
        Lang::Typescript => tree_sitter_javascript::language(),
        Lang::Rust => tree_sitter_rust::language(),
        _ => tree_sitter_javascript::language(),
    }
}
pub fn get_query_from_each_language(language: &Lang) -> &str {
    match language {
        Lang::Javascript => tree_sitter_javascript::HIGHLIGHT_QUERY,
        Lang::Rust => tree_sitter_rust::HIGHLIGHT_QUERY,
        _ => tree_sitter_javascript::HIGHLIGHT_QUERY,
    }
}


