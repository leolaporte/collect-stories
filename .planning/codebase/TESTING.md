# Testing Patterns

**Analysis Date:** 2026-01-30

## Test Framework

**Runner:**
- Cargo built-in test runner (no external test framework)
- Config: `Cargo.toml` [package] section with `edition = "2021"`
- Default unit test harness via `#[cfg(test)]` modules

**Assertion Library:**
- Rust standard `assert!()`, `assert_eq!()` macros (no external crate)

**Run Commands:**
```bash
cargo test              # Run all tests
cargo test -- --nocapture  # Run with output visible
cargo test --release    # Run with optimizations
```

## Current Testing State

**Critical Finding: No tests present**

The codebase currently has **zero test coverage**. No test modules, unit tests, integration tests, or test files exist in `src/`:

```
/home/leo/Projects/podcast-briefing/src/
├── main.rs            # No #[cfg(test)] module
├── config.rs          # No tests
├── raindrop.rs        # No tests
├── extractor.rs       # No tests
├── summarizer.rs      # No tests
├── clustering.rs      # No tests
└── briefing.rs        # No tests
```

Running `cargo test` will find no tests to execute.

## Test File Organization

**Recommended Location:** Co-located with source

**Naming Convention:** `#[cfg(test)] mod tests { ... }` at bottom of each `.rs` file

**Structure:**
```
src/config.rs          # Config module
├── pub struct Config { ... }
├── impl Config { ... }
└── #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_name() { ... }
    }
```

**Alternative:** Separate `tests/` directory for integration tests

```
tests/
├── integration_test.rs   # Full pipeline tests
└── fixtures/
    ├── sample.html       # Test data
    └── sample.json       # API response mocks
```

## Test Structure Template

**Unit Test Pattern:**
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_description() {
        // Arrange
        let input = setup_test_data();

        // Act
        let result = function_under_test(input);

        // Assert
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), expected_value);
    }

    #[test]
    fn test_error_handling() {
        let result = risky_operation();
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("expected message"));
    }
}
```

**Async Test Pattern (using `#[tokio::test]`):**
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_async_function() {
        let client = setup_async_client();
        let result = client.fetch_data().await;
        assert!(result.is_ok());
    }
}
```

## Areas Requiring Tests

**High Priority (Core Functionality):**

1. **`config.rs`** - Configuration loading
   - Test: `.env` file loading from multiple locations
   - Test: Missing env var error handling
   - Test: Environment variable fallback
   - Location: `src/config.rs` in `#[cfg(test)]` module

2. **`raindrop.rs`** - API client
   - Test: Bookmark parsing from JSON response
   - Test: Pagination logic (multiple pages)
   - Test: Empty result handling
   - Test: HTTP error handling (401, 403, 404, 5xx)
   - Location: `src/raindrop.rs` in `#[cfg(test)]` module

3. **`summarizer.rs`** - AI summarization
   - Test: Bullet point parsing (various formats: `-`, `*`, `•`, numbered)
   - Test: Content truncation at UTF-8 boundaries
   - Test: Retry logic with exponential backoff
   - Test: Rate limit detection in error messages
   - Test: Summary enum variants (Success, Insufficient, Failed)
   - Location: `src/summarizer.rs` in `#[cfg(test)]` module

4. **`extractor.rs`** - Content extraction
   - Test: Retry logic with semaphore limits
   - Test: Status code handling (401, 403, 404 return None; others retry)
   - Test: Minimum content length check (< 100 chars returns None)
   - Test: Empty/whitespace-only content rejection
   - Location: `src/extractor.rs` in `#[cfg(test)]` module

5. **`clustering.rs`** - Topic clustering
   - Test: JSON response parsing from Claude API
   - Test: Article index validation (bounds checking)
   - Test: Single story edge case (returns one "News" topic)
   - Test: Empty stories list handling
   - Test: Fallback chronological clustering on API failure
   - Location: `src/clustering.rs` in `#[cfg(test)]` module

6. **`briefing.rs`** - HTML/CSV generation
   - Test: HTML escaping for XSS prevention
   - Test: CSV escaping (quotes, commas, newlines)
   - Test: Date formatting edge cases
   - Test: Empty topic list handling
   - Test: File path generation with proper dates
   - Location: `src/briefing.rs` in `#[cfg(test)]` module

7. **`main.rs`** - Integration tests
   - Test: Show selection prompt handling (1, 2, 3, invalid input)
   - Test: End-to-end pipeline (requires mocking)
   - Location: `tests/integration_test.rs`

## Mocking Strategy

**HTTP Requests:** Use `mockito` or `wiremock` crate

**Dependencies to add to `Cargo.toml`:**
```toml
[dev-dependencies]
mockito = "1.0"          # HTTP mocking
tokio-test = "0.4"       # Tokio test utilities
tempfile = "3.8"         # Temporary files for testing
```

**Mock Pattern for HTTP:**
```rust
#[cfg(test)]
mod tests {
    use super::*;
    use mockito::mock;

    #[tokio::test]
    async fn test_fetch_with_mock() {
        let mut server = mockito::Server::new_async().await;
        let mock = server.mock("GET", "/rest/v1/raindrops/0")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"items":[],"count":0}"#)
            .create();

        // Test code using server.url()
        mock.assert();
    }
}
```

**What to Mock:**
- HTTP requests (all external API calls)
- Claude API responses
- Raindrop API responses
- File system operations (use `tempfile` crate)

**What NOT to Mock:**
- Parsing logic (test with real sample data)
- String manipulation functions
- Error handling paths (test with real errors)
- Configuration loading (test with actual .env files in tests)

## Fixtures and Test Data

**Test Data Organization:**
```
tests/
├── fixtures/
│   ├── raindrop_response.json     # Sample API response
│   ├── claude_response.json        # Sample summary response
│   ├── article_content.html        # Sample article HTML
│   └── invalid_response.json       # Error case
└── integration_test.rs
```

**Fixture Example in Code:**
```rust
#[cfg(test)]
mod tests {
    const SAMPLE_RAINDROP_RESPONSE: &str = r#"{
        "items": [{
            "_id": 12345,
            "title": "Test Article",
            "link": "https://example.com/test",
            "tags": ["#twit"],
            "created": "2026-01-30T12:00:00Z"
        }],
        "count": 1
    }"#;

    #[test]
    fn test_parse_response() {
        let parsed: RaindropResponse = serde_json::from_str(SAMPLE_RAINDROP_RESPONSE)
            .expect("Should parse sample response");
        assert_eq!(parsed.items.len(), 1);
        assert_eq!(parsed.items[0].title, "Test Article");
    }
}
```

## Coverage Goals

**Minimum Target: 80% coverage**

**By Module (estimated current → target):**
- `config.rs`: 0% → 90% (critical for setup)
- `raindrop.rs`: 0% → 85% (API integration)
- `extractor.rs`: 0% → 85% (retry logic complexity)
- `summarizer.rs`: 0% → 90% (multiple error paths)
- `clustering.rs`: 0% → 80% (fallback logic)
- `briefing.rs`: 0% → 85% (output formatting)
- `main.rs`: 0% → 50% (interactive CLI, harder to test fully)

**View Coverage:**
```bash
cargo tarpaulin --out Html --output-dir coverage/
# Opens coverage/tarpaulin-report.html
```

## Test Dependencies (Required)

Add to `Cargo.toml`:

```toml
[dev-dependencies]
mockito = "1.2"        # HTTP mocking
tokio-test = "0.4"     # Tokio utilities
tempfile = "3.8"       # Temporary test files
serde_json = "1.0"     # JSON test data
```

## Priority Test Implementation Order

1. **Phase 1 (Core):** `config.rs` tests - simplest, no async
2. **Phase 2 (Data):** `briefing.rs` tests - parsing and escaping
3. **Phase 3 (API):** `raindrop.rs` tests - mocked HTTP
4. **Phase 4 (Processing):** `summarizer.rs` tests - retry logic
5. **Phase 5 (Extraction):** `extractor.rs` tests - semaphore behavior
6. **Phase 6 (Clustering):** `clustering.rs` tests - JSON parsing
7. **Phase 7 (Integration):** `tests/integration_test.rs` - full pipeline

---

*Testing analysis: 2026-01-30*
