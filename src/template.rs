use std::collections::{HashMap, HashSet};

use crate::discord::*;
use js_sys::Error;
use serde::{Serialize, Deserialize};
use regex::Regex;

#[derive(Serialize, Deserialize)]
pub struct EmbedTemplate {
    pub keywords: HashSet<String>,
    pub internal: APIEmbed,
}

impl EmbedTemplate {
    pub fn generate(mut self, keywords: HashMap<String, String>) -> Result<APIEmbed, Error> {
        if self.keywords.len() != keywords.len() {
            return Err(Error::new(&format!("Keyword mismatch: expected {} keywords, got {}", self.keywords.len(), keywords.len())));
        }

        // maybe not as fast (although still takes microseconds) but it works and is probably the least lines of code
        let mut s = toml::to_string(&self.internal).unwrap();

        for (k, v) in keywords.iter() {
            let re = Regex::new(&format!(r#"\{{%\s*{}\s*%\}}"#, k)).unwrap();
            s = re.replace_all(&s, v).to_string();
        }

        self.internal = toml::from_str(&s)
            .map_err(|e| Error::new(&format!("Failed to parse generated template: {:?}", e)))?;

        Ok(self.internal)
    }
}