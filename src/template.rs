use std::collections::HashMap;

use crate::discord::*;
use js_sys::Error;
use serde::{Serialize, Deserialize};
use regex::Regex;

#[derive(Serialize, Deserialize)]
pub struct EmbedTemplate {
    pub keywords: Vec<String>,
    pub internal: APIEmbed,
}

impl EmbedTemplate {
    /*
    #[deprecated]
    pub fn generate(mut self, keywords: HashMap<String, String>) -> Result<APIEmbed, Error> {
        for k in self.keywords {
            let re = Regex::new(&format!(r"{{\s*{}\s*}}", k)).unwrap();

            if let Some(v) = keywords.get(&k) {
                self.internal.title = re.replace_all(&self.internal.title, v).to_string();
                self.internal.description = re.replace_all(&self.internal.description, v).to_string();

                if let Some(url) = self.internal.url {
                    self.internal.url = Some(re.replace_all(&url, v).to_string());
                }

                if let Some(mut footer) = self.internal.footer {
                    footer.text = re.replace_all(&footer.text, v).to_string();
                    self.internal.footer = Some(footer);
                }
            } else {
                return Err(Error::new("Missing keyword"));
            }
        }

        Ok(self.internal)
    }
    */

    pub fn generate(mut self, keywords: HashMap<String, String>) -> Result<APIEmbed, Error> {
        if self.keywords.len() != keywords.len() {
            return Err(Error::new("Missing template keyword"));
        }

        let mut s = toml::to_string(&self.internal).unwrap();

        for (k, v) in keywords.iter() {
            let re = Regex::new(&format!(r#"\{{%\s*{}\s*%\}}"#, k)).unwrap();
            s = re.replace_all(&s, v).to_string();
        }

        self.internal = toml::from_str(&s)
            .map_err(|_| Error::new("Failed to parse generated template"))?;

        Ok(self.internal)
    }
}