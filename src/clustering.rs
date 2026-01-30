use anyhow::{Context, Result};
use reqwest::Client;
use serde::{Deserialize, Serialize};

use crate::summarizer::Summary;

#[derive(Debug, Clone)]
pub struct Story {
    pub title: String,
    pub url: String,
    pub created: String,
    pub summary: Summary,
}

#[derive(Debug, Clone)]
pub struct Topic {
    pub title: String,
    pub stories: Vec<Story>,
}

#[derive(Serialize)]
struct ClaudeRequest {
    model: String,
    max_tokens: u32,
    messages: Vec<Message>,
}

#[derive(Serialize, Deserialize)]
struct Message {
    role: String,
    content: String,
}

#[derive(Deserialize)]
struct ClaudeResponse {
    content: Vec<Content>,
}

#[derive(Deserialize)]
struct Content {
    text: String,
}

#[derive(Deserialize)]
struct ClusteringResult {
    topics: Vec<TopicCluster>,
}

#[derive(Deserialize)]
struct TopicCluster {
    title: String,
    article_indices: Vec<usize>,
}

pub struct TopicClusterer {
    client: Client,
    api_key: String,
}

impl TopicClusterer {
    pub fn new(api_key: String) -> Result<Self> {
        let client = Client::builder()
            .timeout(std::time::Duration::from_secs(60))
            .build()
            .context("Failed to create HTTP client")?;

        Ok(Self { client, api_key })
    }

    pub async fn cluster_stories(&self, stories: Vec<Story>) -> Result<Vec<Topic>> {
        if stories.is_empty() {
            return Ok(Vec::new());
        }

        if stories.len() == 1 {
            return Ok(vec![Topic {
                title: "News".to_string(),
                stories,
            }]);
        }

        match self.try_cluster_with_ai(&stories).await {
            Ok(topics) => Ok(topics),
            Err(e) => {
                eprintln!("Clustering failed: {}, using chronological fallback", e);
                Ok(self.fallback_chronological(stories))
            }
        }
    }

    async fn try_cluster_with_ai(&self, stories: &[Story]) -> Result<Vec<Topic>> {
        let articles_text = stories
            .iter()
            .enumerate()
            .map(|(idx, story)| {
                let first_point = match &story.summary {
                    Summary::Success(points) => points.first().map(|s| s.as_str()).unwrap_or(""),
                    _ => "",
                };
                format!("{}: {} - {}", idx, story.title, first_point)
            })
            .collect::<Vec<_>>()
            .join("\n");

        let prompt = format!(
            r#"You are analyzing a list of news articles for a tech podcast briefing.

GROUPING RULES (in priority order):
1. PRIMARY: If an article is primarily about a specific company (Google, Apple, Microsoft, Tesla, Meta, Amazon, etc.), use the company name as the topic title
2. Group all articles about the same company together under that company's name
3. For articles not primarily about a single company, use a descriptive topic (e.g., "AI Development", "Privacy & Security", "Industry News")
4. Use concise topic names (1-3 words preferred, company names exactly as they are commonly known)

Articles:
{}

Format your response as JSON:
{{
  "topics": [
    {{
      "title": "Apple",
      "article_indices": [0, 3, 7]
    }},
    {{
      "title": "Google",
      "article_indices": [1, 5]
    }},
    {{
      "title": "AI Development",
      "article_indices": [2, 4, 6]
    }}
  ]
}}

Important: Every article index from 0 to {} must appear in exactly one topic."#,
            articles_text,
            stories.len() - 1
        );

        let request = ClaudeRequest {
            model: "claude-3-5-haiku-20241022".to_string(),
            max_tokens: 2048,
            messages: vec![Message {
                role: "user".to_string(),
                content: prompt,
            }],
        };

        let response = self
            .client
            .post("https://api.anthropic.com/v1/messages")
            .header("x-api-key", &self.api_key)
            .header("anthropic-version", "2023-06-01")
            .header("content-type", "application/json")
            .json(&request)
            .send()
            .await
            .context("Failed to send request to Claude API")?;

        if !response.status().is_success() {
            let error_text = response.text().await.unwrap_or_else(|_| String::from("unknown error"));
            anyhow::bail!("Claude API error: {}", error_text);
        }

        let claude_response = response
            .json::<ClaudeResponse>()
            .await
            .context("Failed to parse Claude API response")?;

        let response_text = claude_response
            .content
            .first()
            .map(|c| c.text.as_str())
            .unwrap_or("");

        let json_text = if let Some(start) = response_text.find('{') {
            if let Some(end) = response_text.rfind('}') {
                &response_text[start..=end]
            } else {
                response_text
            }
        } else {
            response_text
        };

        let clustering_result: ClusteringResult = serde_json::from_str(json_text)
            .context("Failed to parse clustering JSON response")?;

        let mut topics = Vec::new();
        for cluster in clustering_result.topics {
            let mut topic_stories = Vec::new();
            for &idx in &cluster.article_indices {
                if idx < stories.len() {
                    topic_stories.push(stories[idx].clone());
                }
            }
            if !topic_stories.is_empty() {
                topics.push(Topic {
                    title: cluster.title,
                    stories: topic_stories,
                });
            }
        }

        if topics.is_empty() {
            anyhow::bail!("No topics generated from clustering");
        }

        Ok(topics)
    }

    fn fallback_chronological(&self, stories: Vec<Story>) -> Vec<Topic> {
        vec![Topic {
            title: "News Stories".to_string(),
            stories,
        }]
    }
}
