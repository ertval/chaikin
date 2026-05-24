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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_draw_circle() {
        let mut buffer = vec![0u32; 25];
        let width = 5;
        let height = 5;
        let cx = 2.0;
        let cy = 2.0;
        let r = 1;
        let color = 0xFFFFFFFF;

        draw_circle(&mut buffer, width, height, cx, cy, r, color);

        // Center pixel (2, 2) => index 2 * 5 + 2 = 12
        assert_eq!(buffer[12], color);
        // Pixels on direct edges (dx=0, dy=1), (dx=0, dy=-1), (dx=1, dy=0), (dx=-1, dy=0) should be colored
        assert_eq!(buffer[7], color);  // (2, 1) => index 7
        assert_eq!(buffer[17], color); // (2, 3) => index 17
        assert_eq!(buffer[11], color); // (1, 2) => index 11
        assert_eq!(buffer[13], color); // (3, 2) => index 13

        // Diagonal pixel (1, 1) => index 6. dx=1, dy=1 => dx^2 + dy^2 = 2 > r^2 (1), so it should not be colored
        assert_eq!(buffer[6], 0);
    }

    #[test]
    fn test_placeholders_dont_panic() {
        let mut buffer = vec![0u32; 25];
        draw_line(&mut buffer, 5, 5, Point { x: 0.0, y: 0.0 }, Point { x: 4.0, y: 4.0 }, 0xFFFFFFFF);
        draw_text_message(&mut buffer, 5, 5, "Hello", 0xFFFFFFFF);
        assert!(true);
    }
}

