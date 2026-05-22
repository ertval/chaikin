use chaikin::{Point, chaikin_step};

#[test]
fn one_segment_coordinates() {
    // Three points; first segment (0,0)→(4,0) cuts at 25% and 75%
    let input = [
        Point { x: 0.0, y: 0.0 },
        Point { x: 4.0, y: 0.0 },
        Point { x: 0.0, y: 4.0 },
    ];
    let out = chaikin_step(&input);
    assert_eq!(
        out[0..2],
        [
            Point { x: 1.0, y: 0.0 },
            Point { x: 3.0, y: 0.0 },
        ]
    );
}

#[test]
fn point_count_after_one_iteration() {
    let input: Vec<Point> = (0..4).map(|i| Point { x: i as f64, y: 0.0 }).collect();
    let out = chaikin_step(&input);
    assert_eq!(out.len(), 2 * input.len() - 2);
}

#[test]
fn edge_case_zero_points() {
    assert!(chaikin_step(&[]).is_empty());
}

#[test]
fn edge_case_one_point() {
    let input = [Point { x: 2.0, y: 3.0 }];
    assert_eq!(chaikin_step(&input), input);
}

#[test]
fn edge_case_two_points() {
    let input = [
        Point { x: 0.0, y: 0.0 },
        Point { x: 10.0, y: 10.0 },
    ];
    assert_eq!(chaikin_step(&input), input);
}
