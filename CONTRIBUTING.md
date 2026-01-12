# Contributing to rubs

Thanks for your interest in contributing! This document covers everything you need to get started.

## Getting Started

1. Fork and clone the repository
2. Install [just](https://github.com/casey/just) for task running
3. Install [cargo-nextest](https://nexte.st/) for running tests
4. Run `just ci` to verify everything works

## Development Workflow

Before submitting a PR, run the full CI check locally:

```bash
just ci
```

This runs formatting checks, clippy lints, tests, and a release build.

Individual commands are also available:

```bash
just fmt        # Format code
just check      # Run clippy
just test       # Run tests
just build      # Build release binary
```

## Pull Requests

1. Create a branch from `main`
2. Make your changes with clear, atomic commits
3. Include tests for new functionality
4. Run `just ci` to verify everything passes
5. Open a PR with a summary explaining the change and why

## Commits

We maintain a linear history with no merge commits. Rebase your branch before merging.

### Format

```
topic: Short summary in imperative mood

Explain WHY this change was made, not what changed (the diff
shows that). What problem does it solve? What was the previous
behavior and why was it insufficient?

- Additional context if needed
- Implementation notes for non-obvious decisions
```

### Guidelines

- **Subject line**: `topic: summary` format, max 50 characters, imperative mood ("Add" not "Added")
- **Body**: Wrap at 72 characters, focus on motivation and context
- **Atomic commits**: Each commit should be a single logical change that builds and passes tests
- **Bisectable history**: Every commit must compile and pass CI independently

Common topics: `generator`, `crypto`, `tui`, `cli`, `build`, `ci`, `docs`, `test`

### Tooling

Use whatever git workflow you prefer. [jj](https://github.com/martinvonz/jj) works great if you like rewriting history. The important thing is the end result: clean, atomic commits with meaningful messages.

## Code Quality

The project enforces:

- `cargo fmt` formatting
- `cargo clippy` with warnings as errors
- Tests via `cargo nextest`

CI runs these checks on every PR. Save yourself a round-trip by running `just ci` locally first.

## Questions and Feature Requests

- **Questions**: Open a [Discussion](https://github.com/axeberg/rubs/discussions)
- **Feature requests**: Open a [Discussion](https://github.com/axeberg/rubs/discussions) in the Ideas category
- **Bugs**: Open an [Issue](https://github.com/axeberg/rubs/issues)

## License

Contributions are dual-licensed under MIT and BSD-2-Clause. By submitting a PR, you agree to license your contribution under these terms.
