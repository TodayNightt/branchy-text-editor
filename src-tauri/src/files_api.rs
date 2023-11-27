use crate::{
    error::{Error, FileError, MutexLockError, NotFoundError, PathError},
    language::Lang,
};

use derivative::Derivative;
use path_absolutize::*;
use rand::RngCore;
use serde::{Deserialize, Serialize};
use specta::Type;
use std::{
    collections::HashMap,
    ffi::OsStr,
    fs::{self},
    path::{Path, PathBuf},
    sync::{Arc, Mutex},
};

#[derive(Derivative)]
#[derivative(Debug, PartialEq, Eq, Hash, PartialOrd, Clone, Ord, Hash)]
struct OpenedFile {
    name: String,
    language: Option<Lang>,
    path: PathBuf,
    openable: bool,
    extension: String,
    source_code: Vec<u8>,
}

impl OpenedFile {
    pub fn new(path_string: impl Into<String>) -> Result<Self, Error> {
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

    fn name(&self) -> String {
        self.name.clone()
    }

    fn language(&self) -> Option<Lang> {
        self.language.to_owned()
    }

    fn path(&self) -> PathBuf {
        self.path.clone()
    }

    fn save(&self) -> Result<(), FileError> {
        Ok(fs::write(&self.path, &self.source_code)
            .map_err(|_err| FileError::SavingFileError(self.name.clone()))?)
    }

    fn update_source_code(&mut self, source_code: &Vec<u8>) {
        self.source_code = source_code.to_owned();
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

    pub fn get_source_code_from_file(&self, id: &u32) -> Result<Vec<u8>, Error> {
        let file_mutex = self._get_file(id)?;
        let file = file_mutex
            .try_lock()
            .map_err(|err| MutexLockError(err.to_string()))?;

        Ok(file.source_code.clone())
    }

    pub fn load_file(&mut self, path: impl Into<String>) -> Result<(u32, bool), Error> {
        let file = OpenedFile::new(path.into())?;
        let same_name_exist = self._search_same_name_exist(&file.name())?;
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

    pub fn clear(&mut self) {
        self.files.clear();
    }

    pub fn get_file_info(&self, id: &u32) -> Result<(String, Option<Lang>, PathBuf), Error> {
        let file_mutex = self
            ._get_file(id)
            .map_err(|err| FileError::ReadFileError(err.to_string()))?;
        let file = file_mutex
            .try_lock()
            .map_err(|err| MutexLockError(err.to_string()))?;
        Ok((file.name(), file.language(), file.path()))
    }
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

fn read_file(path: &Path) -> Result<Vec<u8>, FileError> {
    Ok(fs::read(path)
        .map_err(|_err| FileError::ReadFileError(path.to_str().unwrap().to_string()))?)
}
