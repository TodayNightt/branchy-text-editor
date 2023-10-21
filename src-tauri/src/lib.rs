// use tree_sitter_cli::highlight::Theme;
// use tree_sitter_highlight::{Highlight, Highlighter, HtmlRenderer};
// use tree_sitter_loader::{Config, Loader};

// // Adapter function for interoperating with Tree-sitter's highlight library.
// //
// // * `code` - The code snippet to highlight.
// // * `scope` - The TextMate scope identifying the language of the code snippet.
// pub fn highlight_adapter(code: &str, scope: &str) -> String {
//     // The directory to search for parsers
//     let parser_directory = std::env::current_dir().unwrap().join("parsers");

//     let theme = Theme::default();

//     // The loader is used to load parsers
//     let loader = {
//         let mut loader = Loader::new().unwrap();
//         let config = {
//             let parser_directories = vec![parser_directory];
//             Config { parser_directories }
//         };
//         loader.find_all_languages(&config).unwrap();
//         loader.configure_highlights(&theme.highlight_names);
//         loader
//     };

//     // Retrieve the highlight config for the given language scope
//     let config = loader
//         .language_configuration_for_scope(scope)
//         .unwrap()
//         .and_then(|(language, config)| config.highlight_config(language).ok())
//         .unwrap()
//         .unwrap();

//     let code = code.as_bytes();

//     // Highlight the code
//     let mut highlighter = Highlighter::new();
//     let highlights = highlighter.highlight(config, code, None, |_| None).unwrap();

//     // Render and return the highlighted code as an HTML snippet
//     let get_style_css = |h: Highlight| theme.styles[h.0].css.as_ref().unwrap().as_bytes();
//     let mut renderer = HtmlRenderer::new();
//     renderer.render(highlights, code, &get_style_css).unwrap();
//     renderer.lines().collect()
// }

use debug_ignore::DebugIgnore;
use rand::prelude::*;
use serde::{Deserialize, Serialize};
use std::{
    cell::RefCell,
    collections::HashMap,
    ffi::{OsStr, OsString},
    fs,
    io::Error,
    path::Path,
    sync::{Arc, Mutex, RwLock},
};
use tree_sitter::{Language, Parser, Tree};
pub mod backend_api;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Serialize, Deserialize)]
pub struct OpenedFile {
    name: OsString,
    source_code: Vec<u8>,
    language: Option<Languages>,
}

#[derive(Default)]
pub struct ParserLoader {
    pub parsers: HashMap<Languages, RefCell<Parser>>,
}

impl ParserLoader {
    pub fn load_parse(&mut self, lang: Languages, language: Language) {
        let mut parser = Parser::new();
        let _ = parser.set_language(language);
        self.parsers.insert(lang, RefCell::new(parser));
    }
}

#[derive(Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Clone, Serialize, Deserialize)] // #[derive(Type)]
pub enum Languages {
    Javascript,
    Typescript,
    Rust,
    Python,
    Java,
    Ruby,
    Html,
    Css,
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct FileManager {
    files: RefCell<HashMap<u64, RefCell<OpenedFile>>>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Theme;

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct LastSection;

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct EditorConfig {
    theme: RefCell<Theme>,
    last_section: Option<RefCell<LastSection>>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct StateManager {
    pub file_manager: Mutex<FileManager>,
    pub editor_config: Mutex<EditorConfig>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct RequestParse {
    id: u64,
    source_code: String,
}

pub enum Responses {
    FileInfo,
    FileID,
    FileRead,
}

pub enum Requests {
    RequestParse,
    RequestOpenFile,
    RequestConfigChange,
    RequestsSaveFile,
    RequestCloseFile,
    Request,
}

impl Languages {
    pub fn check_lang(file_extension: &str) -> Option<Self> {
        match file_extension {
            "java" => Some(Self::Java),
            "rs" => Some(Self::Rust),
            "ts" | "tsx" => Some(Self::Typescript),
            "js" | "jsx" => Some(Self::Javascript),
            "py" => Some(Self::Python),
            "rb" => Some(Self::Ruby),
            "htm" | "html" => Some(Self::Html),
            "css" | "scss" | "sass" => Some(Self::Css),
            _ => None,
        }
    }
}

impl OpenedFile {
    pub fn new(path: &Path) -> Result<Self, Error> {
        Ok(Self {
            name: path.file_name().unwrap_or(OsStr::new("unknown")).into(),
            source_code: read_file(path)?.into(),
            language: Languages::check_lang(
                path.extension()
                    .unwrap_or(OsStr::new("unknown"))
                    .to_str()
                    .unwrap(),
            ),
        })
    }

    // pub fn parse(&self, parser: &ParserLoader) -> Option<Tree> {
    //     if let Some(language) = &self.language {
    //         let mut parser = parser.parsers.get(&language).unwrap().borrow_mut();
    //         return parser.parse(&self.source_code, None);
    //     }
    //     None
    // }

    pub fn save(&self) {}
}

impl FileManager {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn load_file(&mut self, path: &Path) -> Result<u64, Error> {
        let mut files_list = self.files.borrow_mut();
        let file = OpenedFile::new(path)?;
        let id = rand::thread_rng().next_u64();
        files_list.insert(id, RefCell::new(file));
        Ok(id)
    }

    fn _get_file(&self, id: u64) -> OpenedFile {
        self.files.borrow().get(&id).unwrap().borrow().clone()
    }
}

impl StateManager {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn load(&mut self, path: &Path) -> Result<u64, Error> {
        let mut file_manager = self.file_manager.lock().unwrap();
        file_manager.load_file(path)
        // self.file_manager.borrow_mut().load_file(path)
    }
}

pub fn read_file(path: &Path) -> Result<Vec<u8>, Error> {
    fs::read(path)
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn check_file_insert() {
        let path = Path::new("build.rs");
        let mut file_manager = FileManager::default();
        let file_id = file_manager.load_file(path);
        let test_file = OpenedFile::new(path).unwrap();
        if let Ok(id) = file_id {
            assert_eq!(file_manager._get_file(id), test_file);
        }
    }

    #[test]
    fn check_file_extension() {
        let file = OpenedFile::new(Path::new("build.rs")).unwrap();
        assert_eq!(file.language, Some(Languages::Rust));
    }
}
