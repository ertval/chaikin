use chaikin::{build_frames, Point, chaikin_step, ANIMATION_STEPS};

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

#[test]
fn build_frames_produces_seven_steps() {
    let points = [
        Point { x: 0.0, y: 0.0 },
        Point { x: 4.0, y: 0.0 },
        Point { x: 4.0, y: 4.0 },
    ];
    let frames = build_frames(&points, ANIMATION_STEPS);
    assert_eq!(frames.len(), ANIMATION_STEPS);
    assert_eq!(frames[0], points);
    assert_ne!(frames[0], frames[1]);
}

#[test]
fn build_frames_returns_empty_for_fewer_than_three_points() {
    assert!(build_frames(&[Point { x: 0.0, y: 0.0 }], ANIMATION_STEPS).is_empty());
}

#[test]
fn build_frames_pins_first_and_last_points() {
    let points = [
        Point { x: 0.0, y: 0.0 },
        Point { x: 4.0, y: 0.0 },
        Point { x: 4.0, y: 4.0 },
    ];
    let frames = build_frames(&points, ANIMATION_STEPS);

    for frame in &frames {
        assert_eq!(frame[0], points[0]);
        assert_eq!(frame[frame.len() - 1], points[2]);
    }
}
