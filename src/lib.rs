pub mod discord;
pub mod template;

pub use discord::*;
use regex::Regex;
pub use template::*;

use std::collections::{HashMap, HashSet};

use wasm_bindgen::prelude::*;
use js_sys::Error;

type JsResult = Result<JsValue, Error>;
const TEMPLATE_REGEX: &str = r"\{%\s*(.*?)\s*%\}";

#[wasm_bindgen]
pub fn parse_template(template: String) -> JsResult {
    let re = Regex::new(TEMPLATE_REGEX).unwrap();
    let keywords: HashSet<_> = re.captures_iter(&template).map(|c| c[1].to_string()).collect();

    let template = EmbedTemplate {
        keywords,
        internal: template,
    };

    Ok(serde_wasm_bindgen::to_value(&template).unwrap())
}

#[wasm_bindgen]
pub fn render(template: JsValue, keywords: JsValue) -> JsResult {
    let keywords: HashMap<String, String> = serde_wasm_bindgen::from_value(keywords)
        .map_err(|e| Error::new(&format!("Failed to parse keywords: {:?}", e)))?;

    let template: EmbedTemplate = serde_wasm_bindgen::from_value(template)
        .map_err(|e| Error::new(&format!("Failed to parse template: {:?}", e)))?;

    Ok(serde_wasm_bindgen::to_value(&template.generate(keywords)?).unwrap())
}