use crate::chaikin::Point;

pub struct AppState {
    pub control_points: Vec<Point>,
    pub frames: Vec<Vec<Point>>,
    pub animating: bool,
    pub current_step: usize,
    pub step_frame_counter: u32,
    pub left_was_down: bool,
    pub enter_was_down: bool,
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
            dragging_index: None,
        }
    }

    pub fn update(&mut self) {
        // Placeholder for event updates and animation logic
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
        assert_eq!(app.dragging_index, None);
    }

    #[test]
    fn test_app_state_update_placeholder() {
        let mut app = AppState::new();
        app.update();
        assert!(app.control_points.is_empty());
    }
}

