/*
 * =======================================================================
 * Author:     Rita Ferreira
 * File:       shapes.rs
 * Purpose:    Defines the Shape trait and several geometric types to,
 *            calculate area, perimeter, and volume.
 * =======================================================================
 */

use std::f64::consts::PI;

pub trait Shape {
    fn area(&self) -> Option<f64>;
    fn perimeter(&self) -> Option<f64>;
    fn volume(&self) -> Option<f64>;
}

pub struct Square {
    pub side: f64,
}

impl Shape for Square {
    fn area(&self) -> Option<f64> {
        Some(self.side * self.side)
    }

    fn perimeter(&self) -> Option<f64> {
        Some(4.0 * self.side)
    }

    fn volume(&self) -> Option<f64> {
        None 
    }
}

pub struct Circle {
    pub radius: f64,
}

impl Shape for Circle {
    fn area(&self) -> Option<f64> {
        Some(PI * self.radius * self.radius)
    }

    fn perimeter(&self) -> Option<f64> {
        Some(2.0 * PI * self.radius)
    }

    fn volume(&self) -> Option<f64> {
        None 
    }
}

pub struct Cube {
    pub side: f64,
}

impl Shape for Cube {
    fn area(&self) -> Option<f64> {
        Some(6.0 * self.side * self.side)
    }

    fn perimeter(&self) -> Option<f64> {
        None 
    }

    fn volume(&self) -> Option<f64> {
        Some(self.side.powi(3)) 
    }
}

pub struct Cylinder {
    pub radius: f64,
    pub height: f64,
}

impl Shape for Cylinder {
    fn area(&self) -> Option<f64> {
        Some(2.0 * PI * self.radius * (self.radius + self.height))
    }

    fn perimeter(&self) -> Option<f64> {
        None 
    }

    fn volume(&self) -> Option<f64> {
        Some(PI * self.radius * self.radius * self.height) 
    }
}

pub struct Sphere {
    pub radius: f64,
}

impl Shape for Sphere {
    fn area(&self) -> Option<f64> {
        Some(4.0 * PI * self.radius * self.radius)
    }

    fn perimeter(&self) -> Option<f64> {
        None 
    }

    fn volume(&self) -> Option<f64> {
        Some((4.0 / 3.0) * PI * self.radius.powi(3)) 
    }
}

pub struct RegularPolygon {
    pub sides: u32,
    pub length: f64,
}

impl Shape for RegularPolygon {
    fn area(&self) -> Option<f64> {
        let angle = PI * (self.sides - 2) as f64 / self.sides as f64;
        Some(self.sides as f64 * self.length * self.length / (4.0 * (angle / 2.0).tan()))
    }

    fn perimeter(&self) -> Option<f64> {
        Some(self.sides as f64 * self.length) 
    }

    fn volume(&self) -> Option<f64> {
        None /
    }
}
