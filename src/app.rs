use crate::chaikin::{build_frames, Point, ANIMATION_STEPS};

pub const TARGET_FPS: usize = 60;
pub const STEP_FRAMES: u32 = 30;
pub const DRAG_RADIUS: f64 = 12.0;

pub const NO_POINTS_MESSAGE: &str = "Add control points first";

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StatusMessage {
    pub text: &'static str,
}

pub struct AppState {
    pub control_points: Vec<Point>,
    pub frames: Vec<Vec<Point>>,
    pub animating: bool,
    pub show_result: bool,
    pub current_step: usize,
    pub step_frame_counter: u32,
    pub left_was_down: bool,
    pub enter_was_down: bool,
    pub clear_was_down: bool,
    pub message: Option<StatusMessage>,
    pub dragging_index: Option<usize>,
}

impl AppState {
    pub fn new() -> Self {
        AppState {
            control_points: Vec::new(),
            frames: Vec::new(),
            animating: false,
            show_result: false,
            current_step: 0,
            step_frame_counter: 0,
            left_was_down: false,
            enter_was_down: false,
            clear_was_down: false,
            message: None,
            dragging_index: None,
        }
    }

    pub fn update(&mut self) {
        if !self.animating || self.frames.is_empty() {
            return;
        }

        self.step_frame_counter += 1;
        if self.step_frame_counter >= STEP_FRAMES {
            self.step_frame_counter = 0;
            self.current_step = (self.current_step + 1) % self.frames.len();
        }
    }

    pub fn stop_animation(&mut self) {
        self.animating = false;
        self.frames.clear();
        self.current_step = 0;
        self.step_frame_counter = 0;
    }

    pub fn active_points(&self) -> &[Point] {
        if self.animating {
            if let Some(frame) = self.frames.get(self.current_step) {
                return frame;
            }
        }
        &self.control_points
    }

    pub fn prepare_shutdown(&mut self) {
        self.stop_animation();
    }

    pub fn add_control_point(&mut self, point: Point) {
        self.message = None;
        self.show_result = false;
        self.stop_animation();
        self.control_points.push(point);
    }

    pub fn handle_enter(&mut self) {
        match self.control_points.len() {
            0 => {
                self.message = Some(StatusMessage {
                    text: NO_POINTS_MESSAGE,
                });
                self.show_result = false;
            }
            1 => {
                self.message = None;
                self.show_result = false;
                self.stop_animation();
            }
            2 => {
                self.message = None;
                self.show_result = true;
                self.stop_animation();
            }
            _ => {
                self.message = None;
                self.show_result = false;
                self.frames = build_frames(&self.control_points, ANIMATION_STEPS);
                self.animating = !self.frames.is_empty();
                self.current_step = 0;
                self.step_frame_counter = 0;
            }
        }
    }
    pub fn handle_clear(&mut self) {
        self.control_points.clear();
        self.frames.clear();
        self.stop_animation();
        self.message = None;
        self.show_result = false;
        self.current_step = 0;
        self.step_frame_counter = 0;
        self.dragging_index = None;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_app_state_initialization() {
        let app = AppState::new();
        assert!(app.control_points.is_empty());
        assert!(app.frames.is_empty());
        assert!(!app.animating);
        assert_eq!(app.current_step, 0);
        assert_eq!(app.step_frame_counter, 0);
        assert!(!app.left_was_down);
        assert!(!app.enter_was_down);
        assert_eq!(app.message, None);
        assert_eq!(app.dragging_index, None);
    }

    #[test]
    fn test_stop_animation() {
        let mut app = AppState::new();
        app.animating = true;
        app.frames.push(vec![Point { x: 1.0, y: 2.0 }]);
        app.current_step = 3;
        app.step_frame_counter = 10;

        app.stop_animation();

        assert!(!app.animating);
        assert!(app.frames.is_empty());
        assert_eq!(app.current_step, 0);
        assert_eq!(app.step_frame_counter, 0);
    }

    #[test]
    fn test_clear_resets_all_state() {
        let mut app = AppState::new();
        // 1. dirty the state (add points, start animation, set a message...)
        app.control_points.push(Point { x: 1.0, y: 2.0 });
        app.animating = true;
        app.frames.push(vec![Point { x: 3.0, y: 4.0 }]);
        app.current_step = 1;
        app.step_frame_counter = 5;
        app.message = Some(StatusMessage { text: NO_POINTS_MESSAGE });
        // 2. call app.clear()
        app.handle_clear();
        // 3. assert everything is back to default
        assert!(app.control_points.is_empty());
        assert!(app.frames.is_empty());
        assert!(!app.animating);
        assert_eq!(app.current_step, 0);
        assert_eq!(app.step_frame_counter, 0);
        assert_eq!(app.message, None);
    }

    #[test]
    fn test_add_control_point_stops_animation() {
        let mut app = AppState::new();
        app.animating = true;
        app.frames.push(vec![]);
        app.current_step = 2;

        app.add_control_point(Point { x: 5.0, y: 6.0 });

        assert!(!app.animating);
        assert!(app.frames.is_empty());
        assert_eq!(app.current_step, 0);
        assert_eq!(app.control_points.len(), 1);
        assert_eq!(app.control_points[0], Point { x: 5.0, y: 6.0 });
    }

    #[test]
    fn test_handle_enter_zero_points_sets_message() {
        let mut app = AppState::new();
        app.handle_enter();
        assert_eq!(
            app.message,
            Some(StatusMessage {
                text: NO_POINTS_MESSAGE,
            })
        );
        assert!(!app.animating);
    }

    #[test]
    fn test_handle_enter_one_point_stops_animation() {
        let mut app = AppState::new();
        app.control_points.push(Point { x: 1.0, y: 2.0 });
        app.animating = true;
        app.frames.push(vec![]);

        app.handle_enter();

        assert!(!app.animating);
        assert!(app.frames.is_empty());
        assert_eq!(app.message, None);
    }

    #[test]
    fn test_handle_enter_two_points_stops_animation() {
        let mut app = AppState::new();
        app.control_points.push(Point { x: 0.0, y: 0.0 });
        app.control_points.push(Point { x: 10.0, y: 10.0 });
        app.animating = true;

        app.handle_enter();

        assert!(!app.animating);
        assert_eq!(app.message, None);
    }

    #[test]
    fn test_handle_enter_three_points_starts_animation() {
        let mut app = AppState::new();
        app.control_points.push(Point { x: 0.0, y: 0.0 });
        app.control_points.push(Point { x: 10.0, y: 0.0 });
        app.control_points.push(Point { x: 10.0, y: 10.0 });

        app.handle_enter();

        assert!(app.animating);
        assert_eq!(app.frames.len(), ANIMATION_STEPS);
        assert_eq!(app.current_step, 0);
        assert_eq!(app.message, None);
    }

    #[test]
    fn test_handle_enter_while_animating_rebuilds_frames() {
        let mut app = AppState::new();
        app.control_points.push(Point { x: 0.0, y: 0.0 });
        app.control_points.push(Point { x: 10.0, y: 0.0 });
        app.control_points.push(Point { x: 10.0, y: 10.0 });
        app.handle_enter();
        app.current_step = 5;

        app.handle_enter();

        assert!(app.animating);
        assert_eq!(app.frames.len(), ANIMATION_STEPS);
        assert_eq!(app.current_step, 0);
    }

    #[test]
    fn test_add_control_point_clears_message() {
        let mut app = AppState::new();
        app.message = Some(StatusMessage {
            text: NO_POINTS_MESSAGE,
        });

        app.add_control_point(Point { x: 1.0, y: 1.0 });

        assert_eq!(app.message, None);
    }

    #[test]
    fn test_update_advances_animation_step() {
        let mut app = AppState::new();
        app.animating = true;
        app.frames = vec![vec![], vec![]];
        app.step_frame_counter = STEP_FRAMES - 1;

        app.update();

        assert_eq!(app.step_frame_counter, 0);
        assert_eq!(app.current_step, 1);
    }

    #[test]
    fn test_update_wraps_animation_step() {
        let mut app = AppState::new();
        app.animating = true;
        app.frames = vec![vec![], vec![]];
        app.current_step = 1;
        app.step_frame_counter = STEP_FRAMES - 1;

        app.update();

        assert_eq!(app.current_step, 0);
    }

    #[test]
    fn test_shutdown_while_animating_leaves_clean_state() {
        let mut app = AppState::new();
        app.control_points.push(Point { x: 0.0, y: 0.0 });
        app.control_points.push(Point { x: 10.0, y: 0.0 });
        app.control_points.push(Point { x: 10.0, y: 10.0 });
        app.handle_enter();
        app.current_step = 4;

        app.prepare_shutdown();

        assert!(!app.animating);
        assert!(app.frames.is_empty());
        assert_eq!(app.current_step, 0);
        assert_eq!(app.control_points.len(), 3);
    }

    #[test]
    fn test_active_points_falls_back_when_frame_missing() {
        let mut app = AppState::new();
        app.control_points.push(Point { x: 1.0, y: 2.0 });
        app.animating = true;
        app.current_step = 0;

        assert_eq!(app.active_points(), &[Point { x: 1.0, y: 2.0 }]);
    }

    #[test]
    fn test_app_state_update_placeholder() {
        let mut app = AppState::new();
        app.update();
        assert!(app.control_points.is_empty());
        assert_eq!(app.step_frame_counter, 0);
    }
}
