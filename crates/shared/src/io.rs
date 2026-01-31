use anyhow::{Context, Result};
use chrono::DateTime;
use std::fs;
use std::path::PathBuf;

use crate::models::BriefingData;

/// Get the default directory for storing story files
pub fn get_default_stories_dir() -> Result<PathBuf> {
    let data_dir = dirs::data_local_dir()
        .context("Could not determine local data directory")?
        .join("podcast-briefing")
        .join("stories");

    fs::create_dir_all(&data_dir).context("Failed to create stories directory")?;

    Ok(data_dir)
}

/// Save story data to a JSON file
pub fn save_stories(data: &BriefingData, filename: &str) -> Result<PathBuf> {
    let stories_dir = get_default_stories_dir()?;
    let filepath = stories_dir.join(filename);

    let json = serde_json::to_string_pretty(data).context("Failed to serialize briefing data")?;

    fs::write(&filepath, json).context("Failed to write story file")?;

    Ok(filepath)
}

/// Load story data from a JSON file
pub fn load_stories(filepath: &PathBuf) -> Result<BriefingData> {
    // Check if file exists
    if !filepath.exists() {
        anyhow::bail!("Story file not found: {}", filepath.display());
    }

    let content = fs::read_to_string(filepath)
        .with_context(|| format!("Failed to read story file: {}", filepath.display()))?;

    // Try to parse JSON with helpful error message
    let data: BriefingData = serde_json::from_str(&content)
        .with_context(|| {
            format!(
                "Failed to parse story JSON from {}. The file may be corrupted or not a valid story file.",
                filepath.display()
            )
        })?;

    // Validate version
    if data.version != "1.0" {
        anyhow::bail!(
            "Unsupported story file version: {}. Expected 1.0. Please regenerate the story file with collect-stories.",
            data.version
        );
    }

    // Validate required fields
    if data.topics.is_empty() {
        anyhow::bail!(
            "Story file {} contains no topics. The file may be incomplete.",
            filepath.display()
        );
    }

    Ok(data)
}

/// List all available story files with metadata
pub fn list_story_files() -> Result<Vec<(PathBuf, BriefingData)>> {
    let stories_dir = get_default_stories_dir()?;

    let mut files = Vec::new();

    if stories_dir.exists() {
        for entry in fs::read_dir(&stories_dir).context("Failed to read stories directory")? {
            let entry = entry?;
            let path = entry.path();

            if path.extension().and_then(|s| s.to_str()) == Some("json") {
                match load_stories(&path) {
                    Ok(data) => {
                        files.push((path, data));
                    }
                    Err(e) => {
                        eprintln!("Warning: Could not load {}: {}", path.display(), e);
                    }
                }
            }
        }
    }

    // Sort by creation date (newest first)
    files.sort_by(|a, b| {
        let time_a = DateTime::parse_from_rfc3339(&a.1.created_at).ok();
        let time_b = DateTime::parse_from_rfc3339(&b.1.created_at).ok();
        time_b.cmp(&time_a)
    });

    Ok(files)
}
