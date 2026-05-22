#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

/// One iteration of Chaikin's corner-cutting algorithm (open curve).
pub fn chaikin_step(points: &[Point]) -> Vec<Point> {
    match points.len() {
        0 => vec![],
        1 | 2 => points.to_vec(),
        _ => {
            let mut out = Vec::with_capacity(2 * points.len() - 2);
            for window in points.windows(2) {
                let a = window[0];
                let b = window[1];
                out.push(Point {
                    x: 0.75 * a.x + 0.25 * b.x,
                    y: 0.75 * a.y + 0.25 * b.y,
                });
                out.push(Point {
                    x: 0.25 * a.x + 0.75 * b.x,
                    y: 0.25 * a.y + 0.75 * b.y,
                });
            }
            out
        }
    }
}
