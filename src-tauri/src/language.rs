use serde::{Deserialize, Serialize};
use specta::Type;
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
