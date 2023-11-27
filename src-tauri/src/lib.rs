use std::{path::PathBuf, sync::Mutex};

use crate::files_api::FileManager;
use app_config::EditorConfig;
use error::Error;
use treesitter_backend::{parser::ParserHelper, query::QueryManager};

pub mod app_config;
pub mod backend_api;
pub mod error;
pub mod files_api;
pub mod language;
pub mod treesitter_backend;

#[derive(Default)]
pub struct StateManager {
    pub editor_config: Mutex<EditorConfig>,
    pub file_manager: Mutex<FileManager>,
    parser_helper: Mutex<ParserHelper>,
    query_iter: QueryManager,
}

impl StateManager {
    pub fn new(path: Option<PathBuf>) -> Result<Self, Error> {
        Ok(Self {
            file_manager: Mutex::new(FileManager::new()),
            editor_config: Mutex::new(EditorConfig::load(path)?),
            parser_helper: Mutex::new(ParserHelper::default()),
            query_iter: QueryManager::default(),
        })
    }
}
