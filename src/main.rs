use chaikin::app::AppState;
use chaikin::chaikin::Point;
use chaikin::renderer::{
    self, draw_circle, draw_line, BG_COLOR, HEIGHT, INITIAL_WINDOW_X, INITIAL_WINDOW_Y, LINE_COLOR,
    POINT_COLOR, POINT_RADIUS, WIDTH,
};
use minifb::{Key, MouseButton, MouseMode, Window, WindowOptions};

fn euclidean_distance(p1: Point, p2: Point) -> f64 {
    let dx = p1.x - p2.x;
    let dy = p1.y - p2.y;
    (dx * dx + dy * dy).sqrt()
}

fn is_close(p1: Point, p2: Point, threshold: f64) -> bool {
    euclidean_distance(p1, p2) < threshold
}

fn poll_input(window: &Window, state: &mut AppState) {
    use chaikin::app::DRAG_RADIUS;
    if let Some((x, y)) = window.get_mouse_pos(MouseMode::Clamp) {
        let left_down = window.get_mouse_down(MouseButton::Left);
        let mouse = Point { x: x as f64, y: y as f64 };

        if left_down && !state.left_was_down && !state.animating {
            let is_near = state.control_points.iter().position(|p| is_close(*p, mouse, DRAG_RADIUS));
            if let Some(idx) = is_near {
                state.dragging_index = Some(idx);
            } else {
                state.add_control_point(mouse);
            }
        }

        if left_down {
            if let Some(idx) = state.dragging_index {
                state.update_dragged_point(idx, x as f64, y as f64);
            }
        } else {
            state.dragging_index = None;
        }

        state.left_was_down = left_down;
    } else {
        state.left_was_down = false;
        state.dragging_index = None;
    }

    let enter_down = window.is_key_down(Key::Enter);
    let right_down = window.get_mouse_down(MouseButton::Right);
    if (enter_down && !state.enter_was_down) || (right_down && !state.right_was_down) {
        state.handle_enter();
    }
    state.enter_was_down = enter_down;
    state.right_was_down = right_down;
    let clear_down = window.is_key_down(Key::C);
    if clear_down && !state.clear_was_down {
        state.handle_clear();
    }
    state.clear_was_down = clear_down;
}

fn draw_polyline(buffer: &mut [u32], points: &[Point], color: u32) {
    for segment in points.windows(2) {
        draw_line(
            buffer,
            WIDTH,
            HEIGHT,
            segment[0],
            segment[1],
            color,
        );
    }
}

fn render(buffer: &mut [u32], state: &AppState) {
    buffer.fill(BG_COLOR);

    let active_points = state.active_points();

    for point in active_points {
        draw_circle(
            buffer,
            WIDTH,
            HEIGHT,
            point.x,
            point.y,
            POINT_RADIUS,
            POINT_COLOR,
        );
    }

    if (state.animating || state.show_result) && active_points.len() >= 2 {
        draw_polyline(buffer, active_points, LINE_COLOR);
    }

    if let Some(message) = &state.message {
        renderer::draw_text_message(buffer, WIDTH, HEIGHT, message.text, LINE_COLOR);
    }
}

fn main() {
    let mut window = Window::new(
        "Chaikin",
        WIDTH,
        HEIGHT,
        WindowOptions::default(),
    )
    .expect("failed to open window");

    window.set_target_fps(chaikin::app::TARGET_FPS);
    window.set_position(INITIAL_WINDOW_X, INITIAL_WINDOW_Y);

    let mut buffer = vec![BG_COLOR; WIDTH * HEIGHT];
    let mut state = AppState::new();

    while window.is_open() && !window.is_key_down(Key::Escape) {
        poll_input(&window, &mut state);
        state.update();
        render(&mut buffer, &state);

        if window.update_with_buffer(&buffer, WIDTH, HEIGHT).is_err() {
            break;
        }
    }

    state.prepare_shutdown();
}
