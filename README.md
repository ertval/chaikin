# Chaikin's Corner-Cutting Subdivision (Rust)

[![Rust](https://img.shields.io/badge/Rust-1.80+-000000?style=flat-square&logo=rust&logoColor=white)](https://www.rust-lang.org)
[![CI](https://img.shields.io/github/actions/workflow/status/ertval/chaikin/rust.yml?style=flat-square&logo=github&logoColor=white)](https://github.com/ertval/chaikin/actions)
[![License](https://img.shields.io/badge/License-MIT-green?style=flat-square)](LICENSE)

---

**Problem:** Generating smooth curves from sparse control points is complex without using heavy math frameworks or relying on high-level GPU-bound graphics stacks.

**Solution:** An interactive Rust application implementing Chaikin's subdivision algorithm in a custom software renderer (Bresenham's and midpoint circle lines) drawing directly onto a 1D pixel buffer.

---

## Key Features

- **Interactive Canvas**: Add control points using mouse left-clicks.
- **Real-Time Drag and Drop**: Drag existing control points to reshape the curve in real time.
- **Animated Subdivision**: View a 7-step progressive animation of Chaikin's corner-cutting algorithm.
- **Strict Boundary Conditions**: Pins endpoints to ensure open curve limits remain anchored.
- **Custom Software Renderer**: Uses Bresenham's line algorithm and midpoint circle drawing directly on a 1D pixel buffer.
- **Robust Corner Cases**: Handles 0, 1, and 2 control points gracefully, displaying status alerts and fallback rendering modes.

---

## Architecture Flow

```
Mouse Input → AppState (points/drag/animation) → Chaikin Algorithm → Software Renderer (Bresenham) → Pixel Buffer → Window Display
```

---

## Directory Structure

- **`Cargo.toml`**: Project manifest & external dependencies
- **`README.md`**: Project overview and quickstart guide
- **`TODO.md`**: Requirements checklist and audit log
- **`docs/`**: Detailed documentation & specifications
  - **`audit.md`**: Functional audit test checklist
  - **`guide.md`**: Comprehensive logic, flow, Rust features, and Q&A
  - **`requirements.md`**: Core requirements and specifications
- **`src/`**: Application source code
  - **`lib.rs`**: Library entrypoint exposing modules
  - **`main.rs`**: OS event loop, input polling, and main renderer
  - **`app.rs`**: AppState, animation ticks, and drag-and-drop state machine
  - **`chaikin.rs`**: Mathematical implementation of corner-cutting algorithm
  - **`renderer.rs`**: Bresenham lines, circle filling, and text rendering primitives
- **`tests/`**: Automated testing suites
  - **`chaikin_tests.rs`**: Core algorithmic math and point length edge case tests
  - **`integration-tests.rs`**: Full state transition and functional audit verification tests

---

## Getting Started

### Prerequisites

You must have Rust and Cargo installed. To install them, follow the instructions at [rustup.rs](https://rustup.rs).

### Build and Run

To run the interactive application:
```bash
cargo run --release
```

### Run Tests

To run the full suite of unit and integration tests:
```bash
cargo test
```

---

## Interactive Controls

- **Left Click**: Place a new control point on the canvas (when not animating).
- **Left Click + Drag**: Drag an existing control point in real time.
- **Enter / Right Click**: Start / Restart the 7-step subdivision animation.
- **C / R**: Clear the screen, reset the state, and start over.
- **Escape**: Exit the application.

---

## Codebase Architecture & Deep Dive

For an in-depth explanation of the application's logic, flow, software rendering math, and the specific Rust language features utilized, please see:
*   [**Comprehensive Logic, Flow & Rust Architecture Guide**](file:///home/ertval/code/zone-modules/chaikin/docs/guide.md)

This guide includes:
- Structural analysis of `app.rs`, `chaikin.rs`, and `renderer.rs`.
- Rust concept explanation (trait derivations, pattern matching, memory safety, slices, lifetimes).
- Full answers to the audit checklist in `docs/audit.md`.
- Advanced Q&A testing understanding of Rust memory safety and performance design.

---

## Related
- [CV & Portfolio](https://ertval.github.io)
- [lem-in-e](https://github.com/ertval/lem-in-e) — Go algorithmic routing solver