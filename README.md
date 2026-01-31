# Podcast Briefing Tool

A Rust CLI tool that fetches bookmarked articles from Raindrop.io, summarizes them using Claude Haiku, groups related stories by topic, and generates an Emacs org-mode briefing document for TWiT, MacBreak Weekly, or Intelligent Machines podcasts.

## Features

- **Raindrop.io Integration**: Fetches tagged bookmarks from the past week
- **Article Extraction**: Parallel web scraping with retry logic
- **AI Summarization**: 5-bullet summaries using Claude Haiku 4.5
- **Topic Clustering**: Groups related articles automatically
- **Org-Mode Output**: Clean, structured Emacs org-mode documents
- **Simple Workflow**: One command generates your briefing

## How It Works

Run `collect-stories` to:
1. Fetch bookmarks from Raindrop.io (tagged with show)
2. Extract article content in parallel
3. Summarize each article with Claude AI
4. Cluster stories by topic (companies or categories)
5. Generate org-mode document in `~/Documents/`

**Output**: `~/Documents/{show}-{date}.org` ready to open in Emacs!

## Setup

### Prerequisites

- Rust toolchain (install from [rustup.rs](https://rustup.rs))
- Raindrop.io API token ([get here](https://app.raindrop.io/settings/integrations))
- Anthropic Claude API key ([get here](https://console.anthropic.com/settings/keys))

### Installation

1. Clone and build:
```bash
cd ~/Projects/podcast-briefing
cargo build --release --workspace
cp target/release/collect-stories ~/.local/bin/
```

2. Create environment file:
```bash
mkdir -p ~/.config/podcast-briefing
cat > ~/.config/podcast-briefing/.env << 'EOF'
RAINDROP_API_TOKEN=your_raindrop_token_here
ANTHROPIC_API_KEY=your_anthropic_api_key_here
EOF
```

### Environment Variables

Create a `.env` file at `~/.config/podcast-briefing/.env`:

```env
RAINDROP_API_TOKEN=your_raindrop_token_here
ANTHROPIC_API_KEY=your_anthropic_api_key_here
```

The tool searches for `.env` in these locations (in order):
1. Current directory (for development)
2. `~/.config/podcast-briefing/.env` (recommended)
3. `~/.env` (home directory)
4. System environment variables

## Usage

Run `collect-stories` to generate your briefing:

```bash
# Interactive mode (prompts for show selection)
collect-stories

# Non-interactive mode
collect-stories --show twit
collect-stories --show mbw --days 14
```

**Options:**
- `--show <slug>` - Show to collect for: `twit`, `mbw`, or `im` (default: interactive prompt)
- `--days <num>` - Number of days to look back (default: 7)

The tool will:
1. 📚 Fetch bookmarks with the corresponding tag from Raindrop.io
2. 🌐 Extract article content in parallel
3. 🤖 Summarize each article using Claude Haiku
4. 🔗 Group articles by company or topic
5. 📝 Generate org-mode document

**Output:** `~/Documents/{show}-{date}.org`

Then open in Emacs:
```bash
emacsclient ~/Documents/twit-2026-01-31.org
```

## Show Tags

Tag your bookmarks in Raindrop.io with these tags:

- TWiT: `#twit`
- MacBreak Weekly: `#mbw`
- Intelligent Machines: `#im`

## Org-Mode Output Format

`collect-stories` generates an Emacs org-mode document in `~/Documents/`:

```org
#+TITLE: TWiT Briefing - 2026-01-31
#+DATE: 2026-01-31

* Apple

** Apple unveils new MacBook Pro

*** URL
https://example.com/article

*** Summary
- First key point
- Second key point
- Third key point

* Google

** Google releases Gemini update

*** URL
https://example.com/article2

*** Summary
- Key point about Gemini
- Another important detail
```

**Org-mode structure:**
- Level 1 (`*`) - Topic names (company or category)
- Level 2 (`**`) - Article titles
- Level 3 (`***`) - URL and Summary sections
- Summary bullets use standard org-mode list format (`-`)

**Using in Emacs:**
- Open the `.org` file in Emacs
- Use `TAB` to fold/unfold sections
- Navigate with `C-c C-n` (next heading) and `C-c C-p` (previous heading)
- Export to other formats with `C-c C-e` (org-export)

## Workflow Examples

**Basic workflow:**
```bash
# Collect and generate briefing
collect-stories --show twit
# Output: ~/Documents/twit-2026-01-31.org

# Open in Emacs
emacsclient ~/Documents/twit-2026-01-31.org
```

**Different time ranges:**
```bash
collect-stories --show mbw --days 14  # Last 2 weeks
collect-stories --show im --days 3    # Last 3 days
```

## Cost Estimate

- Claude Haiku 4.5: ~$0.001 per article (summarization + clustering)
- For 100 articles: <$0.10 total

## Error Handling

The tools gracefully handle:
- **Missing API keys**: Helpful error messages with setup instructions
- **Invalid story files**: Version validation and corruption detection
- **Paywalled articles**: Marked as "No summary available"
- **Unreachable URLs**: Retry with exponential backoff
- **Rate limits**: Automatic throttling and retry
- **Failed clustering**: Chronological fallback

## Troubleshooting

**Problem:** `RAINDROP_API_TOKEN not found`
- **Solution:** Create `~/.config/podcast-briefing/.env` with your API tokens

**Problem:** No bookmarks found
- **Solution:** Check that you've tagged bookmarks in Raindrop.io with the correct tag (`#twit`, `#mbw`, or `#im`)

**Problem:** Articles failing to extract
- **Solution:** Some sites are paywalled or block scraping - this is normal, the tool will skip them

**Problem:** Rate limits from Claude API
- **Solution:** The tool automatically throttles and retries with exponential backoff

## Development

Build the workspace:
```bash
cargo build --workspace
```

Build for release:
```bash
cargo build --release --workspace
cp target/release/collect-stories ~/.local/bin/
```

Run in development:
```bash
cargo run -p collect-stories
cargo run -p prepare-briefing
```

Run tests:
```bash
cargo test --workspace
```

Check code:
```bash
cargo check --workspace
cargo clippy --workspace
```

## Project Structure

```
podcast-briefing/
├── Cargo.toml              # Workspace manifest
└── crates/
    ├── collect-stories/    # Main binary
    │   ├── Cargo.toml
    │   └── src/main.rs
    ├── prepare-briefing/   # Optional: HTML/CSV generation
    │   ├── Cargo.toml
    │   └── src/main.rs
    └── shared/             # Shared library
        ├── Cargo.toml
        └── src/
            ├── config.rs      # Environment configuration
            ├── raindrop.rs    # Raindrop.io API client
            ├── extractor.rs   # Web scraping
            ├── summarizer.rs  # Claude AI summarization
            ├── clustering.rs  # Topic clustering
            └── briefing.rs    # Org-mode/HTML/CSV generation
```

## License

MIT
