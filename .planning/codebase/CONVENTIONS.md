# Coding Conventions

**Analysis Date:** 2026-01-30

## Naming Patterns

**Files:**
- Snake case: `raindrop.rs`, `summarizer.rs`, `extractor.rs`, `clustering.rs`
- Matches Rust convention for module/crate names
- Single responsibility per file (e.g., `raindrop.rs` only contains Raindrop-related code)

**Functions:**
- Snake case for all functions: `fetch_bookmarks()`, `try_fetch_article()`, `parse_bullet_points()`
- Public API functions use verb-first pattern: `fetch_`, `summarize_`, `cluster_`, `generate_`
- Private helper functions use `try_` prefix for fallible operations: `try_fetch_article()`, `try_summarize()`, `try_load_dotenv()`

**Variables:**
- Snake case throughout: `all_bookmarks`, `content_map`, `successful_summaries`, `semaphore`
- Immutable by default; `let` bindings never reassigned
- Loop counters: `page`, `attempt`, `idx` (short names acceptable for loop scope)

**Types:**
- PascalCase for structs: `Bookmark`, `RaindropClient`, `ContentExtractor`, `ClaudeSummarizer`, `Topic`, `Story`
- PascalCase for enums: `Show`, `Summary`
- Enums use descriptive variants: `Show::TWiT`, `Summary::Success(Vec<String>)`, `Summary::Insufficient`, `Summary::Failed(String)`
- Private internal request/response types use PascalCase: `ClaudeRequest`, `Message`, `ClaudeResponse`, `Content`

## Code Style

**Formatting:**
- No explicit linting/formatting config file; follows Rust standard (2-space indentation visible in code)
- Module declarations at top of `main.rs`: `mod briefing; mod clustering; mod config;`
- All source files use standard Rust style with no custom formatting rules detected

**Linting:**
- README mentions `cargo clippy` as development best practice
- No `.clippy.toml` or custom clippy rules configured
- Standard Rust edition 2021 conventions apply

## Import Organization

**Order:**
1. Standard library imports: `use std::collections::HashMap;`, `use std::fs;`
2. External crate imports: `use anyhow::{Context, Result};`, `use reqwest::Client;`, `use serde::{Deserialize, Serialize};`
3. Module-local imports: `use crate::summarizer::Summary;`, `use crate::clustering::Story;`

**Path Aliases:**
- No path aliases (e.g., no `#[path = "..."]`) used
- Crates imported fully: `use anyhow::{Context, Result};` (standard error handling)
- Module hierarchy simple: `mod briefing; mod config;` with direct use statements

**Example from `main.rs`:**
```rust
use anyhow::{Context, Result};
use briefing::BriefingGenerator;
use chrono::{Duration, Utc};
use clustering::{Story, TopicClusterer};
use config::Config;
use extractor::ContentExtractor;
use raindrop::RaindropClient;
use std::collections::HashMap;
use std::io::{self, Write};
use summarizer::{ClaudeSummarizer, Summary};
```

## Error Handling

**Patterns:**
- Comprehensive error handling using `anyhow::Result<T>` throughout
- All public functions return `Result<T>` or `Result<()>` (e.g., `pub fn new(api_token: String) -> Result<Self>`)
- Errors include context via `.context("Human-readable message")`
- `anyhow::bail!()` for explicit error generation with formatted messages

**From `config.rs`:**
```rust
let raindrop_api_token = env::var("RAINDROP_API_TOKEN")
    .context("RAINDROP_API_TOKEN not found. Set it as an environment variable...")?;
```

**From `raindrop.rs`:**
```rust
if !status.is_success() {
    let error_text = response.text().await.unwrap_or_else(|_| String::from("unknown error"));
    anyhow::bail!(
        "Raindrop API returned error: {} - {}",
        status,
        error_text
    );
}
```

## Logging

**Framework:** Console output via `println!()` and `eprintln!()`

**Patterns:**
- User-facing output: `println!()` with emoji indicators: `println!("✓ Found {} bookmarks", bookmarks.len());`
- Error output: `eprintln!()` for stderr: `eprintln!("Failed to fetch {}: {}", url, e);`
- Progress indicators: `println!("\n📚 Fetching bookmarks...");` (section headers)
- No structured logging framework (no `tracing`, `log`, or `slog`)
- No console.log equivalents; all logging is direct to stdout/stderr

**Example from `main.rs`:**
```rust
println!("\n✓ Selected: {}", show.name());
println!("\n📚 Fetching bookmarks from Raindrop.io...");
println!("✓ Found {} bookmarks", bookmarks.len());
eprintln!("Failed to fetch {}: {}", url, e);
```

## Comments

**When to Comment:**
- Algorithm explanation in complex sections: `// buffer_unordered returns in random order`
- Fallback reasoning: `// If none found, that's okay - environment variables might be set system-wide`
- API implementation details: `// Truncate content to 10000 chars, respecting UTF-8 boundaries`

**JSDoc/TSDoc:**
- Not used; Rust uses doc comments instead (none observed in current codebase)
- Future enhancement: Consider adding `///` doc comments to public functions and types

**Example from `main.rs`:**
```rust
// Create a map of URL -> Content for correct pairing (buffer_unordered returns in random order)
let content_map: HashMap<String, String> = content_results
    .into_iter()
    .filter_map(|(url, content)| content.map(|c| (url, c)))
    .collect();
```

## Function Design

**Size:**
- Small, focused functions: 10-30 lines typical
- Longest observed: `summarize_article()` at ~40 lines (including retry logic)
- Largest file: `briefing.rs` at 178 lines (single responsibility: HTML/CSV generation)

**Parameters:**
- Minimal parameters; use `self` for stateful operations
- Client structs hold HTTP clients and API keys: `RaindropClient { client: Client, api_token: String }`
- Functions take references where appropriate: `fetch_bookmarks(&self, tag: &str, since: DateTime<Utc>)`

**Return Values:**
- Always explicit: `Result<T>` or `Option<T>`
- Public API returns: `Result<String>`, `Result<Vec<Topic>>`, `Result<PathBuf>`
- Internal parallelization returns: `Vec<(String, Option<String>)>` (URL paired with optional content)

**Example async pattern from `summarizer.rs`:**
```rust
pub async fn summarize_article(&self, content: &str) -> Result<Summary> {
    let _permit = self.semaphore.acquire().await?;
    // ... retry logic with exponential backoff ...
}
```

## Module Design

**Exports:**
- Each module exports main public types: `pub struct`, `pub enum`, `pub impl`
- Private internal structures marked implicitly (no `pub`): `struct ClaudeRequest`, `struct Message`
- Modules control API surface explicitly

**Barrel Files:**
- No barrel files (`mod.rs`) used; flat structure with `main.rs` declaring modules
- Module structure: `main.rs` → declares mods → each file is standalone module

**Example module declaration from `main.rs`:**
```rust
mod briefing;
mod clustering;
mod config;
mod extractor;
mod raindrop;
mod summarizer;
```

Each module is imported and used directly, e.g., `use briefing::BriefingGenerator;`

## Async/Concurrency Patterns

**Async Framework:** Tokio with `#[tokio::main]` macro

**Parallelization Patterns:**
- `futures::stream::StreamExt::buffer_unordered()` for parallel operations with concurrency limit
- `tokio::sync::Semaphore` for rate limiting (e.g., 10 permits for content extraction, 2 for API calls)

**Example from `extractor.rs`:**
```rust
pub async fn fetch_articles_parallel(&self, urls: Vec<String>) -> Vec<(String, Option<String>)> {
    stream::iter(urls)
        .map(|url| { ... })
        .buffer_unordered(10)  // Limit to 10 concurrent requests
        .collect()
        .await
}
```

**Retry Logic:**
- Exponential backoff with `std::time::Duration::from_millis()` and `tokio::time::sleep()`
- Rate limit handling with longer backoff (15s * attempt)
- Maximum 3-5 retries depending on operation

**Example from `summarizer.rs`:**
```rust
for attempt in 0..5 {
    match self.try_summarize(content).await {
        Ok(summary) => return Ok(summary),
        Err(e) => {
            let backoff = if is_rate_limit {
                std::time::Duration::from_secs(15 * (attempt + 1) as u64)
            } else {
                std::time::Duration::from_millis(1000 * (2_u64.pow(attempt as u32)))
            };
            tokio::time::sleep(backoff).await;
        }
    }
}
```

---

*Convention analysis: 2026-01-30*
