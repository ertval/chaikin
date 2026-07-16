# Audit Answers (code review)

This is based on static code review only; I did not run the program.

## Functional

### Run the program using `cargo run`.

#### Does it compile and run without warnings?

Answer: Not verifiable from code review alone. The entry point is `main()` in [src/main.rs](src/main.rs), and the helper modules appear fully used (no obvious unused functions or variables in [src/app.rs](src/app.rs), [src/chaikin.rs](src/chaikin.rs), or [src/renderer.rs](src/renderer.rs)). There is no `#![deny(warnings)]` or warning suppression present, so compiler warnings (if any) would depend on the full build.

### Left click on the canvas to set one or more control points.

#### Does the program allow you to mark these control points?

Answer: Yes. In `poll_input()` (see [src/main.rs](src/main.rs)), a left mouse click adds a control point when `!state.animating`, using `state.add_control_point(mouse)` when the click is not near an existing point. `add_control_point()` in [src/app.rs](src/app.rs) appends to `control_points` and resets animation state.

#### Can you confirm that the program draws a small circle around the control points in order to identify them?

Answer: Yes. `render()` in [src/main.rs](src/main.rs) iterates `active_points()` and calls `draw_circle()` for each, using `POINT_RADIUS` from [src/renderer.rs](src/renderer.rs). This draws filled circles around each control point every frame.

### Left click on the canvas to set three or more control points and press `Enter`.

#### Does the animation of the Chaikin's algorithm start?

Answer: Yes. `poll_input()` in [src/main.rs](src/main.rs) calls `handle_enter()` on `Enter`. In [src/app.rs](src/app.rs), `handle_enter()` builds frames via `build_frames()` and sets `animating = true` when there are three or more points. `update()` advances frames using `current_step` and `STEP_FRAMES`.

### Press `Escape` to exit the program.

#### Does the program exit without errors?

Answer: It should exit cleanly based on control flow. The main loop condition in [src/main.rs](src/main.rs) stops when `Escape` is down, and `prepare_shutdown()` is called after the loop. There is no explicit error handling beyond breaking if `update_with_buffer` fails, so runtime errors cannot be fully ruled out without running.

### Start the program and left click on the canvas to set just one control point and press `Enter`.

#### Can you confirm that only the control point is shown and nothing changes?

Answer: Yes. For one point, `handle_enter()` in [src/app.rs](src/app.rs) clears animation and sets `show_result = false`, so only the control point circle is rendered (no line drawing) in [src/main.rs](src/main.rs).

### Restart the program and left click on the canvas to set just two control points and press `Enter`.

#### Can you confirm that only a straight line was drawn?

Answer: Yes. With two points, `handle_enter()` sets `show_result = true` in [src/app.rs](src/app.rs). Then `render()` in [src/main.rs](src/main.rs) draws a polyline when `show_result` is true and `active_points.len() >= 2`, which yields a straight line between the two points.

### Restart the program and left click on the canvas to set three or more control points and press `Enter`.

#### Does the animation complete 7 steps before restarting?

Answer: Yes. `ANIMATION_STEPS` is `7` in [src/chaikin.rs](src/chaikin.rs), and `build_frames()` builds exactly that many frames. In [src/app.rs](src/app.rs), `update()` increments `current_step` modulo `frames.len()`, so it cycles through 7 frames before restarting.

### Restart the program and left click on the canvas to set three or more control points and press `Enter`. Then press `Escape` to exit the program.

#### Does the program exit without errors?

Answer: Same as above: it should exit cleanly by leaving the loop in [src/main.rs](src/main.rs) and calling `prepare_shutdown()`. No error handling is triggered in this path, but runtime errors cannot be fully confirmed without executing.

### Start the program and press `Enter` without selecting any points.

#### Does the program continue without errors?

Answer: Yes. `handle_enter()` in [src/app.rs](src/app.rs) handles the zero-point case without panicking and does not start animation. It sets a status message instead.

#### After you pressed `Enter` before selecting points, is it possible to place points without needing to kill the program?

Answer: Yes. `add_control_point()` in [src/app.rs](src/app.rs) clears the message and appends a point. `poll_input()` in [src/main.rs](src/main.rs) still allows left clicks while not animating, so point placement works after the message appears.

### Bonus

#### When you pressed `Enter` without drawing any points, was a message displayed to inform you that you forgot to draw any points?

Answer: Yes. `handle_enter()` sets `message = Some(StatusMessage { text: NO_POINTS_MESSAGE })` when there are zero points in [src/app.rs](src/app.rs), and `render()` draws it with `draw_text_message()` in [src/renderer.rs](src/renderer.rs).

#### Is it possible to clear the screen and add new control points without killing and relaunching the program?

Answer: Yes, by pressing `C`. `poll_input()` checks `Key::C` and calls `handle_clear()` in [src/app.rs](src/app.rs), which resets points, frames, and status so new points can be added.

#### Is it possible to drag the control points in real time and get a new curve?

Answer: Yes. `poll_input()` in [src/main.rs](src/main.rs) detects left-drag near a point (`DRAG_RADIUS`) and updates it via `update_dragged_point()` in [src/app.rs](src/app.rs). When animating and there are at least three points, `update_dragged_point()` rebuilds frames so the curve updates in real time.
