#!/usr/bin/env -S just --justfile

_default:
  @just --list -u

# ==================== ALIASES ====================
alias r := ready
alias f := fix

# ==================== SETUP & INITIALIZATION ====================

# Install git pre-commit hook to format files
install-hook:
  echo -e "#!/bin/sh\njust fmt" > .git/hooks/pre-commit
  chmod +x .git/hooks/pre-commit

# ==================== CORE DEVELOPMENT ====================

# When ready, run the same CI commands
ready:
  # git diff --exit-code --quiet
  # typos
  just fmt
  just check
  just test
  just lint
  just doc
  got status

# Run cargo check
check:
  cargo ck

# Run all the tests
test:
  cargo test --all-features

# Lint the whole project
lint:
  cargo lint -- --deny warnings

# Format all files
fmt:
  cargo fmt --all
  dprint fmt

[unix]
doc:
  RUSTDOCFLAGS='-D warnings' cargo doc --no-deps --document-private-items

[windows]
doc:
  $Env:RUSTDOCFLAGS='-D warnings'; cargo doc --no-deps --document-private-items

# Fix all auto-fixable format and lint issues
fix:
  cargo clippy --fix --allow-staged --no-deps
  just fmt
  typos -w
  git status

# ==================== DEVELOPMENT TOOLS ====================

watch *args='':
  watchexec --no-vcs-ignore {{args}}

watch-check:
  just watch "'cargo check; cargo clippy'"

watch-example *args='':
  just watch 'just example {{args}}'

# Run examples in parser, formatter, linter
example tool *args='':
  cargo run -p oxc_{{tool}} --example {{tool}} -- {{args}}

# Run the benchmarks
benchmark:
  cargo benchmark

# Run benchmarks for a single component
benchmark-one *args:
  cargo benchmark --bench {{args}} --no-default-features --features {{args}}

# ==================== TESTING & CONFORMANCE ====================

# Get code coverage
codecov:
  cargo codecov --html
