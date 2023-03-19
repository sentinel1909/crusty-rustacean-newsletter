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

## 2023-02-24
### Steps Completed
- using a sample repo provided by a member of the Shuttle Discord, I was able to refactor my code so that I could build and deploy on Shuttle AND build and deploy on any other service, including running locally
### Learning
- huge improvement in understanding gained today, in the following areas:
  - workspace setup
  - modules and structuring code into modules
  - how Shuttle can draw from and wrap around the "non-shuttle deployment" code, in order to host on Shuttle

## 2023-02-25
### Steps Completed
- implemented the /subscriptions route
  - went down a rabbit hole because Axum returns a different Status Code (422) when it's form handler fails. Tried to implement something to address this, but couldn't do it.  I hardwired the /health_check test to look for a 422 code instead of 400.  I will return to this before all is said and done.
- installed the database dependencies, including the sqlx-cli command line tool
- created the script to spin up a database and do the initial migrations
- implemented app configuration using the config crate

# 2023-03-01
### Steps Completed
- have completed up to 3.8.5.5 in the book, stuck now while I sort out the CI pipeline. Had difficulty with GitHub Actions (they never worked) and so decided to get familar with CircleCI.
- learned a few more things about workspaces and how to structure them
- learned about the ```cmd_lib``` crate
- learned about the ```just``` task runner

# 2023-03-06
### Steps Completed
- have spent 5 days trying to conquer CircleCI...I gave up.  There's just no good guidance on how to setup Postgres with it.  I returned to GitHub actions and managed to beat it into submission.  I was just having a pathing issue.  Once I ran stuff locally and visualized what was happening, I was able to fix things up.  There are still issues with things failing, but at least I'm 90% there now.

# 2023-03-07
### Steps Completed
- finished Chapter 3
- /subscriptions route and all tests are working
- the shuttle deployed side of this project needs it's database, will figure that out before I go further in the book

# 2023-03-08
### Steps Completed
- took a break from progress in the book
- worked on understanding how to spin up a Postgres database on Shuttle
- have got it working, but can't rely on the routes I've written for the non-Shuttle version of the project
- when I include the non-Shuttle version of the code as a dependency, compilation fails

# 2023-03-11
### Steps Completed
- figured out why the shuttle code wouldn't compile, thanks to the folks at Shuttle, they reminded me to read the relevant parts of Chapter 5 in Zero to Production, one must set sqlx to function offline
- completed a portion of the telemetry section of Chapter 4, began to instrument the code base

# 2023-03-12
### Steps Completed
- completed instrumenting up to request_id, need to figure out how to make this work in Axum

# 2023-03-12
### Steps Completed
- fumbled my way through implementation at the end of Chapter 4
- getting a level of instrumentation which appears "good enough", with the same request_id attached to the different actions, going to move on

# 2023-03-16
### Steps Completed
- circled back to the last steps in Chapter 4 and begged, borrowed and stole from [this repo](https://github.com/SaadiSave/zero2prod) to implement my telemetry
- started back into Chapter 5, intially had pathing problems because of how I structured the repo.  Sorted it all out and built the Docker image in preparation for deployment.

# 2023-03-18
### Steps Completed
- finished Chapter 5, api is deployed to Digital Ocean and everything appears to be working as expected


