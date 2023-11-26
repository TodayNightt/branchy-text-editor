use std::marker::PhantomData;

use serde::{Deserialize, Serialize};
use specta::Type;

// use crate::language::Lang;

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
pub struct Theme<T> {
    rules: Vec<Token>,
    #[serde(skip)]
    shadow_data: std::marker::PhantomData<T>,
}

macro_rules! define {
    // Define all the struct
    ($($token : ident),*)=>{
        $(
            #[derive(Debug,Clone,Serialize,Type)]
            struct $token;
        )*
    };

    // implement the default implementation for the type structs
    ($theme_type : ident ,$([$token : expr , $color:expr]),* ) => {
        impl Default for Theme<$theme_type> {
            fn default()-> Self{
                let rules = vec![
                    $((String::from($token),String::from($color))),*
                ];

                let rules = rules.into_iter().map(|(token,color)| Token::new(&token,&color)).collect();

                Self{
                    rules,
                    shadow_data: PhantomData
                }
            }
        }
    };
}

define!(Basic, Javascript, Rust, Java, Html, Css, Python, Ruby);

define!(
    Basic,
    ["keyword", "#DD91FA"],
    ["function", "#71B1FF"],
    ["type", "#DCAE80"],
    ["variable", "#9CDCFE"],
    ["number", "#b4d5ff"],
    ["string", "#72c0ff"],
    ["comment", "#93d0ff"],
    ["class", "#D1B9FF"]
);
define!(
    Javascript,
    ["keyword", "#DD91FA"],
    ["function", "#71B1FF"],
    ["type", "#DCAE80"],
    ["variable", "#9CDCFE"],
    ["number", "#b4d5ff"],
    ["string", "#72c0ff"],
    ["comment", "#93d0ff"],
    ["class", "#D1B9FF"],
    ["local", "#aaaaaa"]
);
define!(
    Rust,
    ["keyword", "#DD91FA"],
    ["function", "#71B1FF"],
    ["type", "#DCAE80"],
    ["variable", "#9CDCFE"],
    ["number", "#b4d5ff"],
    ["string", "#72c0ff"],
    ["comment", "#93d0ff"],
    ["class", "#D1B9FF"],
    ["macro", "#BEB7FF"],
    ["struct", "#5DC9B3"]
);

#[derive(Deserialize, Serialize, Debug, Clone, Type, Default)]
pub struct LanguageTheme {
    default: Theme<Basic>,
    javascript: Option<Theme<Javascript>>,
    rust: Option<Theme<Rust>>,
    java: Option<Theme<Java>>,
    html: Option<Theme<Html>>,
    css: Option<Theme<Css>>,
    python: Option<Theme<Python>>,
    ruby: Option<Theme<Ruby>>,
}

impl LanguageTheme {
    pub fn set_default(&mut self, language: String) {
        match language.as_str() {
            "javascript" => self.javascript = Some(Theme::default()),
            "rust" => self.rust = Some(Theme::default()),
            _ => {}
        };
    }
}
