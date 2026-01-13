# Instructions
Always build code that conforms to the following priorities (order is significant)

#### Simple
The API - and code - should be intuitive and unsurprising. Less is more is a guiding principle.

#### Resource Efficient
Running the service should require minimal resources, both in terms of memory and CPU usage.

#### Secure & Reliable

This project should be built in such way that it has as few attack vectors as possible. Reducing complexity is a corner stone for achieving this.

#### Testable
Code should be easy to test, and the API of functions should make them easy to write tests for.

#### Performant
Performance is important and desirable, but not as important as the above priorities.

## Dependencies
The following crates.io dependencies are considered "whitelisted", no explicit approval is required for using them.
- regex
- clap
- tokio
- serde
- serde_json
- http
- axum
- reqwest
- log
- walkdir
- itertools
- giro
- quickcheck
- quickcheck_macros

In addition to this, any depdenency which is hosted on GitHub AND belongs to the user 'mantono' is also permitted.

For all other dependencies, clear and explicit approval must be asked before the dependencies may be used. Avoid adding dependencies for minor or trivial functionality that we could implement ourselves. Only enable the minimal amount of features required for dependencies, i.e. no default features.

## Code Style
- Use small-to-medium functions; avoid deeply nested logic
- Prefer functional idioms over purely imperative code
- Never use `unwrap()`, except for in tests or doctest
- Make functions `const` when possible

### Library Crates
- Never use `expect()`
- Never expose error types in the public API which are declared in dependecy crates

### Binary Crates
- Use of `expect()` may only be permitted if an error is of an irrevoverable nature
- Create an own application error enum that can be used to convert to from underlying errors. This error enum should implement the Error trait and each error should map towards a unique status coe (for binary crates) 

## Tests
Where it is suitable, use quickcheck for property based testing, otherwise use the regular mechanism for testing in Rust. Keep a limited scope of each test, prefer many smaller tests over sinlge larger tests that tests multiple things at once.

## Documentation
- Any public (`pub`) function should be documented, and any non-trivial private function as well
- Add doctests for public functions

## Validation
Always verify that
- code has corrext syntax (`cargo check --all-features`)
- tests are passing (`cargo test --all-features`)
- code is correctly formatted (`cargo fmt`)
- code adheres to best practices (`cargo clippy`)
