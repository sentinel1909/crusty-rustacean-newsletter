# Progress to Date

## 2023-02-21
### Steps Completed
- project skeleton started and pushed up to a fresh GitHub repo: [crusty-rustacean-api](https://github.com/crusty-rustacean-api)

- toolchain setup complete
  - VS Code with Rust Analyzer

- Inner Development Loop
  - lld linker installed
  - cargo watch installed

- Continuous Integration (CI)
  - tests: cargo test
  - code coverage: cargo tarpaulin
  - linting: cargo clippy
    - cargo clippy -- -D warnings (fails the linter check if clippy emits any warnings)
  - formatting: cargo fmt
    - fine tune formatting for any project with a rustfmt.toml configuration file
  - auditing for security vulnerabilities: cargo audit
  - CI pipeline
    - installed GitHub Actions (per the book examples) in my repo
      - Actions seem to have lots of issues and warnings about things being deprecated, will have to explore this further

## 2023-02-22
### Steps Completed
- completed basic /health_check endpoint using Axum
- refactored to facilitate addition of tests in /tests folder
  - implemented test to check health_check endpoint
### Learning
- had modified the /health_check endpoint to return an HTML message, instead of a zero length body stated in the book
- health_check test failed...because the body had a length of 22, not zero...took me a second to see that

## 2023-02-23
### Steps Completed
- polished the /health_check endpoint test so that it uses a random port to test on
### Learning
- between yesterday and today, I overcame the first major hurdle I had in mind about converting to Axum.
  - hurdle was understanding how to structure the app so that it could spin up the server as a background task
  - also understand now how to get the test tasks to spin up a server with a random port


