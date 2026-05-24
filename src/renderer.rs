use crate::chaikin::Point;

pub const WIDTH: usize = 800;
pub const HEIGHT: usize = 600;
pub const POINT_RADIUS: i32 = 6;

pub const BG_COLOR: u32 = 0xFF_1E_1E_2E;
pub const POINT_COLOR: u32 = 0xFF_F9_E2_AF;
pub const LINE_COLOR: u32 = 0xFF_CBA_6F7;

pub const INITIAL_WINDOW_X: isize = 100;
pub const INITIAL_WINDOW_Y: isize = 100;

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
    buffer: &mut [u32],
    width: usize,
    height: usize,
    p1: Point,
    p2: Point,
    color: u32,
) {
    let mut x0 = p1.x.round() as i32;
    let mut y0 = p1.y.round() as i32;
    let x1 = p2.x.round() as i32;
    let y1 = p2.y.round() as i32;
    let dx = (x1 - x0).abs();
    let dy = -(y1 - y0).abs();
    let sx = if x0 < x1 { 1 } else { -1 };
    let sy = if y0 < y1 { 1 } else { -1 };
    let mut err = dx + dy;

    loop {
        if x0 >= 0 && y0 >= 0 && (x0 as usize) < width && (y0 as usize) < height {
            buffer[y0 as usize * width + x0 as usize] = color;
        }
        if x0 == x1 && y0 == y1 {
            break;
        }
        let e2 = 2 * err;
        if e2 >= dy {
            err += dy;
            x0 += sx;
        }
        if e2 <= dx {
            err += dx;
            y0 += sy;
        }
    }
}

const CHAR_W: usize = 5;

// Each char is encoded as 7 rows of 5 pixels.
// In each byte, bit 4 is the leftmost pixel and bit 0 is the rightmost.
const FONT: &[(char, [u8; 7])] = &[
    (' ', [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]),
    ('A', [0x0E, 0x11, 0x11, 0x1F, 0x11, 0x11, 0x11]),
    ('c', [0x00, 0x00, 0x0E, 0x10, 0x10, 0x10, 0x0E]),
    ('d', [0x01, 0x01, 0x0F, 0x11, 0x11, 0x11, 0x0F]),
    ('f', [0x06, 0x08, 0x08, 0x1C, 0x08, 0x08, 0x08]),
    ('i', [0x04, 0x00, 0x04, 0x04, 0x04, 0x04, 0x0E]),
    ('l', [0x08, 0x08, 0x08, 0x08, 0x08, 0x08, 0x0E]),
    ('n', [0x00, 0x00, 0x1C, 0x12, 0x12, 0x12, 0x12]),
    ('o', [0x00, 0x00, 0x0E, 0x11, 0x11, 0x11, 0x0E]),
    ('p', [0x00, 0x1E, 0x11, 0x11, 0x1E, 0x10, 0x10]),
    ('r', [0x00, 0x00, 0x1E, 0x11, 0x10, 0x10, 0x10]),
    ('s', [0x00, 0x00, 0x0E, 0x10, 0x0E, 0x01, 0x0E]),
    ('t', [0x04, 0x04, 0x1F, 0x04, 0x04, 0x04, 0x03]),
];

fn lookup_char(c: char) -> [u8; 7] {
    FONT.iter()
        .find(|(ch, _)| *ch == c)
        .map(|(_, bitmap)| *bitmap)
        .unwrap_or([0u8; 7])
}

/// Draw a text message into the pixel buffer using the embedded bitmap font.
pub fn draw_text_message(
    buffer: &mut [u32],
    width: usize,
    height: usize,
    message: &str,
    color: u32,
) {
    let start_x: usize = 10;
    let start_y: usize = 10;

    for (i, ch) in message.chars().enumerate() {
        let bitmap = lookup_char(ch);
        let char_x = start_x + i * (CHAR_W + 1);
        for (row, &byte) in bitmap.iter().enumerate() {
            for col in 0..CHAR_W {
                if byte & (1 << (CHAR_W - 1 - col)) != 0 {
                    let px = char_x + col;
                    let py = start_y + row;
                    if px < width && py < height {
                        buffer[py * width + px] = color;
                    }
                }
            }
        }
    }
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

        assert_eq!(buffer[12], color);
        assert_eq!(buffer[7], color);
        assert_eq!(buffer[17], color);
        assert_eq!(buffer[11], color);
        assert_eq!(buffer[13], color);
        assert_eq!(buffer[6], 0);
    }

    #[test]
    fn test_draw_line() {
        let mut buffer = vec![0u32; 25];
        draw_line(
            &mut buffer,
            5,
            5,
            Point { x: 0.0, y: 0.0 },
            Point { x: 4.0, y: 0.0 },
            0xFFFFFFFF,
        );
        assert_eq!(buffer[0], 0xFFFFFFFF);
        assert_eq!(buffer[4], 0xFFFFFFFF);
    }

    #[test]
    fn test_placeholders_dont_panic() {
        let mut buffer = vec![0u32; 25];
        draw_line(&mut buffer, 5, 5, Point { x: 0.0, y: 0.0 }, Point { x: 4.0, y: 4.0 }, 0xFFFFFFFF);
        draw_text_message(&mut buffer, 5, 5, "Hello", 0xFFFFFFFF);
        assert!(true);
    }
}
