/*
 * =======================================================================
 * Author:     Rita Ferreira
 * File:       main.rs
 * Purpose:    explore geometric shapes and compute their properties.
 * =======================================================================
 */

mod shapes;

use shapes::*;
use std::io;

fn main() {
    geometric_shapes();
}

fn geometric_shapes() {
    loop {
        println!(
            "\nChoose a geometric shape:
1) Square
2) Circle
3) Cube
4) Cylinder
5) Sphere
6) Regular Polygon
7) Exit"
        );

        let choice = read_u32("Your choice: ");

        match choice {
            1 => {
                let side = read_f64("Enter the side length of the square: ");
                let square = Square { side };
                show_result(&square);
            }
            2 => {
                let radius = read_f64("Enter the radius of the circle: ");
                let circle = Circle { radius };
                show_result(&circle);
            }
            3 => {
                let side = read_f64("Enter the side length of the cube: ");
                let cube = Cube { side };
                show_result(&cube);
            }
            4 => {
                let radius = read_f64("Enter the radius of the cylinder: ");
                let height = read_f64("Enter the height of the cylinder: ");
                let cylinder = Cylinder { radius, height };
                show_result(&cylinder);
            }
            5 => {
                let radius = read_f64("Enter the radius of the sphere: ");
                let sphere = Sphere { radius };
                show_result(&sphere);
            }
            6 => {
                let sides = read_u32("Enter the number of sides: ");
                let length = read_f64("Enter the length of each side: ");
                let poly = RegularPolygon { sides, length };
                show_result(&poly);
            }
            7 => {
                println!("Leaving..");
                break;
            }
            _ => println!("Sorry, invalid option."),
        }
    }
}

fn show_result<T: Shape>(shape: &T) {
    match shape.area() {
        Some(area) => println!("Area: {:.2}", area),
        None => println!("Area: Not applicable"),
    }
    match shape.perimeter() {
        Some(p) => println!("Perimeter: {:.2}", p),
        None => println!("Perimeter: Not applicable"),
    }
    match shape.volume() {
        Some(v) => println!("Volume: {:.2}", v),
        None => println!("Volume: Not applicable"),
    }
}

fn read_f64(prompt: &str) -> f64 {
    loop {
        let mut input = String::new();
        print!("{}", prompt);
        io::Write::flush(&mut io::stdout()).unwrap();
        io::stdin()
            .read_line(&mut input)
            .expect("Sorry, failed to read line");
        match input.trim().parse() {
            Ok(val) => return val,
            Err(_) => println!("Please enter a valid number."),
        }
    }
}

fn read_u32(prompt: &str) -> u32 {
    loop {
        let mut input = String::new();
        print!("{}", prompt);
        io::Write::flush(&mut io::stdout()).unwrap();
        io::stdin()
            .read_line(&mut input)
            .expect("Sorry, failed to read line");
        match input.trim().parse() {
            Ok(val) => return val,
            Err(_) => println!("Please enter a valid number."),
        }
    }
}
