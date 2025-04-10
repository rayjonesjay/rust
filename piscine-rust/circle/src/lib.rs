use std::f64::consts::PI;

#[derive(Debug, Clone, Copy)]
pub struct Circle {
    pub center: Point,
    pub radius: f64,
}

impl Circle {
    pub fn new(x: f64,y: f64,radius:f64) -> Circle {
        Circle{
            center: Point(x,y),
            radius,
        }
    }
    // diameter = 2 * radius
    pub fn diameter(&self) -> f64 {
        self.radius*2f64
    }

    pub fn area(&self) -> f64 {
        self.radius.powi(2) * PI as f64
    }

    /*
        conditions for intersecting
        1. if distance <= sum of the two radii
        2. if distance < absolute difference sum of radii
        3. if distance == sum of radii
        4. if distance == absolute difference of sum of radii
    */

    pub fn intersect(&self,c: Circle) -> bool {
        // distance between self and c
        let distance  = self.center.distance(c.center);
        let radii_sum = self.radius + c.radius;
        let radii_diff_abs  = (self.radius - c.radius).abs();

        distance <= radii_sum && distance >= radii_diff_abs
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Point(pub f64,pub f64);

impl Point {
    pub fn distance(&self,p :Point) -> f64 {
        ((p.0 - self.0).powi(2) + (p.1-self.1).powi(2)).sqrt()
    }
}