# Architecture

**Analysis Date:** 2026-01-30

## Pattern Overview

**Overall:** Pipeline/Sequential Processing with Async I/O

**Key Characteristics:**
- Linear data flow: Fetch → Extract → Summarize → Cluster → Generate Output
- Async/await for concurrent operations at each stage
- Rate limiting and semaphore-based concurrency control
- Graceful fallback handling for failed operations
- Modular stage separation with clear responsibilities

## Layers

**Entry Point & Orchestration:**
- Purpose: Main flow control, user interaction, stage coordination
- Location: `src/main.rs`
- Contains: Show selection, stage invocation, progress logging
- Depends on: All other modules
- Used by: CLI invocation

**External Service Integration:**
- Purpose: API clients for third-party services
- Location: `src/raindrop.rs`, `src/summarizer.rs`
- Contains: HTTP clients, API request/response handling
- Depends on: reqwest, serde, anyhow
- Used by: Main orchestration, feature modules

**Content Extraction:**
- Purpose: Fetch and parse article HTML to text
- Location: `src/extractor.rs`
- Contains: HTML→text conversion, retry logic, parallel fetching
- Depends on: reqwest, html2text, tokio semaphores
- Used by: Main orchestration

**Data Processing:**
- Purpose: Transform and group data
- Location: `src/clustering.rs`
- Contains: Story grouping by topic via AI clustering, fallback strategies
- Depends on: Summarizer API, clustering algorithm
- Used by: Main orchestration

**Output Generation:**
- Purpose: Format and save results
- Location: `src/briefing.rs`
- Contains: HTML generation, CSV generation, file I/O
- Depends on: chrono, dirs, file system
- Used by: Main orchestration

**Configuration:**
- Purpose: Environment variable management
- Location: `src/config.rs`
- Contains: Config loading from multiple .env locations
- Depends on: dotenvy, dirs, env crate
- Used by: Main orchestration

## Data Flow

**Primary Pipeline:**

1. **User Interaction** (`main.rs`): Prompt for show selection
2. **Fetch Bookmarks** (`raindrop.rs`): Query Raindrop.io API with show-specific tag, paginated
3. **Extract Content** (`extractor.rs`): Parallel HTML fetch and text extraction with retries
4. **Summarize Articles** (`summarizer.rs`): Parallel Claude Haiku API calls for 5-bullet summaries
5. **Cluster Stories** (`clustering.rs`): Claude API to group articles by topic/company
6. **Generate Output** (`briefing.rs`): Create HTML briefing and CSV links file
7. **Write Files** (`briefing.rs`): Save to ~/Documents/ with dated filenames

**State Management:**
- Immutable structures: `Bookmark`, `Story`, `Topic`, `Summary`
- State flows as data through function parameters
- No global state; pure functional transformations
- Maps used in main to deduplicate results across async boundaries (content_map, summary_map)

## Key Abstractions

**Bookmark:**
- Purpose: Represents a Raindrop.io bookmark with metadata
- Examples: `src/raindrop.rs`
- Pattern: Serde deserializable struct matching Raindrop API response

**Story:**
- Purpose: Enriched bookmark with extracted content summary
- Examples: `src/clustering.rs`
- Pattern: Immutable struct combining bookmark data with summarization result

**Topic:**
- Purpose: Collection of related stories for briefing output
- Examples: `src/clustering.rs`
- Pattern: Container struct with title and story list

**Summary:**
- Purpose: Result enum for article summarization outcomes
- Examples: `src/summarizer.rs`, variants: Success(Vec<String>), Insufficient, Failed(String)
- Pattern: Algebraic data type allowing graceful degradation

**Show:**
- Purpose: Enum mapping user selection to tag, name, slug
- Examples: `src/main.rs`
- Pattern: Zero-cost abstraction for compile-time show definitions

## Entry Points

**Main CLI:**
- Location: `src/main.rs:main()`
- Triggers: `cargo run` or `podcast-briefing` binary invocation
- Responsibilities: Prompt user, coordinate pipeline stages, display progress and results

**Config Initialization:**
- Location: `src/config.rs:Config::from_env()`
- Triggers: Called first in main()
- Responsibilities: Load API tokens from environment/dotenv files

**Show Selection:**
- Location: `src/main.rs:prompt_show_selection()`
- Triggers: After config loaded
- Responsibilities: Interactive stdin prompt for show choice

## Error Handling

**Strategy:** Recovery-first with graceful degradation

**Patterns:**
- **Retry with Exponential Backoff**: `src/extractor.rs`, `src/summarizer.rs` (2-3 retries for extraction, 5 retries for API calls)
- **Fallback Strategies**:
  - Clustering: If AI fails, use chronological grouping (`src/clustering.rs` line 85-87)
  - Summarization: Return `Summary::Insufficient` or `Summary::Failed` without crashing
  - Content Extraction: Return `None` for unreachable URLs; continue with remaining articles
- **Rate Limiting**:
  - Semaphore-based concurrency (10 concurrent extractors, 2 concurrent summarizers)
  - Explicit delays between successful requests
  - Longer backoff for rate limit errors (15s * attempt)
- **Error Context**: `anyhow::Context` for all error paths to provide operation context

## Cross-Cutting Concerns

**Logging:** Informational println! statements with emoji status indicators
- Shows progress at each stage
- Reports counts and failures
- No structured logging; simple stderr/stdout

**Validation:** Input validation handled at API response parsing via serde
- HTTP status code checks in all API clients
- Content length checks in extraction
- Bullet point parsing validation in summarization

**Authentication:** API token handling via environment variables
- Raindrop: Bearer token in Authorization header
- Claude: x-api-key header
- Tokens passed as owned String to clients; never logged

**Concurrency:** Async/await with tokio runtime
- HTTP clients created once, reused
- Semaphore permits ensure bounded concurrency
- Stream operations with `buffer_unordered` for parallel processing

---

*Architecture analysis: 2026-01-30*
