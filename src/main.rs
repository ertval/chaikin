use chaikin::app::AppState;
use chaikin::chaikin::Point;
use chaikin::renderer::{
    self, draw_circle, BG_COLOR, HEIGHT, INITIAL_WINDOW_X, INITIAL_WINDOW_Y, LINE_COLOR,
    POINT_COLOR, POINT_RADIUS, WIDTH,
};
use minifb::{Key, MouseButton, MouseMode, Window, WindowOptions};

fn poll_input(window: &Window, state: &mut AppState) {
    if let Some((x, y)) = window.get_mouse_pos(MouseMode::Clamp) {
        let left_down = window.get_mouse_down(MouseButton::Left);
        if left_down && !state.left_was_down {
            state.control_points.push(Point {
                x: x as f64,
                y: y as f64,
            });
        }
        state.left_was_down = left_down;
    } else {
        state.left_was_down = false;
    }

    let enter_down = window.is_key_down(Key::Enter);
    state.enter_was_down = enter_down;
}

fn render(buffer: &mut [u32], state: &AppState) {
    buffer.fill(BG_COLOR);

    let active_points = if state.animating && !state.frames.is_empty() {
        &state.frames[state.current_step]
    } else {
        &state.control_points
    };

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

        window
            .update_with_buffer(&buffer, WIDTH, HEIGHT)
            .expect("failed to update window");
    }
}
