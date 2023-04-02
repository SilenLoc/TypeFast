@_list:
	just --list --unsorted


run:
    cargo run

test:
    cargo test

# Perform all verifications (compile, test, lint etc.)
verify: test lint

# Run the static code analysis
lint:
	cargo fmt --check
	cargo clippy --all-targets

# run the release process in dry run mode (requires `npm`, a `GITHUB_TOKEN` and a `CARGO_REGISTRY_TOKEN`)
release *args:
	npm install --no-save conventional-changelog-conventionalcommits @semantic-release/exec
	npx semantic-release {{args}}      