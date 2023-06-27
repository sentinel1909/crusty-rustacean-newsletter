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
- finished Chapter 5, API is deployed to Digital Ocean and everything appears to be working as expected

# 2023-03-19
### Steps Completed
- finished Chapter 6, completed input validation for name and email fields, re-deployment to Digital Ocean successful.

# 2023-03-22
### Steps Completed
- began working through Chapter 7
- began implementing email server and associated unit test, forgot to mention that this project is also live on Shuttle.  The /health_check endpoint returns a 200 OK and the /subscriptions endpoint saves a name and email into the database.

# 2023-04-02
### Steps Completed
- continued working through Chapter 7
- completed up to basic refactoring of test suite into separate files.  Feeling a bit stuck with the next steps of extracting the startup code.  Will read in the book and carry on a bit later

# 2023-04-08
### Steps Completed
- continued working through Chapter 7
- completed first round of refactoring of test suite, finished up to Chapter 7.4

# 2023-04-10
## Steps Completed
- continued working through Chapter 7
- completed up to Chapter 7.7, "Sending a Confirmation Email"
- tiny stumbling block in that I forgot how to migrate changes to the production database, went back to Chapter 5 for a refresh
  - to migrate to the production data base on Digital Ocean, at the command prompt type: DATABASE_URL sqlx migrate run
- as part of today's progress, I resolved a couple of issues in GitHub actions such that all tests are now green

# 2023-04-11
## Steps Completed
- started into Chapter 7.7, but have fallen a bit flat.
- need to study state in Axum, having difficulty understanding how to put the email client in state and then access it as needed, may have to use a combination of state and extensions

# 2023-04-12
## Steps Completed
- managed to get the application state setup correctly (I think) with the database pool and email client
- tests that should be green by end of Chapter 7.7.1 are failing, not sure why yet
- Shuttle related code is broken because of the change to app state

# 2023-04-14
## Steps Completed
- completed up to the end of Chapter 7.7.2, all tests in the green
- figured out the blocker related to the tests from Chapter 7.7.1, was missing some essential code in the integration tests
- have reverted Shuttle main runtime to Axum boilerplate, as it was diverging from the book code, which is meant for deployment on Digital Ocean

# 2023-04-15
## Steps Completed
- completed Chapter 7, all tests in the green

# 2023-04-16
## Steps Completed
- started on Chapter 8
- had to go off and understand the fundamentals of error handling in Axum, the docs are not clear, needed some context
- good article here [Using Rust with Axum for Error Handling](https://blog.logrocket.com/rust-axum-error-handling/)
- key points:
  - Axum handlers must be infallible, meaning they don't run into errors while they're running
  - if a handler runs into an error, the incoming request won't get a response
  - fallible handlers and pattern matching are two ways to handle errors

# 2023-04-18
## Steps Completed
- completed up to the end of Chapter 8
- error handling for the subscribe handler is 90% there, but lost error resolution on the store_token function
- need to implement a missing anyhow::Error related trait on the StoreTokenError type

# 2023-04-21
## Steps Completed
- redo of Chapter 8 complete, with book code for subscribe error handling completed
- no error fidelity as described in the book, just not seeing the same things or messages that are expected
- will try to do the confirm handler next

# 2023-04-23
## Steps Completed
- Chapter 8 now working as intended
- all that was needed was a tracing::error! at the top of the SubscribeError IntoResponse code

# 2023-04-27
## Steps Completed
- a little behind in these updates
- added error handling to confirm handler to round out Chapter 8
- started working on main for running on Shuttle

# 2023-04-28
## Steps Completed
- completed up to end of Chapter 9, all tests pass
- a bit stalled in the main implementation for Shuttle, set up access to secrets and can spin up the database
- having difficulty understanding how to hook in to the rest of the app from the Shuttle main

# 2023-04-29
## Steps Completed
- completed up to end of Chapter 10.2.2
- all tests green

# 2023-05-13
## Steps Completed
- finished up to the end of Chapter 10.2
- all tests green
- this is a tough chapter, need to step back and review

# 2023-05-16
## Steps Completed
- finished up to the end of Chapter 10.6.3
- Welcome page served from "/"
- login form mocked up, login redirects back to "/"
- all tests green

# 2023-05-22
## Steps Completed
- finished up to the beginning of "Query Parameters" (Chapter 10.6.4.2)
- using Axum is a slog now, the work to figure out the conversions from Actix are almost out of reach, had to cheat by looking for hints in other repos
- login form refuses invalid credentials and redirects to the /login page, with no explanation
- all tests green

# 2023-05-22
## Steps Completed
- huge log jam get past Chapter 10.6.4.2, finally figured it out, just expand on the use of Redirect.
- finished up to the end of Chapter 10.6.4.3
- all tests green

# 2023-05-28
## Steps completed
- finished up to the end of Chapter 10.6.4.13
- another log jam is in front of me, how to delete cookies per Chapter 10.6.4.13
- all tests green

# 2023-05-28
## Steps completed
- finished up to the end of Chapter 10.6
- all tests green
- I'm afraid I just don't understand cookie management in Axum
- get thing working by just jumping to axum-flash, per [this repo](https://github.com/damccull/zero2prod) 

# 2023-05-28
## Steps completed
- have decided this is officially the end of the road for this project
- I've read ahead and have judged that the rest of Chapter 10 is beyond my skill level
- will leave this repo up to serve as a record/resource for those attempting to do the same thing
- perhaps someday I'll return and finish

## 2023-06-15
## Steps completed
- have restarted, finished up to the end of Chapter 10.7.4
- redis implemented for session storage
- all tests green

# 2023-06-19
## Steps completed
- mostly done Chapter 10.7
- lost on how to implement the typed interface to Session

# 2023-06-20
## Steps completed
- finished Chapter 10.7 and added TypedSession (replaces axum_session::Session)
- started on Chapter 10.8
- login functionality for admin user works
- code base general clean up and adding of comments
- began to add Askama templates for rendering of HTML (rather than static strings)
- added assets directory to hold CSS and other files
- tests green

# 2023-06-21
## Steps completed
- finished up to Chapter 10.8.2.1
- all tests green

# 2023-06-25
## Steps completed
- finished up to Chapter 10.9
- Askama template conversion complete for all API endpoints
- flash messages are showing up correctly, but I'm seeing cases where maybe they're not clearing properly, will have to monitor
- all tests green

# 2023-06-25
## Steps completed
- added password length check to admin/password/post route, to fully finish out Chapter 10.8
- added associated test for password length check
- all tests green

# 2023-06-27
## Steps completed
- finished up to the end of Chapter 10.9
- all tests green

