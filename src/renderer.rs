use crate::chaikin::Point;

/// Helper function to draw a filled circle centered at (cx, cy) with radius r.
pub fn draw_circle(buffer: &mut [u32], width: usize, height: usize, cx: f64, cy: f64, r: i32, color: u32) {
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

/// Helper function to draw a line segment from p1 to p2 using Bresenham's line algorithm.
pub fn draw_line(
    _buffer: &mut [u32],
    _width: usize,
    _height: usize,
    _p1: Point,
    _p2: Point,
    _color: u32,
) {
    // Placeholder: Implement Bresenham's line drawing algorithm
}

/// Helper function to draw a text status message onto the frame buffer.
pub fn draw_text_message(
    _buffer: &mut [u32],
    _width: usize,
    _height: usize,
    _message: &str,
    _color: u32,
) {
    // Placeholder: Render standard message alerts to buffer
}
