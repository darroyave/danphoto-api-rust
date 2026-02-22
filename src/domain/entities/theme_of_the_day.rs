use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThemeOfTheDay {
    pub id: String, // MMdd
    pub name: String,
    pub url: String,
}
