# Podcast Briefing Tool

A Rust CLI tool that fetches bookmarked articles from Raindrop.io, summarizes them using Claude Haiku, groups related stories by topic, and generates a markdown briefing document for TWiT, MacBreak Weekly, or Intelligent Machines podcasts.

## Features

- **Raindrop.io Integration**: Fetches tagged bookmarks from the past week
- **Article Extraction**: Parallel web scraping with retry logic
- **AI Summarization**: 5-bullet summaries using Claude Haiku 4.5
- **Topic Clustering**: Groups related articles automatically
- **Markdown Output**: Clean, formatted briefing documents

## Setup

### Prerequisites

- Rust toolchain (install from [rustup.rs](https://rustup.rs))
- Raindrop.io API token ([get here](https://app.raindrop.io/settings/integrations))
- Anthropic Claude API key ([get here](https://console.anthropic.com/settings/keys))

### Installation

1. Clone and build:
```bash
cd ~/Projects/podcast-briefing
cargo build --release
cp target/release/podcast-briefing ~/.local/bin/
```

2. Create `.env` file:
```bash
cp .env.example .env
# Edit .env and add your API tokens
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

Run the tool:

```bash
podcast-briefing
```

You'll be prompted to select a show:
1. TWiT
2. MacBreak Weekly
3. Intelligent Machines

The tool will:
1. Fetch bookmarks with the corresponding tag from Raindrop.io (past 7 days)
2. Extract article content in parallel
3. Summarize each article using Claude Haiku
4. Group articles by company (Apple, Google, Tesla, etc.) or general topic
5. Generate two files:
   - **HTML briefing** with collapsible sections: `~/Documents/{show-slug}-{date}.html`
   - **CSV links file** for Google Sheets: `~/Documents/{show-slug}-{date}-LINKS.csv`

## Show Tags

- TWiT: `#twit`
- MacBreak Weekly: `#mbw`
- Intelligent Machines: `#im`

Tag your bookmarks in Raindrop.io with these tags for the tool to find them.

## Output Format

The tool generates an **HTML file** with:
- **Collapsible summaries**: Click "Summary" to collapse/expand bullet points
- **Clickable links**: All article URLs are hyperlinked
- **Clean styling**: Professional appearance with color-coded sections
- **Proper heading hierarchy**: H1 (title) → H2 (topics) → H3 (articles)

### Using with Google Docs

**Method 1: Direct open (preserves collapsible sections)**
1. Open the HTML file in your browser (double-click it)
2. View and navigate with collapsible sections working

**Method 2: Import to Google Docs (for editing)**
1. Go to [Google Docs](https://docs.google.com)
2. File → Open → Upload the HTML file
3. Google Docs will convert it and create a **Document Outline** in the left sidebar
4. Use the outline to navigate between topics and articles
5. Note: Collapsible sections become regular bullet lists in Google Docs

**Method 3: Copy/Paste**
1. Open the HTML file in a browser
2. Select all (Ctrl+A) and copy (Ctrl+C)
3. Paste into a new Google Doc
4. Use View → Show document outline for navigation

### HTML Structure
```html
<h1>Show Name Briefing - YYYY-MM-DD</h1>

<h2>Apple</h2>

<h3>Apple unveils new MacBook Pro</h3>
Link: [clickable URL]
Date: Wednesday, 01/29/2026 3:17 AM

<details open>
  <summary>Summary (click to collapse)</summary>
  <ul>
    <li>First key point</li>
    <li>Second key point</li>
    <li>Third key point</li>
    <li>Fourth key point</li>
    <li>Fifth key point</li>
  </ul>
</details>
```

### Links CSV Format

The tool also generates a CSV file suitable for importing into Google Sheets:

**Format:**
- Column A: (blank)
- Column B: Topic title (on first article row only)
- Column C: Article titles
- Column D: (blank)
- Column E: Article URLs

**Example:**
```
,Apple,Apple unveils new MacBook Pro,,https://example.com/article1
,,New M4 chip benchmarks released,,https://example.com/article2
,,,,
,Google,Google releases Gemini update,,https://example.com/article3
,,Google Photos adds new AI features,,https://example.com/article4
,,,,
,AI Development,OpenAI announces new GPT model,,https://example.com/article5
```

**To use in Google Sheets:**
1. Open Google Sheets
2. File → Import → Upload the CSV file
3. Select "Comma" as the separator
4. Optionally:
   - Bold column B to make topics stand out
   - Adjust column widths for better readability

## Cost Estimate

- Claude Haiku 4.5: ~$0.001 per article
- For 100 articles: <$0.10 total

## Error Handling

The tool gracefully handles:
- Paywalled articles → "No summary available"
- Unreachable URLs → Retry with exponential backoff
- Rate limits → Automatic throttling
- Failed clustering → Chronological fallback

## Development

Build and run in development:
```bash
cargo run
```

Run tests:
```bash
cargo test
```

Check code:
```bash
cargo check
cargo clippy
```
