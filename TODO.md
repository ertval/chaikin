# Chaikin Project Plan

This file tracks the full requirements and implementation plan. It expands the original README checklist with more detail to ensure the project is complete and testable.

## Core algorithm (library)

- [x] Implement `chaikin_step(points: &[Point]) -> Vec<Point>`.
- [x] Handle edge cases without crashing: 0, 1, 2 points.
- [x] Unit tests:
- [x] Exact coordinates for a simple segment.
- [x] Count rule: `N -> 2N - 2` for `N >= 2`.
- [x] Edge cases: empty, one, two points.

## App state and input

- [ ] Define app state:
- [ ] `control_points: Vec<Point>`.
- [ ] `frames: Vec<Vec<Point>>` precomputed for steps 0..6.
- [ ] `animating: bool`, `current_step: usize`, and a timer or frame counter.
- [ ] Left mouse click:
- [ ] Convert screen coordinates to `Point` and append to `control_points`.
- [ ] Decide whether to allow adding points while animating (if not, ignore clicks).
- [ ] Enter key:
- [ ] If 0 points: do nothing (optionally display a short message).
- [ ] If 1 point: show point only, no animation.
- [ ] If 2 points: show a single line segment, no animation.
- [ ] If 3+ points: precompute 7 steps and start animation.
- [ ] Escape key:
- [ ] Quit immediately.

## Animation loop

- [ ] Step definitions:
- [ ] Step 0 is the original control points.
- [ ] Steps 1..6 are successive applications of Chaikin.
- [ ] Timing:
- [ ] Advance steps on a fixed timer or fixed frame count.
- [ ] After step 6, wrap back to step 0 and continue looping.
- [ ] Ensure animation restarts cleanly after new points are added and Enter is pressed.

## Rendering

- [ ] Clear the background every frame.
- [ ] Control points:
- [ ] Draw small circles at each control point.
- [ ] Curve:
- [ ] Draw a polyline for the current step when animating.
- [ ] For 2 points: draw a straight line segment.
- [ ] For 1 point: draw only the point.
- [ ] Decide whether to always show original control points during animation.

## Bonus (optional)

- [ ] Clear screen key (C or R): reset points and stop animation.
- [ ] Drag points:
- [ ] On mouse down, pick nearest point within a hit radius.
- [ ] While dragging, update that point and recompute frames if animating.
- [ ] On mouse up, release selection.

## Manual test checklist (audit)

- [ ] `cargo run` compiles and runs without warnings.
- [ ] Left click adds control points and shows circles.
- [ ] 3+ points + Enter starts animation.
- [ ] Animation runs 7 steps and loops.
- [ ] 1 point + Enter shows only the point.
- [ ] 2 points + Enter shows a straight line.
- [ ] Enter with no points does nothing; you can still add points after.
- [ ] Escape exits without errors.
- [ ] Bonus: optional message shown when Enter is pressed without points.
- [ ] Bonus: clear screen works.
- [ ] Bonus: dragging works and curve updates in real time.

## Notes

- Warning: "Failed to create server-side surface decoration: Missing" is harmless.
