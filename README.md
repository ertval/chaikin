# Chaikin (Rust)

Animated implementation of Chaikin's algorithm. See the project plan in TODO.md for full requirements and progress tracking.

## Repo layout

```
chaikin/
├── Cargo.toml
├── README.md
├── TODO.md
├── src/
│   ├── main.rs
│   ├── chaikin.rs
│   └── lib.rs
└── tests/
    └── chaikin_tests.rs
```

## Controls

- Left click: add a control point
- Enter: start animation (behavior depends on number of points)
- Escape: quit

## Build and run

- Run: `cargo run`
- Tests: `cargo test`

## Notes

- You might see a warning like "Failed to create server-side surface decoration: Missing". It is safe to ignore.