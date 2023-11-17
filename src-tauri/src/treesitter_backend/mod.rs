use tree_sitter::Language;

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
            query_combine.push_str(tree_sitter_javascript::LOCALS_QUERY);
            // query_combine.push_str(tree_sitter_javascript::INJECTION_QUERY);
        }
        Lang::Typescript => {
            query_combine.push_str(tree_sitter_typescript::HIGHLIGHT_QUERY);
        }
        Lang::Rust => {
            query_combine.push_str(tree_sitter_rust::HIGHLIGHT_QUERY);
            // query_combine.push_str(tree_sitter_rust::INJECTIONS_QUERY);
        }
        Lang::Html => {
            query_combine.push_str(tree_sitter_html::HIGHLIGHT_QUERY);
            query_combine.push_str(tree_sitter_html::INJECTION_QUERY)
        }
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
            let mut local_scope = None;
            let mut local_definition = None;
            let mut local_reference = None;
            let mut injection_content = None;
            let mut injection_language = None;
            for (index, name) in query.capture_names().iter().enumerate() {
                let index = index as u32;
                match name.as_str() {
                    "local.scope" => local_scope = Some(index),
                    "local.definition" => local_definition = Some(index),
                    "local.reference" => local_reference = Some(index),
                    // NOTE : Injection is hard!!!  Maybe later~
                    "injection.content" => injection_content = Some(index),
                    "injection.language" => injection_language = Some(index),
                    _ => (),
                };
            }

            let legend = &query.capture_names().to_owned();
            let mut token_types: HashSet<String> = HashSet::new();
            let mut modifier: HashSet<String> = HashSet::new();
            for item in legend.iter() {
                // let mut item = item.to_owned();
                // if item.eq("punctuation.bracket") {
                //     item = String::from("brackets");
                // }
                let spilt: Vec<&str> = item.split(".").collect();
                let first = spilt.first();
                let last = spilt.last();
                token_types.insert(first.unwrap().to_string());
                if spilt.last().is_some() && first.unwrap().ne(last.unwrap()) {
                    modifier.insert(last.unwrap().to_string());
                }
            }
            let splited_legend = SemanticLegend {
                _token_types: token_types.into_iter().collect(),
                _token_modifier: modifier.into_iter().collect(),
            };
            $queries.insert(
                $lang,
                QueryData {
                    query,
                    _local_scope: local_scope,
                    _local_reference: local_reference,
                    _local_definition: local_definition,
                    _injection_content: injection_content,
                    _injection_language: injection_language,
                    legend: Arc::new(SemanticLegend::create(legend.to_vec())),
                    modified_legend: Arc::new(splited_legend),
                },
            );
        }
    };
}
