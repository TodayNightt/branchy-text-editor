use backend_api::file_system::CustomError;
use derivative::Derivative;
use path_absolutize::*;
use rand::prelude::*;
use serde::{Deserialize, Serialize};
use specta::Type;
use std::{
    collections::HashMap,
    ffi::OsStr,
    fs,
    io::Error,
    path::{Path, PathBuf},
    sync::{Arc, Mutex},
};
use tree_sitter::{Query, Tree};
use treesitter_backend::{parser::ParserHelper, query::QueryManager, theme::ThemeConfig};
pub mod backend_api;
pub mod error;
pub mod treesitter_backend;

#[derive(Derivative)]
#[derivative(Debug, PartialEq, Eq, Hash, PartialOrd, Clone, Ord, Hash)]
pub struct OpenedFile {
    name: String,
    language: Option<Lang>,
    path: PathBuf,
    source_code: Vec<u8>,
    #[derivative(
        PartialEq = "ignore",
        Hash = "ignore",
        PartialOrd = "ignore",
        Ord = "ignore",
        Debug = "ignore"
    )]
    tree: Arc<Mutex<Option<Tree>>>,
    #[derivative(
        PartialEq = "ignore",
        Hash = "ignore",
        PartialOrd = "ignore",
        Ord = "ignore",
        Debug = "ignore"
    )]
    query: Arc<Option<Query>>,
}

impl OpenedFile {
    fn new(path_string: impl Into<String>) -> Result<Self, Error> {
        let path = PathBuf::from(path_string.into());
        //Note : This functions should check whether the file is openable / or it is a text file
        let _file = fs::metadata(&path).unwrap();
        let lang = Lang::check_lang(
            path.extension()
                .unwrap_or(OsStr::new("unknown"))
                .to_str()
                .unwrap(),
        );
        Ok(Self {
            name: path
                .file_name()
                .unwrap_or(OsStr::new("unknown"))
                .to_os_string()
                .into_string()
                .unwrap(),
            language: lang,
            path,
            source_code: vec![],
            tree: Arc::new(Mutex::new(None)),
            query: Arc::new(None),
        })
    }

    fn language(&self) -> Option<Lang> {
        self.language.to_owned()
    }

    fn save(&self) -> Result<(), std::io::Error> {
        fs::write(&self.path, &self.source_code)?;
        Ok(())
    }

    fn update_source_code(&mut self, source_code: &Vec<u8>) {
        self.source_code = source_code.to_owned();
    }
}

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
    Json,
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
            "json" => Some(Lang::Json),
            _ => None,
        }
    }
}

#[derive(Debug, Default)]
pub struct FileManager {
    files: Box<HashMap<u32, Arc<Mutex<OpenedFile>>>>,
}

impl FileManager {
    pub fn new() -> Self {
        Self {
            files: Box::default(),
        }
    }

    pub fn close_file(&mut self, id: &u32) {
        self.files.as_mut().remove_entry(id);
    }

    pub fn get_file_language(&self, id: &u32) -> Option<Lang> {
        let file_mutex = self._get_file(id);

        if let Ok(file_mutex) = file_mutex.clone() {
            let file = file_mutex.lock().unwrap();
            file.language().clone()
        } else {
            None
        }
    }

    pub fn update_source_code(&self, id: &u32, source_code: &Vec<u8>) {
        let file_mutex = self._get_file(id);

        if let Ok(file_mutex) = file_mutex {
            let mut file = file_mutex.lock().unwrap();

            file.update_source_code(source_code);
        }
    }

    pub fn update_source_code_for_file(&self, id: &u32, source_code: &Vec<u8>) {
        let file_mutex = self._get_file(id);
        if let Ok(file) = file_mutex {
            let mut file = file.lock().unwrap();
            file.update_source_code(source_code);
        }
    }

    pub fn load_file(&mut self, path: impl Into<String>) -> Result<(u32, bool), Error> {
        let file = OpenedFile::new(path.into())?;
        let same_name_exist = self._search_same_name_exist(&file.name);
        let id = rand::thread_rng().next_u32();
        let files_list = self.files.as_mut();
        files_list.insert(id, Arc::new(Mutex::new(file)));
        Ok((id, same_name_exist))
    }

    fn _get_file(&self, id: &u32) -> Result<Arc<Mutex<OpenedFile>>, String> {
        if let Some(file) = self.files.as_ref().get(&id) {
            Ok(file.clone())
        } else {
            Err("File cannot be found _get_file".to_string())
        }
    }

    fn _search_same_name_exist(&self, name: &String) -> bool {
        let files_list = self.files.as_ref();
        for file in files_list.values() {
            let file = file.lock().unwrap();
            if file.name.eq(name) {
                return true;
            }
        }
        false
    }

    pub fn save_file(&self, id: &u32) -> Result<(), CustomError> {
        let file = self
            ._get_file(&id)
            .map_err(|err| CustomError::GetFileError { message: err })?;
        file.lock().unwrap().save()?;
        Ok(())
    }

    pub fn read_source_code_in_bytes(&self, id: &u32) -> Result<Vec<u8>, String> {
        let file_mutex = self._get_file(id)?;
        let mut file = file_mutex.lock().unwrap();
        let path = &file.path;
        let source_code = read_file(path).map_err(|_err| "Cannot read file".to_string())?;
        file.update_source_code(&source_code);
        Ok(source_code)
    }
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct LastSection;

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct EditorConfig {
    theme: Mutex<ThemeConfig>,
    last_section: Option<Mutex<LastSection>>,
}

#[derive(Default, Serialize)]
pub struct StateManager {
    #[serde(skip)]
    pub file_manager: Mutex<FileManager>,
    pub editor_config: Mutex<EditorConfig>,
    #[serde(skip)]
    parser_helper: Mutex<ParserHelper>,
    #[serde(skip)]
    query_iter: QueryManager,
}

impl StateManager {
    pub fn new() -> Self {
        Self {
            file_manager: Mutex::new(FileManager::new()),
            editor_config: Mutex::new(EditorConfig::default()),
            parser_helper: Mutex::new(ParserHelper::default()),
            query_iter: QueryManager::default(),
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

// #[cfg(test)]
// mod test {

//     use crate::backend_api::file_system::get_file_system_info;

//     use super::*;
//     #[test]
//     fn check_file_insert() {
//         let path = String::from("build.rs");
//         let mut file_manager = FileManager::default();
//         let file_info = file_manager.load_file(&path);
//         let test_file = OpenedFile::new(&path).unwrap();
//         if let Ok((id, _same_name_exist)) = file_info {
//             assert_eq!(file_manager._get_file(&id), test_file);
//         }
//     }

//     #[test]
//     fn check_file_extension() {
//         let file = OpenedFile::new(String::from("build.rs")).unwrap();
//         assert_eq!(file.language, Some(Lang::Rust));
//     }

//     #[test]
//     fn check_directory_item() {
//         let _items = get_directory_items(&PathBuf::from("."), 2);
//         // dbg!(items);
//     }

//     #[test]
//     fn test_path_buf() {
//         dbg!(get_file_system_info(None));
//     }
// }
