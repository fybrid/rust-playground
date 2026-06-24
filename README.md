# Rust Learning

This repository is a practical Rust learning path for software engineers who are new to Rust and want production-oriented habits from the beginning.

The workflow is simple: read a lesson, complete the missing code, run the tests, use compiler feedback, and compare with the solution only after you have made a real attempt.

## Quick Start

```bash
git clone <this-repository>
cd rust-learning
code .
```

In VS Code, choose **Reopen in Container** when prompted. Inside the Dev Container you can run:

```bash
make check
```

The root workspace validates the mini-project scaffolds and environment. The exercises are intentionally separate crates because their tests are meant to fail until you implement the missing code.

## Repository Map

```text
.devcontainer/       VS Code Dev Container setup
.github/workflows/   CI validation
notes/               personal learning notes
exercises/           incomplete test-driven exercises
solutions/           completed reference implementations
projects/            mini-project scaffolds
```

## Learning Path

Start with `exercises/01_basics`, then move through ownership, borrowing, error handling, and data modeling. Each module has a README, starter code in `src/lib.rs`, and integration tests under `tests/`.

Run one exercise module like this:

```bash
cd exercises/01_basics
cargo test
```

When tests fail, read the compiler output carefully. Rust's compiler messages are part of the learning experience, not noise.

## Mini Projects

After the first modules, use the projects under `projects/` to practice combining concepts in small realistic programs:

```bash
cargo run -p cli_calculator -- "2 + 3"
cargo test -p mini_grep
```

## Commands

```bash
make test
make fmt
make lint
make check
make run TARGET=projects/mini_grep
make shell
```

## Recommended Study Loop

Read the module README first. Implement the smallest amount of code needed to move to the next compiler error. Run tests often. Write notes in `notes/` whenever a compiler error teaches you something.

