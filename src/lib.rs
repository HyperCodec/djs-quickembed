pub mod discord;
pub mod template;

pub use discord::*;
use regex::Regex;
pub use template::*;

use std::collections::HashMap;

use wasm_bindgen::prelude::*;
use js_sys::Error;

type JsResult = Result<JsValue, Error>;
const TEMPLATE_REGEX: &str = r"\{%\s*(.*?)\s*%\}";

#[wasm_bindgen]
pub fn parse_template(template: &str) -> JsResult {
    let regex = Regex::new(TEMPLATE_REGEX).unwrap();
    let keywords: Vec<_> = regex.find_iter(template).map(|m| m.as_str().trim().to_string()).collect();

    let internal: APIEmbed = toml::from_str(template)
        .map_err(|_| Error::new("Failed to parse template"))?;

    let template = EmbedTemplate {
        keywords,
        internal,
    };

    Ok(serde_wasm_bindgen::to_value(&template).unwrap())
}

#[wasm_bindgen]
pub fn generate_embed(template: JsValue, keywords: JsValue) -> JsResult {
    let keywords: HashMap<String, String> = serde_wasm_bindgen::from_value(keywords)
        .map_err(|_| Error::new("Failed to parse keywords"))?;

    let template: EmbedTemplate = serde_wasm_bindgen::from_value(template)
        .map_err(|_| Error::new("Failed to parse generate"))?;

    Ok(serde_wasm_bindgen::to_value(&template.generate(keywords)?).unwrap())
}