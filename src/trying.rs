use std::fmt;

struct Point {
    x: f32,
    y: f32,
}

// Self refers to Point
impl Point {
    fn origin() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
        }
    }

    fn from(x: f32, y: f32) -> Self {
        Self {
            x,
            y,
        }
    }

    fn distance(&self, p2: &Point) -> f32 {
        ((self.x - p2.x).powf(2.0) + (self.y - p2.y).powf(2.0)).sqrt()
    }

    fn slope_to(&self, p: &Point) -> Option<f32> {
        let dx = p.x - self.x;
        if dx == 0.0 {
            None
        } else {
            Some((p.y - self.y) / dx)
        }
    }
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

fn main() {
    let o = Point::origin();
    let p = Point::from(19.0, 31.0);
    let q = Point::from(3.0, 4.0);
    let r = Point::from(1.0, -10.0);
    let points = vec![p, q, r];
    println!("{:?}", points.iter().map(|p| (*p).distance(&o)).collect::<Vec<_>>());
    println!("Slope to: {:?}", points.iter().map(|p| p.slope_to(&o)).collect::<Vec<_>>());
}

