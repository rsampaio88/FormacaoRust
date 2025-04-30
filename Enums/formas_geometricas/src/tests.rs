/*
 * =======================================================================
 * Author:     Rita Ferreira
 * File:       tests.rs
 * Purpose:    For testing.
 * =======================================================================
 */

#[cfg(test)]
mod tests {
    use crate::shapes::{Square, Circle, Cube, Cylinder, Sphere, RegularPolygon, Shape};

    #[test]
    fn test_square() {
        let s = Square { side: 4.0 };
        assert_eq!(s.area().unwrap(), 16.0);
        assert_eq!(s.perimeter().unwrap(), 16.0);
    }

    #[test]
    fn test_circle() {
        let c = Circle { radius: 3.0 };
        assert_eq!(c.area().unwrap(), 28.274333882308138);
        assert_eq!(c.perimeter().unwrap(), 18.84955592153876);
    }

    #[test]
    fn test_cube() {
        let c = Cube { side: 3.0 };
        assert_eq!(c.volume().unwrap(), 27.0);
    }

    #[test]
    fn test_cylinder() {
        let cyl = Cylinder { radius: 3.0, height: 5.0 };
        assert_eq!(cyl.volume().unwrap(), 141.3716694115407);
        assert_eq!(cyl.area().unwrap(), 150.79644737231007);
    }

    #[test]
    fn test_sphere() {
        let s = Sphere { radius: 3.0 };
        assert_eq!(s.volume().unwrap(), 113.09733542922654);
        assert_eq!(s.area().unwrap(), 113.09733542922654);
    }

    #[test]
    fn test_polygon() {
        let p = RegularPolygon { sides: 6, side_length: 2.0 };
        assert!((p.area().unwrap() - 10.392304845413264).abs() < 0.0001);
    }
}

