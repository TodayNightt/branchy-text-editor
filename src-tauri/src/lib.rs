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
use tree_sitter::Tree;
pub mod backend_api;
pub mod treesitter_backend;

#[derive(Derivative)]
#[derivative(Debug, PartialEq, Eq, Hash, PartialOrd, Clone, Ord, Hash)]
pub struct OpenedFile {
    name: String,
    language: Option<Lang>,
    path: PathBuf,
    #[derivative(
        PartialEq = "ignore",
        Hash = "ignore",
        PartialOrd = "ignore",
        Ord = "ignore",
        Debug = "ignore"
    )]
    tree: Arc<Mutex<Option<Tree>>>,
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
}

#[derive(Debug, Default)]
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
    pub fn new(path_string: impl Into<String>) -> Result<Self, Error> {
        let path = PathBuf::from(path_string.into());
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
            tree: Arc::new(Mutex::new(None)),
        })
    }

    pub fn save(&self, changes: String) {
        fs::write(&self.path, changes).unwrap();
    }
}

impl FileManager {
    pub fn new() -> Self {
        Self {
            files: Box::default(),
        }
    }
    pub fn load_file(&mut self, path: impl Into<String>) -> Result<(u32, bool), Error> {
        let file = OpenedFile::new(path.into())?;
        let same_name_exist = self._search_same_name_exist(&file.name);
        let id = rand::thread_rng().next_u32();
        let files_list = self.files.as_mut();
        files_list.insert(id, Box::new(file));
        Ok((id, same_name_exist))
    }

    fn get_path_from_current_dir(&self) {
        
    }

    fn _get_file(&self, id: &u32) -> OpenedFile {
        self.files.as_ref().get(id).unwrap().as_ref().clone()
    }

    fn _search_same_name_exist(&self, name: &String) -> bool {
        let files_list = self.files.as_ref();
        for file in files_list.values() {
            if file.name.eq(name) {
                return true;
            }
        }
        false
    }

    pub fn save_file(&self, id: &u32, changes: String) {
        self._get_file(id).save(changes);
    }

    pub fn read_source_code_in_bytes(&self, id: &u32) -> Result<Vec<u8>, Error> {
        let mut file = self._get_file(id);
        let path = &file.path;
        let source_code = read_file(path)?;
        file.update_tree(None);
        file.parse(&source_code);
        Ok(source_code)
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
        let path = String::from("build.rs");
        let mut file_manager = FileManager::default();
        let file_info = file_manager.load_file(&path);
        let test_file = OpenedFile::new(&path).unwrap();
        if let Ok((id, _same_name_exist)) = file_info {
            assert_eq!(file_manager._get_file(&id), test_file);
        }
    }

    #[test]
    fn check_file_extension() {
        let file = OpenedFile::new(String::from("build.rs")).unwrap();
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
