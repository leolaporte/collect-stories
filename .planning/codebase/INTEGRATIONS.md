# External Integrations

**Analysis Date:** 2026-01-30

## APIs & External Services

**Raindrop.io:**
- Service: Bookmark/link collection platform
- Purpose: Fetch tagged bookmarks for podcast shows (past 7 days)
- SDK/Client: reqwest (raw HTTP)
- Auth: Bearer token via `Authorization` header
- Environment variable: `RAINDROP_API_TOKEN`
- Endpoint: `https://api.raindrop.io/rest/v1/raindrops/0`
- Implementation: `src/raindrop.rs` - `RaindropClient`
- Details:
  - Pagination: perpage=50, page-based iteration
  - Filtering: Search query with tag and date filtering
  - Response: JSON array of bookmarks with title, link, tags, created timestamp
  - Rate limiting: 500ms delay between paginated requests
  - Implementation details in: `RaindropClient::fetch_bookmarks()` (lines 38-93)

**Anthropic Claude API:**
- Service: AI text summarization and topic clustering
- Purpose: Generate 5-point summaries, cluster articles by topic
- SDK/Client: reqwest (raw HTTP)
- Auth: API key via `x-api-key` header
- Environment variable: `ANTHROPIC_API_KEY`
- Endpoint: `https://api.anthropic.com/v1/messages`
- API Version: 2023-06-01 (via `anthropic-version` header)
- Model: claude-3-5-haiku-20241022 (cost-optimized for bulk summarization)
- Implementation: `src/summarizer.rs` - `ClaudeSummarizer` and `src/clustering.rs` - `TopicClusterer`
- Details:
  - Summarization: 5 bullet points max (10,000 char truncation)
  - Concurrency: 2 concurrent requests maximum (50k token/min rate limit)
  - Timeout: 60 seconds per request
  - Retry: 5 attempts with exponential backoff (1s, 2s, 4s, 8s, 16s) + special rate limit handling (15s * attempt)
  - Cost: ~$0.001 per article with Haiku
  - Response parsing: JSON with content[0].text extraction

**Web Content Extraction:**
- Service: Generic web scraping (third-party article sites)
- Purpose: Extract plaintext article content for summarization
- Implementation: `src/extractor.rs` - `ContentExtractor`
- Client: reqwest with Mozilla User-Agent
- Details:
  - Timeout: 30 seconds
  - User-Agent: "Mozilla/5.0 (compatible; PodcastBriefing/1.0)"
  - Concurrency: 10 parallel requests (semaphore-limited)
  - Retry: 3 attempts with exponential backoff
  - Error handling: Gracefully handles 401/403/404 (returns None), other errors trigger retry
  - HTML-to-text conversion: html2text crate with 100-char line width
  - Minimum content threshold: 100 characters (shorter content rejected)

## Data Storage

**Databases:**
- None - Stateless CLI application

**File Storage:**
- Local filesystem only
- Output location: `~/Documents/` directory (cross-platform via `dirs::document_dir()`)
- Generated files:
  - HTML briefing: `{show-slug}-{date}.html` (collapsible HTML with styling)
  - CSV links file: `{show-slug}-{date}-LINKS.csv` (Google Sheets compatible format)

**Caching:**
- None - Each run fetches fresh data

## Authentication & Identity

**Auth Provider:**
- Custom token-based authentication
- Raindrop.io: Bearer token in Authorization header
- Claude API: API key in x-api-key header
- No user authentication or session management

**Token Management:**
- Environment variables (RAINDROP_API_TOKEN, ANTHROPIC_API_KEY)
- No token refresh/rotation mechanism (static tokens assumed)

## Monitoring & Observability

**Error Tracking:**
- None - Errors logged to stderr via eprintln! macro
- Failed operations continue (graceful degradation)

**Logs:**
- Console output via println! for progress updates
- Error output via eprintln! for failures
- No persistent logging or log files
- Real-time progress indicators:
  - "✓ Selected: {show}"
  - "📚 Fetching bookmarks from Raindrop.io..."
  - "✓ Found X bookmarks"
  - "🌐 Extracting article content..."
  - "🤖 Summarizing articles with Claude AI..."
  - "🔗 Clustering stories by topic..."
  - "📝 Generating briefing document..."
  - Final summary with counts

## CI/CD & Deployment

**Hosting:**
- GitHub Actions for CI/CD
- Multi-platform binary builds (Linux x86_64, macOS x86_64, macOS ARM64)
- Releases: Tag-based releases via GitHub Actions

**CI Pipeline:**
- Trigger: Push to main/master, PRs, tags starting with 'v', manual dispatch
- Build matrix:
  - ubuntu-latest → x86_64-unknown-linux-gnu
  - macos-latest → x86_64-apple-darwin
  - macos-latest → aarch64-apple-darwin (ARM64)
- Steps: Checkout, setup Rust, build release, strip binary, upload artifact
- Release automation: Creates GitHub Release with binaries when tag pushed
- Workflow file: `.github/workflows/build.yml`

## Environment Configuration

**Required env vars:**
- `RAINDROP_API_TOKEN` - Raindrop.io API token (get from app.raindrop.io/settings/integrations)
- `ANTHROPIC_API_KEY` - Claude API key (get from console.anthropic.com/settings/keys)

**Config file locations:**
- `.env` file at multiple locations (checked in order):
  1. Current working directory (development)
  2. `~/.config/podcast-briefing/.env` (recommended)
  3. `~/.env` (home directory)
  4. System environment variables

**Secrets location:**
- Environment variables (loaded via dotenvy)
- Best practice: Use `~/.config/podcast-briefing/.env` with chmod 600

## Webhooks & Callbacks

**Incoming:**
- None - CLI application, no webhook endpoints

**Outgoing:**
- None - No push notifications or external callbacks

## Show/Podcast Configuration

**Show Mappings:**
- TWiT: Tag `#twit`, slug `twit`
- MacBreak Weekly: Tag `#mbw`, slug `mbw`
- Intelligent Machines: Tag `#im`, slug `im`

**Workflow:**
1. User selects show via CLI prompt
2. Tool fetches bookmarks tagged with corresponding Raindrop tag
3. Filters by creation date (past 7 days)
4. Generates output files with show-specific naming

## Data Flow

**Complete Pipeline:**

1. **User Input** (`src/main.rs:52-69`): Prompt user to select show (TWiT, MacBreak Weekly, or Intelligent Machines)

2. **Fetch Bookmarks** (`src/raindrop.rs`): Query Raindrop.io API for tagged bookmarks from past 7 days

3. **Extract Content** (`src/extractor.rs`): Parallel fetch of article content from URLs (10 concurrent, 3 retries)

4. **Summarize** (`src/summarizer.rs`): Send article text to Claude API, get 5 bullet-point summaries (2 concurrent to avoid rate limits)

5. **Cluster** (`src/clustering.rs`): Use Claude API to group stories by company/topic (JSON-based clustering)

6. **Generate** (`src/briefing.rs`): Create HTML briefing with collapsible sections and CSV file for Google Sheets

7. **Save Output**: HTML and CSV files to `~/Documents/`

---

*Integration audit: 2026-01-30*
