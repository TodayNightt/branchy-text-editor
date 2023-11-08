use serde::{Deserialize, Serialize};
use specta::Type;
use std::error::Error;
use std::fmt::Display;
use std::fs::File;
use std::io::BufReader;
use std::sync::Mutex;

#[derive(Deserialize, Serialize, Debug, Clone, Default, Type)]

pub struct Token {
    token: String,
    foreground: String,
}

impl Token {
    fn new(token: &str, foreground: &str) -> Self {
        Self {
            token: token.to_string(),
            foreground: foreground.to_string(),
        }
    }
}

#[derive(Deserialize, Serialize, Debug, Clone, Type)]
pub struct Theme {
    rules: Vec<Token>, // keyword: String,
                       // function: String,
                       // types: String,
                       // variable: String,
                       // number: String,
                       // string: String,
}

impl Default for Theme {
    fn default() -> Self {
        let mut rules = vec![];
        rules.push(Token::new("keyword", "#DD91FA"));
        rules.push(Token::new("function", "#71B1FF"));
        rules.push(Token::new("type", "#DCAE80"));
        rules.push(Token::new("variable", "#9CDCFE"));
        rules.push(Token::new("number", "#b4d5ff"));
        rules.push(Token::new("string", "#72c0ff"));
        rules.push(Token::new("comment", "#93d0ff"));
        rules.push(Token::new("class", "#D1B9FF"));
        Self { rules }
    }
}

#[derive(Deserialize, Serialize, Debug, Default, Clone, Type)]
pub struct EditorTheme {
    background: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Type)]
pub struct LanguageTheme {
    default: Theme,
    javascript: Option<Theme>,
    rust: Option<Theme>,
    java: Option<Theme>,
    html: Option<Theme>,
    css: Option<Theme>,
    python: Option<Theme>,
    ruby: Option<Theme>,
}

impl Default for LanguageTheme {
    fn default() -> Self {
        Self {
            default: Theme::default(),
            javascript: None,
            rust: None,
            java: None,
            html: None,
            css: None,
            python: None,
            ruby: None,
        }
    }
}

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct ThemeConfig {
    pub language: Mutex<LanguageTheme>,
    pub editor: Mutex<EditorTheme>,
}

impl ThemeConfig {
    fn load(path: &str) -> Result<Self, Box<dyn Error>> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);

        // Read the JSON contents of the file as an instance of `User`.
        Ok(serde_json::from_reader(reader)?)
    }

    fn save(&self) {
        let json = serde_json::to_string(self).expect("Cannot serialize");
        std::fs::write("../../test_home/config.json", json).expect("Cannot save config file");
    }
}

impl Display for ThemeConfig {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}", serde_json::to_string_pretty(&self).unwrap())
    }
}


