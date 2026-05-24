use crate::chaikin::Point;

pub const TARGET_FPS: usize = 60;
pub const STEP_FRAMES: u32 = 30;
pub const DRAG_RADIUS: f64 = 12.0;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StatusMessage {
    pub text: &'static str,
}

pub struct AppState {
    pub control_points: Vec<Point>,
    pub frames: Vec<Vec<Point>>,
    pub animating: bool,
    pub current_step: usize,
    pub step_frame_counter: u32,
    pub left_was_down: bool,
    pub enter_was_down: bool,
    pub message: Option<StatusMessage>,
    pub dragging_index: Option<usize>,
}

impl AppState {
    pub fn new() -> Self {
        AppState {
            control_points: Vec::new(),
            frames: Vec::new(),
            animating: false,
            current_step: 0,
            step_frame_counter: 0,
            left_was_down: false,
            enter_was_down: false,
            message: None,
            dragging_index: None,
        }
    }

    pub fn update(&mut self) {
        // Placeholder for event updates and animation logic
    }

    pub fn stop_animation(&mut self) {
        self.animating = false;
        self.frames.clear();
        self.current_step = 0;
        self.step_frame_counter = 0;
    }

    pub fn add_control_point(&mut self, point: Point) {
        self.stop_animation();
        self.control_points.push(point);
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
    fn test_app_state_update_placeholder() {
        let mut app = AppState::new();
        app.update();
        assert!(app.control_points.is_empty());
    }
}
