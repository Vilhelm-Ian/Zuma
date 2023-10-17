use bevy::prelude::*;

const MAP1: [[f32; 2]; 21] = [
    [170.0, -00.0],
    [190.0, -100.0],
    [90.0, -110.0],
    [30.0, -90.0],
    [30.0, -60.0],
    [10.0, -20.0],
    [90.0, -00.0],
    [160.0, -20.0],
    [150.0, -50.0],
    [140.0, -100.0],
    [90.0, -90.0],
    [50.0, -90.0],
    [40.0, -60.0],
    [40.0, -20.0],
    [90.0, -20.0],
    [130.0, -10.0],
    [140.0, -50.0],
    [130.0, -80.0],
    [100.0, -80.0],
    [60.0, -80.0],
    [60.0, -50.0],
];

// I tried something it didn't work
//struct LinearEquation {
//    m: f32,
//    c: f32,
//}
//
//impl LinearEquation {
//    fn find_y(&self, x: f32) -> f32 {
//        self.m * x + self.c
//    }
//    fn new(p1: &Point, p2: &Point) -> LinearEquation {
//        // y=mx+c
//        let mut m = (p2.y - p1.y) / (p2.x - p1.x);
//        if m.is_infinite() {
//            m = 0.0;
//        }
//        let c = p2.y - m * p2.x;
//        LinearEquation { m, c }
//    }
//}

#[derive(Debug, Copy, Clone)]
struct Point {
    x: f32,
    y: f32,
}

impl Point {
    fn scale(&self, b: f32) -> Point {
        Point {
            x: self.x * b,
            y: self.y * b,
        }
    }
    fn new(point: [f32; 2]) -> Point {
        Point {
            x: point[0],
            y: point[1],
        }
    }
}

//impl Add for Point { This gave me some weird behaviour
//
//    type Output = Self;
//    fn add(self, other: Self) -> Self {
//        Self {
//            x: self.x + other.x,
//            y: self.x + other.y,
//        }
//    }
//}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
) {
    commands.spawn(Camera2dBundle::default());
    let map_converted = MAP1
        .into_iter()
        .map(|p| {
            let mut point = Point::new(p);
            point = point.scale(8.5);
            point
        })
        .collect();

    let map_points = draw_map(map_converted);
    let normalized_points = normalize_all_points(map_points);
    commands.spawn(SpriteBundle {
        texture: asset_server.load("background.png"),
        ..default()
    });
    for point in normalized_points {
        commands.spawn(SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0.25, 0.25, 0.75),
                custom_size: Some(Vec2::new(5.0, 5.0)),
                ..default()
            },
            transform: Transform::from_translation(Vec3::new(point.x - 860.0, point.y + 450.0, 1.)),
            ..default()
        });
    }
}

fn draw_map(map: Vec<Point>) -> Vec<Point> {
    let mut result = vec![];
    let mut i = 0;
    loop {
        let mut curve_points = bezier_curve(map[i], map[i + 1], map[i + 2]);
        result.append(&mut curve_points);

        i += 2;
        if i + 2 >= map.len() {
            break;
        }
    }
    result
}

fn bezier_curve(p0: Point, p1: Point, p2: Point) -> Vec<Point> {
    let mut result = vec![];
    for i in 0..100 {
        let t = i as f32 / 100.0;
        let p0_modified = p0.scale((1.0 - t) * (1.0 - t));
        let p1_modified = p1.scale(2.0 * (1.0 - t) * t);
        let p2_modified = p2.scale(t * t);
        result.push(add_points(vec![p0_modified, p1_modified, p2_modified]));
    }
    result
}

fn add_points(points: Vec<Point>) -> Point {
    let mut result = Point::new([0.0, 0.0]);
    for point in points {
        result.x += point.x;
        result.y += point.y;
    }
    result
}

fn normalize_all_points(mut points: Vec<Point>) -> Vec<Point> {
    let average = get_average_magnitude_between_all_points(&points);
    for i in 0..points.len() - 1 {
        points[i + 1] = normalize_and_scale(&[points[i], points[i + 1]], average);
    }
    points
}

fn normalize_and_scale(points: &[Point], n: f32) -> Point {
    let p1 = points[0];
    let p2 = points[1];
    let magnitude = get_magnitude_between_points(&p1, &p2);
    let x = p1.x + ((p2.x - p1.x) * n) / magnitude;
    let y = p1.y + ((p2.y - p1.y) * n) / magnitude;
    Point::new([x, y])
}

fn get_average_magnitude_between_all_points(points: &Vec<Point>) -> f32 {
    let mut points_sum = 0.0;
    for i in 0..points.len() - 1 {
        points_sum += get_magnitude_between_points(&points[i], &points[i + 1]);
    }
    points_sum / (points.len() as f32 - 1.0)
}

fn get_magnitude_between_points(p1: &Point, p2: &Point) -> f32 {
    ((p2.x - p1.x) * (p2.x - p1.x) + (p2.y - p1.y) * (p2.y - p1.y)).sqrt()
}

#[test]
fn normalize_two_points() {
    let p1 = Point::new([1.0, 1.0]);
    let p2 = Point::new([3.0, 3.0]);
    let points = vec![p1, p2];
    let normalized_point = normalize_and_scale(&points, 1.0);
    let result = get_magnitude_between_points(&points[0], &normalized_point);
    assert_eq!(result, 1.0);
}

#[test]
fn normalize_two_points_and_scale_by_two() {
    let p1 = Point::new([1.0, 1.0]);
    let p2 = Point::new([3.0, 3.0]);
    let points = vec![p1, p2];
    let normalized_point = normalize_and_scale(&points, 2.0);
    let result = get_magnitude_between_points(&points[0], &normalized_point);
    assert_eq!(result, 2.0);
}

#[test]
fn magnitude_between_two_points() {
    let p1 = Point::new([0.0, 0.0]);
    let p2 = Point::new([3.0, 4.0]);
    let result = get_magnitude_between_points(&p1, &p2);
    assert_eq!(result, 5.0);
}
#[test]
fn average_of_three_points() {
    let p1 = Point::new([0.0, 0.0]);
    let p2 = Point::new([3.0, 4.0]);
    let p3 = Point::new([3.0, 8.0]);
    let result = get_average_magnitude_between_all_points(&vec![p1, p2, p3]);
    assert_eq!(result, 4.5);
}

#[test]
fn normalize_three_points() {
    let p1 = Point::new([0.0, 0.0]);
    let p2 = Point::new([3.0, 4.0]);
    let p3 = Point::new([3.0, 8.0]);
    let points = vec![p1, p2, p3];
    let points = normalize_all_points(points);
    let magnitude_1 = get_magnitude_between_points(&points[0], &points[1]);
    let magnitude_2 = get_magnitude_between_points(&points[1], &points[2]);
    assert_eq!(magnitude_1, magnitude_2);
    assert_eq!(4.5, magnitude_2); //the average magnitude is 4.5
}

#[test]
fn normalize_four_points() {
    let p1 = Point::new([0.0, 0.0]);
    let p2 = Point::new([3.0, 4.0]);
    let p3 = Point::new([3.0, 8.0]);
    let p4 = Point::new([5.0, 8.0]);
    let points = vec![p1, p2, p3, p4];
    let points = normalize_all_points(points);
    let magnitude_1 = get_magnitude_between_points(&points[0], &points[1]);
    let magnitude_2 = get_magnitude_between_points(&points[1], &points[2]);
    assert_eq!(magnitude_1, magnitude_2);
}
