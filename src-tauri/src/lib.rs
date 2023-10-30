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

use path_absolutize::*;
use rand::prelude::*;
use serde::{Deserialize, Serialize};
use specta::Type;
use std::{
    cell::RefCell,
    collections::HashMap,
    ffi::OsStr,
    fs,
    io::Error,
    path::{Path, PathBuf},
    sync::Mutex,
};
// use thiserror::Error;
use tree_sitter::{Language, Parser};
pub mod backend_api;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Serialize, Deserialize, Type)]
pub struct OpenedFile {
    name: String,
    source_code: Vec<u8>,
    language: Option<Lang>,
}

// #[derive(Default)]
// pub struct ParserLoader {
//     pub parsers: HashMap<Lang, RefCell<Parser>>,
// }

// impl ParserLoader {
//     pub fn load_parse(&mut self, lang: Lang, language: Language) {
//         let mut parser = Parser::new();
//         let _ = parser.set_language(language);
//         self.parsers.insert(lang, RefCell::new(parser));
//     }
// }

#[derive(Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Clone, Serialize, Deserialize, Type)]
pub enum Lang {
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
    files: Box<HashMap<u32, Box<OpenedFile>>>,
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

impl Lang {
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
            name: path
                .file_name()
                .unwrap_or(OsStr::new("unknown"))
                .to_os_string()
                .into_string()
                .unwrap(),
            source_code: read_file(path)?.into(),
            language: Lang::check_lang(
                path.extension()
                    .unwrap_or(OsStr::new("unknown"))
                    .to_str()
                    .unwrap(),
            ),
        })
    }

    pub fn save(&self) {}
}

impl FileManager {
    pub fn new() -> Self {
        Self {
            files: Box::default(),
        }
    }
    pub fn load_file(&mut self, path: &Path) -> Result<u32, Error> {
        let files_list = self.files.as_mut();
        let file = OpenedFile::new(path)?;
        let id = rand::thread_rng().next_u32();
        files_list.insert(id, Box::new(file));
        Ok(id)
    }

    fn _get_file(&self, id: &u32) -> OpenedFile {
        self.files.as_ref().get(id).unwrap().as_ref().clone()
    }
}

impl StateManager {
    pub fn new() -> Self {
        Self {
            file_manager: Mutex::new(FileManager::new()),
            editor_config: Mutex::new(EditorConfig::default()),
        }
    }
}

pub fn read_file(path: &Path) -> Result<Vec<u8>, Error> {
    fs::read(path)
}

#[derive(Debug, Serialize, Deserialize, Type)]
pub struct DirectoryItem {
    is_file: bool,
    name: String,
    path: PathBuf,
    childrens: Option<Vec<DirectoryItem>>,
}

impl DirectoryItem {
    pub fn create_file(name: String, path: PathBuf) -> Self {
        Self {
            is_file: true,
            name,
            path,
            childrens: None,
        }
    }

    pub fn create_directory(name: String, path: PathBuf, item: Vec<DirectoryItem>) -> Self {
        Self {
            is_file: false,
            name,
            path,
            childrens: Some(item),
        }
    }
}

pub fn get_directory_items(dir: &PathBuf, recursion: i32) -> Vec<DirectoryItem> {
    let mut directory_item: Vec<DirectoryItem> = vec![];
    let entries = dir.read_dir().unwrap();

    for entry in entries {
        if let Ok(entry) = entry {
            if entry.file_type().unwrap().is_file() {
                directory_item.push(DirectoryItem::create_file(
                    entry.file_name().into_string().unwrap(),
                    entry.path().absolutize().unwrap().to_path_buf(),
                ))
            } else if entry.file_type().unwrap().is_dir() && recursion > 0 {
                let recursion = recursion - 1;
                let item = get_directory_items(&entry.path(), recursion);
                directory_item.push(DirectoryItem::create_directory(
                    entry.file_name().into_string().unwrap(),
                    entry.path().absolutize().unwrap().to_path_buf(),
                    item,
                ));
            }
        }
    }
    directory_item
}

// pub fn get_all_directory_along_the_path(dir: &PathBuf) -> HashMap<String, Vec<DirectoryItem>> {
//     let mut directories: HashMap<String, Vec<DirectoryItem>> = HashMap::new();
//     if let Some(dir) = dir.parent() {
//         let path_buf = dir.to_path_buf();
//         let directory_items = get_directory_items(&path_buf, 1);
//         directories.insert(
//             path_buf.into_os_string().into_string().unwrap(),
//             directory_items,
//         );
//     }
//     directories
// }

#[cfg(test)]
mod test {

    use crate::backend_api::file_system::get_file_system_info;

    use super::*;
    #[test]
    fn check_file_insert() {
        let path = Path::new("build.rs");
        let mut file_manager = FileManager::default();
        let file_id = file_manager.load_file(path);
        let test_file = OpenedFile::new(path).unwrap();
        if let Ok(id) = file_id {
            assert_eq!(file_manager._get_file(&id), test_file);
        }
    }

    #[test]
    fn check_file_extension() {
        let file = OpenedFile::new(Path::new("build.rs")).unwrap();
        assert_eq!(file.language, Some(Lang::Rust));
    }

    #[test]
    fn check_directory_item() {
        let _items = get_directory_items(&PathBuf::from("."), 2);
        // dbg!(items);
    }

    #[test]
    fn test_path_buf() {
        dbg!(get_file_system_info(None));
    }
}
