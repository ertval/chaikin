use chaikin::app::{AppState, StatusMessage, NO_POINTS_MESSAGE, STEP_FRAMES};
use chaikin::renderer::{
    draw_circle, BG_COLOR, HEIGHT, POINT_COLOR, POINT_RADIUS, WIDTH,
};
use chaikin::Point;

#[cfg(test)]
mod audit_tests {
    use super::*;

    #[test]
    fn test_compilation_and_basic_functionality() {
        // Audit: "Does it compile and run without warnings?"
        let state = AppState::new();
        assert_eq!(state.control_points.len(), 0);
        assert!(!state.animating);
        assert!(!state.show_result);
        assert_eq!(state.current_step, 0);
        assert_eq!(state.step_frame_counter, 0);
        assert_eq!(state.message, None);
    }

    #[test]
    fn test_control_point_marking() {
        // Audit: "Does the program allow you to mark these control points?"
        let mut state = AppState::new();
        let point1 = Point { x: 100.0, y: 100.0 };
        let point2 = Point { x: 200.0, y: 150.0 };
        
        state.add_control_point(point1);
        state.add_control_point(point2);
        
        assert_eq!(state.control_points.len(), 2);
        assert_eq!(state.control_points[0], point1);
        assert_eq!(state.control_points[1], point2);
    }

    #[test]
    fn test_control_point_rendering() {
        // Audit: "Can you confirm that the program draws a small circle around the control points in order to identify them?"
        // We verify that the draw_circle rendering primitive modifies pixels around a control point as expected.
        let mut buffer = vec![BG_COLOR; WIDTH * HEIGHT];
        let cx = 100.0;
        let cy = 100.0;

        draw_circle(&mut buffer, WIDTH, HEIGHT, cx, cy, POINT_RADIUS, POINT_COLOR);

        // Center pixel should be POINT_COLOR
        let idx = (cy.round() as usize) * WIDTH + (cx.round() as usize);
        assert_eq!(buffer[idx], POINT_COLOR);

        // A pixel just outside the radius should remain BG_COLOR
        let outside_idx = (cy.round() as usize) * WIDTH + (cx.round() as usize + (POINT_RADIUS + 2) as usize);
        assert_eq!(buffer[outside_idx], BG_COLOR);
    }

    #[test]
    fn test_single_control_point_behavior() {
        // Audit: "Can you confirm that only the control point is shown and nothing changes?"
        let mut state = AppState::new();
        let point = Point { x: 100.0, y: 100.0 };
        state.add_control_point(point);

        // Try to start animation with a single point
        state.handle_enter();

        assert_eq!(state.control_points.len(), 1);
        assert!(!state.animating); // Should not animate
        assert!(!state.show_result); // Should not show straight line

        // Active points should only contain the single control point
        let active = state.active_points();
        assert_eq!(active.len(), 1);
        assert_eq!(active[0], point);
    }

    #[test]
    fn test_two_control_points_straight_line() {
        // Audit: "Can you confirm that only a straight line was drawn?"
        let mut state = AppState::new();
        let point1 = Point { x: 50.0, y: 50.0 };
        let point2 = Point { x: 150.0, y: 150.0 };

        state.add_control_point(point1);
        state.add_control_point(point2);

        // Press Enter
        state.handle_enter();

        assert_eq!(state.control_points.len(), 2);
        assert!(!state.animating); // Should not animate
        assert!(state.show_result); // Should set show_result to draw straight line between them

        // Active points should be the original two control points
        let active = state.active_points();
        assert_eq!(active.len(), 2);
        assert_eq!(active[0], point1);
        assert_eq!(active[1], point2);
    }

    #[test]
    fn test_three_or_more_points_animation_start() {
        // Audit: "Does the animation of the Chaikin's algorithm start?"
        let mut state = AppState::new();
        state.add_control_point(Point { x: 100.0, y: 50.0 });
        state.add_control_point(Point { x: 150.0, y: 150.0 });
        state.add_control_point(Point { x: 50.0, y: 150.0 });

        // Press Enter
        state.handle_enter();

        assert_eq!(state.control_points.len(), 3);
        assert!(state.animating); // Animation should start
        assert_eq!(state.frames.len(), 7); // Covers 7 steps
    }

    #[test]
    fn test_seven_step_animation_cycle() {
        // Audit: "Does the animation complete 7 steps before restarting?"
        let mut state = AppState::new();
        state.add_control_point(Point { x: 100.0, y: 50.0 });
        state.add_control_point(Point { x: 150.0, y: 150.0 });
        state.add_control_point(Point { x: 50.0, y: 150.0 });

        state.handle_enter();
        assert!(state.animating);
        assert_eq!(state.current_step, 0);

        // Cycle through all 7 steps (from step 0 to step 6)
        for expected_step in 1..7 {
            // Advance the animation frames to the next step
            for _ in 0..STEP_FRAMES {
                state.update();
            }
            assert_eq!(state.current_step, expected_step);
            assert!(state.animating);
        }

        // The next advance should wrap back to step 0
        for _ in 0..STEP_FRAMES {
            state.update();
        }
        assert_eq!(state.current_step, 0);
        assert!(state.animating);
    }

    #[test]
    fn test_no_points_enter_behavior() {
        // Audit: "Does the program continue without errors?"
        // Audit: "After you pressed Enter before selecting points, is it possible to place points?"
        // Bonus: "When you pressed Enter without drawing any points, was a message displayed to inform you that you forgot to draw any points?"
        let mut state = AppState::new();

        // Press Enter with no points
        state.handle_enter();

        // Should display the warning message and not start animation
        assert_eq!(
            state.message,
            Some(StatusMessage {
                text: NO_POINTS_MESSAGE,
            })
        );
        assert!(!state.animating);
        assert_eq!(state.control_points.len(), 0);

        // Now place a control point
        let point = Point { x: 100.0, y: 100.0 };
        state.add_control_point(point);

        // The warning message should be cleared
        assert_eq!(state.message, None);
        assert_eq!(state.control_points.len(), 1);
        assert_eq!(state.control_points[0], point);
    }

    #[test]
    fn test_clear_screen() {
        // Bonus: "Is it possible to clear the screen and add new control points without killing and relaunching the program?"
        let mut state = AppState::new();
        state.add_control_point(Point { x: 100.0, y: 100.0 });
        state.add_control_point(Point { x: 200.0, y: 100.0 });
        state.add_control_point(Point { x: 150.0, y: 200.0 });
        state.handle_enter();

        assert!(state.animating);
        assert_eq!(state.control_points.len(), 3);

        // Clear the screen
        state.handle_clear();

        assert!(!state.animating);
        assert!(state.control_points.is_empty());
        assert!(state.frames.is_empty());
        assert_eq!(state.message, None);

        // Add a new control point after clearing
        let point = Point { x: 400.0, y: 300.0 };
        state.add_control_point(point);
        assert_eq!(state.control_points.len(), 1);
        assert_eq!(state.control_points[0], point);
    }

    #[test]
    fn test_drag_control_points_realtime() {
        // Bonus: "Is it possible to drag the control points in real time and get a new curve?"
        let mut state = AppState::new();
        let p1 = Point { x: 100.0, y: 100.0 };
        let p2 = Point { x: 200.0, y: 100.0 };
        let p3 = Point { x: 150.0, y: 200.0 };

        state.add_control_point(p1);
        state.add_control_point(p2);
        state.add_control_point(p3);
        state.handle_enter();

        assert!(state.animating);
        let original_frame_5 = state.frames[5].clone();

        // Drag the second control point in real time
        state.update_dragged_point(1, 250.0, 120.0);

        // Check if the control point was updated
        assert_eq!(state.control_points[1], Point { x: 250.0, y: 120.0 });

        // Check if the frames were updated and are different from original
        assert_ne!(state.frames[5], original_frame_5);
    }

    #[test]
    fn test_shutdown_behavior() {
        // Audit: "Does the program exit without errors?"
        let mut state = AppState::new();
        state.add_control_point(Point { x: 100.0, y: 100.0 });
        state.add_control_point(Point { x: 200.0, y: 100.0 });
        state.add_control_point(Point { x: 150.0, y: 200.0 });
        state.handle_enter();

        assert!(state.animating);

        // Prepare shutdown (simulates clean exit behavior when Escape is pressed)
        state.prepare_shutdown();

        assert!(!state.animating);
        assert!(state.frames.is_empty());
        assert_eq!(state.control_points.len(), 3); // Keeps control points for final state if needed
    }
}