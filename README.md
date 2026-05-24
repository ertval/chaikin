# Chaikin (Rust)

Animated implementation of Chaikin's algorithm. See the project plan in TODO.md for full requirements and progress tracking.

## Repo layout

```
chaikin/
├── Cargo.toml            # Project dependencies and manifest
├── README.md             # High-level overview and instructions
├── TODO.md               # Requirements and manual audit checklist
├── docs/                 # Project documentation requirements
│   ├── requirements.md
│   └── audit.md
├── src/                  # Library & binary source code
│   ├── lib.rs            # Library interface defining modules and public API
│   ├── main.rs           # Thin entry wrapper orchestrating UI loops
│   ├── chaikin.rs        # Pure mathematical corner-cutting algorithm
│   ├── app.rs            # AppState logic, animations, and interaction model
│   └── renderer.rs       # Custom buffer pixel-drawing utilities
└── tests/                # Integration test suites
    └── chaikin_tests.rs  # Test suite verifying Chaikin's math & point count edge cases
```

## Modular Architecture

To maintain clear separation of concerns, the application is divided into three key components:
- **Core Library (`chaikin.rs`)**: A pure mathematical implementation of Chaikin's algorithm. It has no GUI dependencies and is completely deterministic for precomputing frame steps.
- **Application Engine (`app.rs`)**: Handles the state management (`AppState`), tracks drag-and-drop actions, animation frame lists, cadence timers, and edge-triggered inputs.
- **Graphics Pipeline (`renderer.rs`)**: Provides general pixel buffer draw utilities (Bresenham lines, midpoint circles, and status alerts) independent of the windowing library.

## Controls

- **Left Click**: Place a control point (or drag existing control points)
- **Enter**: Start or restart Chaikin animation (requires 3+ points)
- **C** / **R**: Clear the canvas and reset the app state
- **Escape**: Quit window

## Build and run

- Run: `cargo run`
- Tests: `cargo test`

## Notes

- You might see a warning like "Failed to create server-side surface decoration: Missing". It is safe to ignore.