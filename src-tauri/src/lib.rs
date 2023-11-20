use derivative::Derivative;
use error::{Error, FileError, MutexLockError, NotFoundError, PathError};
use path_absolutize::*;
use rand::prelude::*;
use serde::{Deserialize, Serialize};
use specta::Type;
use std::{
    collections::HashMap,
    ffi::OsStr,
    fs::{self},
    path::{Path, PathBuf},
    sync::{Arc, Mutex},
};
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
    openable: bool,
    extension: String,
    source_code: Vec<u8>,
}

impl OpenedFile {
    fn new(path_string: impl Into<String>) -> Result<Self, Error> {
        let path = PathBuf::from(path_string.into());
        //Note : This functions should check whether the file is openable / or it is a text file
        let _file = fs::metadata(&path)?;
        let binding = path.clone();
        let file_extension = &binding
            .extension()
            .unwrap_or(OsStr::new("unknown"))
            .to_str()
            .ok_or_else(|| PathError::ToStringError)
            .map_err(|err| FileError::CreateFileError(err.to_string()))?;

        let lang = Lang::check_lang(file_extension);

        // if lang.is_none() {
        //     let mut extension = String::from(".");
        //     extension.push_str(&file_extension.to_owned());
        //     return Err(FileError::LanguageNotSupportError(extension));
        // }

        let name = &path
            .clone()
            .file_name()
            .unwrap_or(OsStr::new("unknown"))
            .to_os_string()
            .into_string()
            .map_err(|_err| PathError::ToStringError)
            .map_err(|err| FileError::CreateFileError(err.to_string()))?
            .to_owned();

        Ok(Self {
            name: name.to_string(),
            language: lang,
            path,
            openable: true,
            extension: file_extension.to_string(),
            source_code: vec![],
        })
    }

    fn language(&self) -> Option<Lang> {
        self.language.to_owned()
    }

    fn save(&self) -> Result<(), FileError> {
        Ok(fs::write(&self.path, &self.source_code)
            .map_err(|_err| FileError::SavingFileError(self.name.clone()))?)
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

impl ToString for Lang {
    fn to_string(&self) -> String {
        match self {
            Lang::Java => "java".to_string(),
            Lang::Javascript => "javascript".to_string(),
            Lang::Typescript => "typescript".to_string(),
            Lang::Ruby => "ruby".to_string(),
            Lang::Rust => "rust".to_string(),
            Lang::Html => "html".to_string(),
            Lang::Css => "css".to_string(),
            Lang::Python => "python".to_string(),
            Lang::Json => "json".to_string(),
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

    pub fn file_exists(&self, id: &u32) -> bool {
        self.files.contains_key(id)
    }

    pub fn close_file(&mut self, id: &u32) {
        self.files.as_mut().remove_entry(id);
    }

    pub fn get_file_language(&self, id: &u32) -> Result<Option<Lang>, Error> {
        let file_mutex = self._get_file(id)?;
        let file = file_mutex
            .try_lock()
            .map_err(|err| MutexLockError(err.to_string()))?;
        Ok(file.language().clone())
    }

    pub fn get_file_extension(&self, id: &u32) -> Result<String, Error> {
        let file_mutex = self._get_file(id)?;
        let file = file_mutex
            .try_lock()
            .map_err(|err| MutexLockError(err.to_string()))?;
        Ok(file.extension.clone())
    }

    pub fn update_source_code_for_file(
        &self,
        id: &u32,
        source_code: &Vec<u8>,
    ) -> Result<(), Error> {
        let file_mutex = self._get_file(id)?;
        let mut file = file_mutex
            .try_lock()
            .map_err(|err| MutexLockError(err.to_string()))?;

        file.update_source_code(source_code);
        Ok(())
    }

    pub fn load_file(&mut self, path: impl Into<String>) -> Result<(u32, bool), Error> {
        let file = OpenedFile::new(path.into())?;
        let same_name_exist = self._search_same_name_exist(&file.name)?;
        let id = rand::thread_rng().next_u32();
        let files_list = self.files.as_mut();
        files_list.insert(id, Arc::new(Mutex::new(file)));
        Ok((id, same_name_exist))
    }

    fn _get_file(&self, id: &u32) -> Result<Arc<Mutex<OpenedFile>>, NotFoundError> {
        Ok(self
            .files
            .get(&id)
            .ok_or_else(|| NotFoundError::FileNotFoundError(*id))?
            .clone())
    }

    fn _search_same_name_exist(&self, name: &String) -> Result<bool, Error> {
        let files_list = self.files.as_ref();
        for file in files_list.values() {
            let file = file
                .try_lock()
                .map_err(|err| MutexLockError(err.to_string()))?;
            if file.name.eq(name) {
                return Ok(true);
            }
        }
        Ok(false)
    }

    pub fn save_file(&self, id: &u32) -> Result<(), Error> {
        let file_mutex = self
            ._get_file(&id)
            .map_err(|err| FileError::SavingFileError(err.to_string()))?;
        let file = file_mutex
            .try_lock()
            .map_err(|err| MutexLockError(err.to_string()))?;
        Ok(file.save()?)
    }

    pub fn read_source_code_in_bytes(&self, id: &u32) -> Result<Vec<u8>, Error> {
        let file_mutex = self
            ._get_file(id)
            .map_err(|err| FileError::ReadFileError(err.to_string()))?;
        let mut file = file_mutex
            .try_lock()
            .map_err(|err| MutexLockError(err.to_string()))?;
        let path = &file.path;
        let source_code = read_file(path)?;
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

pub fn read_file(path: &Path) -> Result<Vec<u8>, FileError> {
    Ok(fs::read(path)
        .map_err(|_err| FileError::ReadFileError(path.to_str().unwrap().to_string()))?)
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

pub fn get_directory_items(dir: &PathBuf, recursion: u32) -> Result<Vec<DirectoryItem>, Error> {
    let mut directory_item: Vec<DirectoryItem> = vec![];
    let entries = dir.read_dir()?;

    for entry in entries {
        if let Ok(entry) = entry {
            if entry.file_type()?.is_file() {
                directory_item.push(DirectoryItem::create_file(
                    entry
                        .file_name()
                        .into_string()
                        .map_err(|_err| PathError::ToStringError)?,
                    entry.path().absolutize()?.to_path_buf(),
                ))
            } else if entry.file_type().unwrap().is_dir() && recursion > 0 {
                let recursion = recursion - 1;
                let item = get_directory_items(&entry.path(), recursion)?;
                directory_item.push(DirectoryItem::create_directory(
                    entry
                        .file_name()
                        .into_string()
                        .map_err(|_err| PathError::ToStringError)?,
                    entry.path().absolutize()?.to_path_buf(),
                    item,
                ));
            }
        }
    }
    Ok(directory_item)
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
