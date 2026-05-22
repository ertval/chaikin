# Chaikin (Rust)

## Repo layout

```
chaikin/
├── Cargo.toml
├── README.md
├── src/
│   ├── main.rs
│   ├── chaikin.rs
│   └── lib.rs
└── tests/
    └── chaikin_tests.rs
```

## Steps

### 1. Core logic first (no UI) — done

Implement a pure function, e.g. `chaikin_step(points) -> new_points`:

- [x] Input: list of `{x, y}`
- [x] Output: list after one iteration
- [x] Handle 0, 1, 2 points without crashing (return same list or empty)

Write unit tests for:

- [x] Exact coordinates after one iteration on a simple segment (e.g. (0,0) to (4,0) → points at 1 and 3)
- [x] Count: N points → 2N − 2 points
- [x] Edge cases: `[]`, `[one]`, `[two]` don’t crash

### 2. Build the window and canvas — done

- [x] Open a window
- [x] Draw background
- [x] Mouse left click → add a control point
- [x] Draw a small circle at each control point

### 3. Keyboard behavior — pending

- [ ] Enter with 0 points → nothing (optional “draw points first” message)
- [ ] Enter with 1 point → show point only
- [ ] Enter with 2 points → draw line between them
- [ ] Enter with 3+ points → start animation
- [ ] Escape → quit

### 4. Animation loop — pending

- [ ] Keep `step` from 0 to 6 (7 steps total)
- [ ] Each frame (or on a timer): show the polyline at the current step
- [ ] Step 0 = original control points; step k = apply Chaikin k times
- [ ] After step 6, go back to step 0 and repeat

You can either precompute all 7 point lists when Enter is pressed, or recompute each frame — precomputing is simpler.

### 5. Drawing — pending

- [ ] Control points: circles
- [ ] Curve: connect points with lines (polyline)
- [ ] During animation, draw the current step’s polyline (and optionally still show original control points)

### 6. Polish (optional bonus) — pending

- [ ] Clear key (e.g. C or R) → wipe points and stop animation
- [ ] Drag control points → update position on mouse move while button held

### 7. Manual test checklist — in progress

- [ ] Click 4+ points, Enter → smooth animation, 7 steps, loops
- [ ] 1 point, Enter → only dot
- [ ] 2 points, Enter → straight line
- [ ] Enter with no points → no crash
- [ ] Escape closes app
- [x] Run unit tests and they pass


So far:

cargo test: to run the tests

With cargo run: opens a window and to test it:

left mouse click - working /
escape to close the window

** DON'T BOTHER WITH THIS WARNING:

Failed to create server-side surface decoration: Missing