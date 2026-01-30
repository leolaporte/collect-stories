# Codebase Structure

**Analysis Date:** 2026-01-30

## Directory Layout

```
podcast-briefing/
├── src/                        # Rust source code
│   ├── main.rs                 # Entry point: orchestration and CLI
│   ├── config.rs               # Configuration and environment setup
│   ├── raindrop.rs             # Raindrop.io API client
│   ├── extractor.rs            # Content extraction and HTML parsing
│   ├── summarizer.rs           # Claude AI summarization service
│   ├── clustering.rs           # Topic clustering via AI
│   └── briefing.rs             # Output generation and file I/O
├── Cargo.toml                  # Package manifest and dependencies
├── Cargo.lock                  # Locked dependency versions
├── .env.example                # Example environment configuration
├── .env                        # Environment variables (not committed)
├── .gitignore                  # Git ignore rules
├── README.md                   # Usage documentation
├── .github/
│   └── workflows/
│       └── build.yml           # GitHub Actions CI/CD pipeline
└── .planning/                  # Planning documentation (not committed)
```

## Directory Purposes

**src/**
- Purpose: All Rust source code
- Contains: Module files implementing the pipeline stages
- Key files: `main.rs` (orchestration), all stage modules

**.github/workflows/**
- Purpose: CI/CD automation
- Contains: GitHub Actions build matrix for multi-platform compilation
- Key files: `build.yml` (builds Linux AMD64, macOS AMD64, macOS ARM64 binaries on push/PR)

**target/**
- Purpose: Build artifacts and compiled binaries
- Generated: Yes (created by `cargo build`)
- Committed: No

**.planning/**
- Purpose: GSD analysis documentation
- Generated: Yes (by GSD agents)
- Committed: No

## Key File Locations

**Entry Points:**
- `src/main.rs`: CLI entrypoint; tokio::main async block orchestrates all stages
- `src/config.rs:Config::from_env()`: Configuration initialization called first

**Configuration:**
- `.env.example`: Template for required environment variables (RAINDROP_API_TOKEN, ANTHROPIC_API_KEY)
- `src/config.rs`: Config loading from .env in multiple locations (current dir, ~/.config/podcast-briefing/.env, ~/.env)
- `Cargo.toml`: Build configuration with target binary name and dependencies

**Core Logic:**
- `src/raindrop.rs`: Fetch bookmarks from Raindrop.io API with pagination
- `src/extractor.rs`: Parallel HTML→text extraction with retry logic and semaphore rate limiting
- `src/summarizer.rs`: Claude Haiku 3.5 API integration with rate limit handling
- `src/clustering.rs`: AI-based topic clustering with chronological fallback
- `src/briefing.rs`: HTML and CSV output generation and file writing

**Output:**
- Generated to `~/Documents/{show-slug}-{date}.html` (HTML briefing with collapsible sections)
- Generated to `~/Documents/{show-slug}-{date}-LINKS.csv` (URL/title spreadsheet format)

## Naming Conventions

**Files:**
- Snake case: `main.rs`, `config.rs`, `extractor.rs` (Rust convention)
- Single responsibility per file (one primary module type per file)
- Example: `content_extractor` logic lives in `extractor.rs`

**Functions:**
- Snake case: `fetch_bookmarks()`, `fetch_article_content()`, `cluster_stories()`
- Method naming: `new()` for constructors, `async` methods use verbs (`fetch_`, `cluster_`, `generate_`)
- Validation: `try_` prefix for fallible operations (`try_fetch_article()`, `try_summarize()`)

**Types/Structs:**
- PascalCase: `ContentExtractor`, `ClaudeSummarizer`, `TopicClusterer`, `BriefingGenerator`
- Enum variants: PascalCase within enum (`Summary::Success`, `Show::MacBreakWeekly`)
- API response structs: Private with `struct` prefix for clarity (`RaindropResponse`, `ClaudeResponse`)

**Variables:**
- Snake case: `articles_with_content`, `summary_map`, `content_extractor`, `api_token`
- Constants/Statics: UPPER_SNAKE_CASE (none currently used, but would follow this pattern)

## Where to Add New Code

**New Feature (e.g., New Show Type):**
- Add variant to `Show` enum in `src/main.rs` (lines 19-50)
- Update `prompt_show_selection()` match arm to display new option
- No changes needed to other modules; tag routing is already parameterized

**New Output Format (e.g., Markdown briefing):**
- Create new functions in `src/briefing.rs`: `generate_markdown()`, `save_markdown()`
- Add output generation call in `src/main.rs` after `generate_links_csv()`
- Follow existing pattern: accept `&[Topic]` + metadata, return `String`

**New Processing Stage (e.g., fact-checking):**
- Create new module file: `src/fact_checker.rs`
- Implement struct with `new(api_key)` constructor pattern
- Add `async fn check_facts(&self, stories: Vec<Story>) -> Result<Vec<Story>>`
- Call from `src/main.rs` main() between clustering and briefing generation
- Update progress logging

**New External Integration (e.g., Slack publishing):**
- Create `src/integrations/slack.rs` or add new `src/slack.rs`
- Follow client pattern from `src/raindrop.rs` or `src/summarizer.rs`
- Implement service struct with new() and async methods
- Add initialization and calling code in `src/main.rs`

**Test Files:**
- Rust convention: add `#[cfg(test)] mod tests { }` at end of source file
- Or create `tests/` directory at crate root for integration tests
- Currently no tests; follow pattern in examples if added

**Utilities/Helpers:**
- Shared string utilities: Add to existing relevant module or create `src/utils.rs`
- Example: HTML escaping currently in `src/briefing.rs`; reuse or extract if needed in multiple modules

## Special Directories

**Cargo Target Directory (target/):**
- Purpose: Compiled binaries and artifacts
- Generated: Yes (created by `cargo build --release`)
- Committed: No (in .gitignore)

**.planning/ Directory:**
- Purpose: GSD codebase analysis documents
- Generated: Yes (by GSD mapping agents)
- Committed: No
- Contains: ARCHITECTURE.md, STRUCTURE.md, CONVENTIONS.md, TESTING.md, CONCERNS.md, STACK.md, INTEGRATIONS.md

**No test directory:**
- Tests would follow Rust convention of co-location in source files or integration tests in top-level tests/ dir
- Currently no tests present; CLI focused on immediate functionality

## File Organization Rationale

**One-file-per-module approach:**
- Small, focused modules (avg 80-200 lines each)
- Clear separation of concerns (API clients, data processing, output)
- Easy navigation: feature name matches filename

**Minimal depth:**
- Root src/ contains all modules
- No nested src/services/, src/models/, src/api/ subdirectories
- Flat structure is appropriate for project size (7 files, ~900 lines total)

**If project grows beyond 1500 lines total:**
- Extract API clients to `src/clients/` (raindrop, claude)
- Group processing stages as `src/stages/` (extraction, summarization, clustering)
- Move output handlers to `src/output/` (briefing generation)

---

*Structure analysis: 2026-01-30*
