use chaikin::Point;
use minifb::{Key, MouseButton, MouseMode, Window, WindowOptions};

const WIDTH: usize = 800;
const HEIGHT: usize = 600;
const POINT_RADIUS: i32 = 6;

const COLOR_BG: u32 = 0xFF_1E_1E_2E;
const COLOR_POINT: u32 = 0xFF_F9_E2_AF;

fn main() {
    let mut window = Window::new(
        "Chaikin",
        WIDTH,
        HEIGHT,
        WindowOptions::default(),
    )
    .expect("failed to open window");

    window.set_target_fps(60);

    let mut buffer = vec![COLOR_BG; WIDTH * HEIGHT];
    let mut control_points: Vec<Point> = Vec::new();
    let mut left_was_down = false;

    while window.is_open() && !window.is_key_down(Key::Escape) {
        if let Some((x, y)) = window.get_mouse_pos(MouseMode::Clamp) {
            let left_down = window.get_mouse_down(MouseButton::Left);
            if left_down && !left_was_down {
                control_points.push(Point {
                    x: x as f64,
                    y: y as f64,
                });
            }
            left_was_down = left_down;
        } else {
            left_was_down = false;
        }

        buffer.fill(COLOR_BG);
        for point in &control_points {
            draw_circle(&mut buffer, WIDTH, HEIGHT, point.x, point.y, POINT_RADIUS, COLOR_POINT);
        }

        window
            .update_with_buffer(&buffer, WIDTH, HEIGHT)
            .expect("failed to update window");
    }
}

fn draw_circle(buffer: &mut [u32], width: usize, height: usize, cx: f64, cy: f64, r: i32, color: u32) {
    let r2 = r * r;
    for dy in -r..=r {
        for dx in -r..=r {
            if dx * dx + dy * dy <= r2 {
                let x = cx.round() as i32 + dx;
                let y = cy.round() as i32 + dy;
                if x >= 0 && y >= 0 && (x as usize) < width && (y as usize) < height {
                    buffer[y as usize * width + x as usize] = color;
                }
            }
        }
    }
}
