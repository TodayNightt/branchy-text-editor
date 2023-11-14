use tree_sitter::{Language, Parser, Query};

use crate::Lang;

pub mod highlighter;
pub mod parser;
pub mod query;
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
pub fn get_query_from_each_language(language: &Lang) -> String {
    let mut query_combine = String::new();
    match language {
        Lang::Javascript => {
            query_combine.push_str(tree_sitter_javascript::HIGHLIGHT_QUERY);
            // query_combine.push_str(tree_sitter_javascript::LOCALS_QUERY);
        }
        Lang::Typescript => {
            query_combine.push_str(tree_sitter_typescript::HIGHLIGHT_QUERY);
        }
        Lang::Rust => {
            query_combine.push_str(tree_sitter_rust::HIGHLIGHT_QUERY);
            // query_combine.push_str(tree_sitter_rust::INJECTIONS_QUERY);
        }
        Lang::Html => query_combine.push_str(tree_sitter_html::HIGHLIGHT_QUERY),
        _ => query_combine.push_str(tree_sitter_javascript::HIGHLIGHT_QUERY),
    }
    query_combine
}

#[macro_export]
macro_rules! insert_to_hash_map {
    (parser $parsers: ident,$lang : expr) => {
        let mut parser = Parser::new();
        let _ = parser.set_language(get_tree_sitter_language(&$lang));
        $parsers.insert($lang, parser);
    };

    (query $queries: ident , $lang: expr) => {
        let query = Query::new(
            get_tree_sitter_language(&$lang),
            get_query_from_each_language(&$lang).as_str(),
        );

        if let Ok(query) = query {
            $queries.insert(
                $lang,
                QueryData {
                    query,
                    locals_index: Some(1),
                    injection_index: Some(1),
                },
            );
        }
    };
}
