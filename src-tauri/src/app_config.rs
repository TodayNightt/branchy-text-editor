use std::{
    fmt::Display,
    fs::File,
    io::{BufReader, BufWriter},
    path::{Path, PathBuf},
    sync::{Arc, Mutex},
};

use serde::{Deserialize, Serialize};
use specta::Type;

use crate::{
    error::{Error, PathError, SerdeError},
    treesitter_backend::theme::LanguageTheme,
};

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct LastSection;

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct EditorConfig {
    theme: Arc<Mutex<ThemeConfig>>,
    #[serde(skip)]
    config_path: PathBuf,
    // last_section: Option<Mutex<LastSection>>,
}

impl EditorConfig {
    fn new(path: PathBuf) -> Self {
        Self {
            config_path: path,
            theme: Arc::new(Mutex::new(ThemeConfig::default())),
        }
    }
    pub fn load(path: Option<PathBuf>) -> Result<Self, Error> {
        let mut path = path.clone().ok_or(PathError::PathNotFoundError)?;
        path.push(Path::new("config.json"));
        let file = File::open(path.clone());
        if let Ok(file) = file {
            let reader = BufReader::new(file);

            let config = serde_json::from_reader(reader)
                .map_err(|err| SerdeError::DerializeError(err.to_string()))?;
            Ok(config)
        } else {
            let editor_config = Self::new(path.clone());
            editor_config.save_config()?;
            Ok(editor_config)
        }
    }
    fn save_config(&self) -> Result<(), Error> {
        let file = File::create(self.config_path.clone())?;
        let writer = BufWriter::new(file);
        Ok(serde_json::to_writer_pretty(writer, self)
            .map_err(|err| SerdeError::SerializeError(err.to_string()))?)
    }

    pub fn theme(&self) -> Arc<Mutex<ThemeConfig>> {
        self.theme.clone()
    }
}

#[derive(Deserialize, Serialize, Debug, Default, Clone, Type)]
pub struct EditorTheme {
    background: String,
}

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct ThemeConfig {
    language: Arc<Mutex<LanguageTheme>>,
    editor: Arc<Mutex<EditorTheme>>,
}

impl ThemeConfig {
    pub fn language_theme(&self) -> Arc<Mutex<LanguageTheme>> {
        self.language.clone()
    }
    pub fn editor_theme(&self) -> Arc<Mutex<EditorTheme>> {
        self.editor.clone()
    }
}

impl Display for ThemeConfig {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}", serde_json::to_string_pretty(&self).unwrap())
    }
}
