set dotenv-load

project_name := "play-jam-4"

export PD_BUILD_BINDINGS_ONCE := "1"

# Perform all verifications (compile, test, lint, etc.)
verify: test lint

# Run the game from sources
run:
    cargo playdate run

# Watch the source files and run `just verify` when source changes
watch:
	cargo watch --delay 0.1 --clear --why -- just verify

# Run the tests
test:
	cargo hack check --feature-powerset --workspace
	cargo hack test --each-feature --workspace --exclude {{project_name}} --exclude crankit-game-loop

# Run the static code analysis
lint:
	cargo fmt -- --check
	cargo hack clippy --each-feature

# Clean up compilation output
clean:
	rm -rf target
	rm -rf node_modules

# Build and prepare release package
package:
	cargo playdate package --release
	strip target/release/playdate/{{project_name}}.pdx/pdex.so

# Install cargo dev-tools used by the `verify` recipe (requires rustup to be already installed)
install-dev-tools:
	rustup install stable
	rustup override set stable
	cargo install cargo-hack cargo-watch cargo-msrv

# Install a git hook to run tests before every commits
install-git-hooks:
	echo '#!/usr/bin/env sh' > .git/hooks/pre-push
	echo 'just verify' >> .git/hooks/pre-push
	chmod +x .git/hooks/pre-push

