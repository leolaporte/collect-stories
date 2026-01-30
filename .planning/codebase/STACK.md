# Technology Stack

**Analysis Date:** 2026-01-30

## Languages

**Primary:**
- Rust 2021 edition - CLI application, all core logic

## Runtime

**Environment:**
- Rust toolchain (stable) - Build and execution

**Package Manager:**
- Cargo - Rust package manager
- Lockfile: `Cargo.lock` (present)

## Frameworks

**Async Runtime:**
- tokio 1.x - Asynchronous task execution, full feature set
  - Used for: async main, sleep, time operations, spawning parallel tasks

**CLI & Configuration:**
- clap 4.x - Command-line argument parsing and help text (derive feature)
- dotenvy 0.15 - Environment variable loading from `.env` files

**HTTP & Networking:**
- reqwest 0.13 - HTTP client with JSON serialization support
  - Features: JSON content-type, connection pooling, timeout handling
  - Used for: Raindrop.io API calls, article fetching, Claude API calls

**Serialization:**
- serde 1.0 - Serialization framework with derive macros
- serde_json 1.0 - JSON parsing and generation

**Error Handling:**
- anyhow 1.0 - Flexible error handling with context propagation

**Data & Time:**
- chrono 0.4 - Date/time parsing and formatting (ISO 8601 handling)
- futures 0.3 - Stream processing with buffer_unordered for parallel operations

**Text Processing:**
- html2text 0.12 - HTML-to-plaintext conversion for article extraction
- urlencoding 2.1 - URL encoding for search queries

**File System:**
- dirs 5.0 - Cross-platform directory resolution (Documents, config, home)

## Key Dependencies

**Critical:**
- tokio - Enables parallel HTTP requests, required for performance
- reqwest - HTTP client for all external API integrations
- serde/serde_json - Required for API request/response serialization
- anyhow - Error handling with context, used throughout
- chrono - Date filtering for bookmarks (past 7 days)
- html2text - Article content extraction, core workflow step

**Infrastructure:**
- futures - Stream operations for parallel processing (buffer_unordered)
- clap - CLI interface for show selection
- dotenvy - Config file loading from multiple locations
- dirs - Cross-platform path resolution

## Configuration

**Environment:**
- Configuration via environment variables or `.env` files
- `.env` file locations (checked in order):
  1. Current directory (development)
  2. `~/.config/podcast-briefing/.env` (recommended)
  3. `~/.env` (home directory)
  4. System environment variables

**Required environment variables:**
- `RAINDROP_API_TOKEN` - API token for Raindrop.io bookmarks
- `ANTHROPIC_API_KEY` - API key for Claude AI summaries

**Build:**
- Cargo.toml defines dependencies and workspace
- No special build configuration (standard Rust build)

## Platform Requirements

**Development:**
- Rust stable toolchain (install from rustup.rs)
- Standard build: `cargo build --release`

**Production:**
- Linux x86_64 (ubuntu-latest CI)
- macOS x86_64 (macos-latest CI)
- macOS ARM64 (aarch64-apple-darwin CI)
- Binaries built and distributed via GitHub Actions

## API Integration Details

**HTTP Client Configuration:**
- Default timeout: 30 seconds (general HTTP), 60 seconds (AI API)
- User-Agent: "Mozilla/5.0 (compatible; PodcastBriefing/1.0)" for article fetching
- Connection pooling via reqwest client reuse

**Concurrency Control:**
- Article fetching: Semaphore-limited to 10 concurrent requests
- Claude summarization: Semaphore-limited to 2 concurrent requests (50k token/min rate limit)
- Raindrop.io: Sequential pagination with 500ms delays between pages

**Retry Strategy:**
- Article fetching: 3 retry attempts with exponential backoff (500ms, 1000ms, 2000ms)
- Claude API: 5 retry attempts with special handling for rate limits (15s * attempt backoff)
- Raindrop.io: Single request per page, no retries

---

*Stack analysis: 2026-01-30*
