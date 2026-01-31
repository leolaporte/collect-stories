use serde::{Deserialize, Serialize};

use crate::clustering::Topic;

/// Metadata about the show
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShowInfo {
    pub name: String,
    pub slug: String,
    pub tag: String,
}

impl ShowInfo {
    pub fn new(name: impl Into<String>, slug: impl Into<String>, tag: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            slug: slug.into(),
            tag: tag.into(),
        }
    }
}

/// Complete briefing data for serialization
#[derive(Debug, Serialize, Deserialize)]
pub struct BriefingData {
    pub version: String,
    pub created_at: String,
    pub show: ShowInfo,
    pub topics: Vec<Topic>,
}

impl BriefingData {
    pub fn new(show: ShowInfo, topics: Vec<Topic>) -> Self {
        Self {
            version: "1.0".to_string(),
            created_at: chrono::Utc::now().to_rfc3339(),
            show,
            topics,
        }
    }
}
