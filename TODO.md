# Chaikin Project Plan

This file tracks the implementation work needed to satisfy [requirements.md](docs/requirements.md) and the manual audit in [audit.md](docs/audit.md). The goal is to make each item concrete enough that the remaining work can be implemented and verified without guessing.

## 1. Core algorithm in the library

- [x] Keep `chaikin_step(points: &[Point]) -> Vec<Point>` as the single library entry point for one Chaikin iteration.
- [x] Preserve the required edge cases for 0, 1, and 2 input points.
- [x] Keep the implementation pure and deterministic so animation frames can be precomputed safely.
- [x] Maintain unit tests for the key behavior:
	- [x] Exact coordinates for a simple segment.
	- [x] Count rule: `N -> 2N - 2` for `N >= 2`.
	- [x] Empty, one-point, and two-point inputs.

## 2. App state and window loop

- [x] Build the interactive window around the existing `minifb` setup in `src/main.rs`.
- [x] Keep the window size fixed and explicit, with a dark background and enough room for a visible curve.
- [x] Add rendering constants: `WIDTH`, `HEIGHT`, `POINT_RADIUS`, `LINE_COLOR`, `POINT_COLOR`, `BG_COLOR`.
- [x] Add animation timing constants: `TARGET_FPS` (already 60) and `STEP_FRAMES` (e.g. 30 for 0.5s per step).
- [x] Introduce a dedicated `AppState` struct so the loop stays simple.
- [x] Store the current control points in `control_points: Vec<Point>`.
- [x] Store the animation frames in `frames: Vec<Vec<Point>>`, where each entry is one displayed step.
- [x] Track whether the app is animating with `animating: bool`.
- [x] Track the currently displayed step with `current_step: usize`.
- [x] Track a frame counter with `step_frame_counter: u32` so steps advance on a fixed cadence.
- [x] Track edge-triggered input with `left_was_down: bool` and `enter_was_down: bool`.
- [x] (Optional) Track `message: Option<StatusMessage>` for the no-points reminder.
- [x] (Bonus) Track dragging state with `dragging_index: Option<usize>` and a `drag_radius`.
- [x] Initialize state to a clean empty scene in a `new()` constructor.
- [x] Keep the main loop limited to: poll input -> update state -> render -> present buffer.
- [x] Open the window at a fixed initial position (`INITIAL_WINDOW_X`, `INITIAL_WINDOW_Y`) instead of a random placement.
- [x] Use the native system title bar for move/close (X11 on WSL); custom in-window chrome removed.

## 3. Mouse input

- [x] Read the mouse position with `window.get_mouse_pos(MouseMode::Clamp)`.
- [x] Read the left button with `window.get_mouse_down(MouseButton::Left)`.
- [x] Only add a point on the rising edge: `left_down && !left_was_down`.
- [x] Convert the mouse position into `Point { x: x as f64, y: y as f64 }`.
- [x] Decide and implement a single rule: ignore clicks while animating (simplest, avoids surprises).
- [x] When adding a new point, stop any animation (`animating = false`, `frames.clear()`, `current_step = 0`).
- [x] Keep left-click behavior stable so the user can place many points without repeats from a held button.

## 4. Enter key behavior

- [x] Track `enter_was_down` to detect the rising edge of Enter.
- [x] On Enter press with 0 points, do nothing except optionally set a message timer.
- [x] On Enter press with 1 point, stop animation and render only that point.
- [x] On Enter press with 2 points, stop animation and render a straight line.
- [x] On Enter press with 3+ points, build 7 frames and start animation at step 0.
- [x] If already animating, Enter should rebuild frames from the current control points and restart.
- [x] If the optional message is implemented, clear it when a point is added or when Enter is pressed with 1+ points.

## 5. Animation model

- [ ] Implement `build_frames(points: &[Point], steps: usize) -> Vec<Vec<Point>>`.
- [ ] Frame 0 is the original control points.
- [ ] Frames 1..steps-1 apply `chaikin_step` repeatedly to the prior frame.
- [ ] Use `steps = 7` so there are 7 displayed frames total (matches the requirement count).
- [ ] In the update step, if `animating` and `frames.len() > 0`, increment `step_frame_counter`.
- [ ] When `step_frame_counter >= STEP_FRAMES`, reset it to 0 and increment `current_step`.
- [ ] Wrap `current_step` with modulo: `(current_step + 1) % frames.len()`.
- [ ] Rebuild frames whenever the animation restarts or control points are modified.
- [ ] Guard against invalid animation states (do not animate when `control_points.len() < 3`).

## 6. Rendering

- [x] Clear the buffer each frame with the background color.
- [x] Select `active_points`:
	- If `animating`, use `frames[current_step]`.
	- If not animating, use `control_points`.
- [ ] For `active_points.len() >= 2`, draw a polyline by connecting consecutive points.
- [ ] For exactly 2 points, the polyline is just one straight line segment.
- [x] For exactly 1 point, skip line drawing and show only the point.
- [x] Draw a small circle for every control point (always visible, even during animation).
- [x] Add a `draw_line` helper (Bresenham or DDA) that writes pixels into the buffer.
- [x] Keep circle radius small and consistent with the reference video.
- [x] If the optional message is implemented, draw it last so it appears above the scene.

## 7. Escape and exit behavior

- [x] Bind Escape to close the window immediately.
- [x] Use the native window close button (system title bar).
- [x] Confirm the app exits cleanly while idle and while animating.
- [x] Keep shutdown path free of panics so the window can close without errors.

## 8. Bonus features

- [ ] Add a clear action, mapped to `C` or `R`, that resets points and stops animation.
- [ ] Clear should reset: `control_points`, `frames`, `animating`, `current_step`, and any message state.
- [ ] Add hit-testing for dragging: on left-down, find the nearest point within `drag_radius`.
- [ ] Store the active dragged index while the left button is held.
- [ ] While dragging, update the point to the current mouse position on every frame.
- [ ] If animating during drag, rebuild frames and restart so the curve updates in real time.
- [ ] On left-up, release `dragging_index`.

## 9. Manual test checklist from the audit

- [x] `cargo run` compiles and runs without warnings that block the workflow.
- [x] Left click on the canvas adds control points.
- [x] Each control point is marked with a small circle.
- [ ] With 3 or more points, pressing Enter starts the Chaikin animation.
- [ ] The animation completes 7 steps and then restarts.
- [x] Pressing Escape exits without errors.
- [x] Closing the window while animating exits without errors.
- [ ] With 1 point, pressing Enter shows only the point and nothing changes.
- [ ] With 2 points, pressing Enter shows only a straight line.
- [ ] Pressing Enter with no points does nothing.
- [ ] After pressing Enter with no points, the user can still add points normally.
- [ ] Bonus: pressing Enter with no points shows a short reminder message (if implemented).
- [ ] Bonus: clearing the screen works without relaunching the app.
- [ ] Bonus: dragging points updates the curve in real time.

## 10. Implementation order

- [x] Create the `AppState` struct and move loop-local state into it.
- [x] Add edge-triggered input for left click and Enter.
- [ ] Add static rendering: clear background, draw points, draw line/polyline.
- [ ] Add `build_frames` and animation timing with `STEP_FRAMES`.
- [ ] Handle the 0/1/2-point Enter cases explicitly.
- [x] Verify Escape exit behavior.
- [ ] Add optional no-points message if desired.
- [ ] Add clear key bonus and then dragging bonus.
- [ ] Finish with the manual audit checklist.

## Notes

- The app uses the X11 backend (`minifb` with `features = ["x11"]`) for native window decorations on WSL. Ensure `DISPLAY` is set (e.g. `:0`).
