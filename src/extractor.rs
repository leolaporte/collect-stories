use anyhow::{Context, Result};
use futures::stream::{self, StreamExt};
use reqwest::Client;
use std::sync::Arc;
use tokio::sync::Semaphore;

pub struct ContentExtractor {
    client: Client,
    semaphore: Arc<Semaphore>,
}

impl ContentExtractor {
    pub fn new() -> Result<Self> {
        let client = Client::builder()
            .timeout(std::time::Duration::from_secs(30))
            .user_agent("Mozilla/5.0 (compatible; PodcastBriefing/1.0)")
            .build()
            .context("Failed to create HTTP client")?;

        let semaphore = Arc::new(Semaphore::new(10));

        Ok(Self { client, semaphore })
    }

    pub async fn fetch_article_content(&self, url: &str) -> Result<Option<String>> {
        let _permit = self.semaphore.acquire().await?;

        for attempt in 0..3 {
            match self.try_fetch_article(url).await {
                Ok(content) => return Ok(content),
                Err(e) => {
                    if attempt == 2 {
                        eprintln!("Failed to fetch {}: {}", url, e);
                        return Ok(None);
                    }
                    let backoff = std::time::Duration::from_millis(500 * (2_u64.pow(attempt)));
                    tokio::time::sleep(backoff).await;
                }
            }
        }

        Ok(None)
    }

    async fn try_fetch_article(&self, url: &str) -> Result<Option<String>> {
        let response = self
            .client
            .get(url)
            .send()
            .await
            .context("Failed to send HTTP request")?;

        let status = response.status();
        if status == 401 || status == 403 || status == 404 {
            return Ok(None);
        }

        if !status.is_success() {
            anyhow::bail!("HTTP error: {}", status);
        }

        let html = response.text().await.context("Failed to read response body")?;

        let text = html2text::from_read(html.as_bytes(), 100);

        if text.trim().is_empty() || text.len() < 100 {
            return Ok(None);
        }

        Ok(Some(text))
    }

    pub async fn fetch_articles_parallel(
        &self,
        urls: Vec<String>,
    ) -> Vec<(String, Option<String>)> {
        stream::iter(urls)
            .map(|url| {
                let url_clone = url.clone();
                async move {
                    let content = self.fetch_article_content(&url).await.ok().flatten();
                    (url_clone, content)
                }
            })
            .buffer_unordered(10)
            .collect()
            .await
    }
}
