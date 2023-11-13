use tree_sitter::{Language, Parser, Query};

use crate::Lang;

pub mod highlighter;
pub mod parser;
pub mod theme;

pub fn get_tree_sitter_language(lang: &Lang) -> Language {
    match lang {
        Lang::Javascript => tree_sitter_javascript::language(),
        Lang::Typescript => tree_sitter_typescript::language_typescript(),
        Lang::Rust => tree_sitter_rust::language(),
        Lang::Html => tree_sitter_html::language(),
        _ => tree_sitter_javascript::language(),
    }
}
pub fn get_query_from_each_language(language: &Lang) -> &str {
    match language {
        Lang::Javascript => tree_sitter_javascript::HIGHLIGHT_QUERY,
        Lang::Typescript => tree_sitter_typescript::HIGHLIGHT_QUERY,
        Lang::Rust => tree_sitter_rust::HIGHLIGHT_QUERY,
        Lang::Html => tree_sitter_html::HIGHLIGHT_QUERY,
        _ => tree_sitter_javascript::HIGHLIGHT_QUERY,
    }
}
