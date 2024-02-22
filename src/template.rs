use std::collections::{HashMap, HashSet};

use crate::discord::*;
use js_sys::Error;
use serde::{Serialize, Deserialize};
use regex::Regex;

#[derive(Serialize, Deserialize)]
pub struct EmbedTemplate {
    pub keywords: HashSet<String>,
    pub internal: String,
}

impl EmbedTemplate {
    pub fn generate(mut self, keywords: HashMap<String, String>) -> Result<APIEmbed, Error> {
        if self.keywords.len() != keywords.len() {
            return Err(Error::new(&format!("Keyword mismatch: expected {} keywords, got {}", self.keywords.len(), keywords.len())));
        }

        for (k, v) in keywords.iter() {
            let re = Regex::new(&format!(r#"\{{%\s*{}\s*%\}}"#, k)).unwrap();
            self.internal = re.replace_all(&self.internal, v).to_string();
        }

        let embed = toml::from_str(&self.internal)
            .map_err(|e| Error::new(&format!("Failed to parse generated template: {:?}", e)))?;

        Ok(embed)
    }
}