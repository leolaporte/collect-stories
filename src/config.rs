use anyhow::{Context, Result};
use std::env;

#[derive(Debug, Clone)]
pub struct Config {
    pub raindrop_api_token: String,
    pub anthropic_api_key: String,
}

impl Config {
    pub fn from_env() -> Result<Self> {
        // Try to load .env from multiple locations
        Self::try_load_dotenv();

        let raindrop_api_token = env::var("RAINDROP_API_TOKEN")
            .context("RAINDROP_API_TOKEN not found. Set it as an environment variable or create ~/.config/podcast-briefing/.env")?;

        let anthropic_api_key = env::var("ANTHROPIC_API_KEY")
            .context("ANTHROPIC_API_KEY not found. Set it as an environment variable or create ~/.config/podcast-briefing/.env")?;

        Ok(Self {
            raindrop_api_token,
            anthropic_api_key,
        })
    }

    fn try_load_dotenv() {
        // Try locations in order of preference:

        // 1. Current directory (for development)
        if dotenvy::dotenv().is_ok() {
            return;
        }

        // 2. ~/.config/podcast-briefing/.env (standard config location)
        if let Some(config_dir) = dirs::config_dir() {
            let config_path = config_dir.join("podcast-briefing").join(".env");
            if config_path.exists() {
                if dotenvy::from_path(&config_path).is_ok() {
                    return;
                }
            }
        }

        // 3. ~/.env (home directory)
        if let Some(home_dir) = dirs::home_dir() {
            let home_path = home_dir.join(".env");
            if home_path.exists() {
                if dotenvy::from_path(&home_path).is_ok() {
                    return;
                }
            }
        }

        // If none found, that's okay - environment variables might be set system-wide
    }
}
