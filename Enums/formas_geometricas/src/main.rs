/*
 * =======================================================================
 * Author:     Rita Ferreira
 * File:       main.rs
 * Purpose:    explore geometric shapes and compute their properties.
 * =======================================================================
 */

use std::io;
//use clearscreen::clear;
use shapes::{Circle, Cube, Cylinder, RegularPolygon, Shape, Sphere, Square};

fn main() {
    geometric_shapes();
}

fn geometric_shapes() {
    loop {
        clear().unwrap();
        println!(
            "Choose a geometric shape:
1) Square
2) Circle
3) Cube
4) Cylinder
5) Sphere
6) Regular Polygon
7) Exit"
        );

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        let choice: u32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input. Please enter a number.");
                continue;
            }
        };

        let shape = match choice {
            1 => {
                println!("Enter the side length of the square:");
                let side = read_f64();
                Shape::Square { side }
            }
            2 => {
                println!("Enter the radius of the circle:");
                let radius = read_f64();
                Shape::Circle { radius }
            }
            3 => {
                println!("Enter the side length of the cube:");
                let side = read_f64();
                Shape::Cube { side }
            }
            4 => {
                println!("Enter the radius of the cylinder:");
                let radius = read_f64();
                println!("Enter the height of the cylinder:");
                let height = read_f64();
                Shape::Cylinder { radius, height }
            }
            5 => {
                println!("Enter the radius of the sphere:");
                let radius = read_f64();
                Shape::Sphere { radius }
            }
            6 => {
                println!("Enter the number of sides for the regular polygon:");
                let sides = read_u32();
                println!("Enter the length of each side:");
                let length = read_f64();
                Shape::RegularPolygon { sides, length }
            }
            7 => break,
            _ => {
                println!("Invalid option, please try again.");
                continue;
            }
        };

        match shape.area() {
            Some(area) => println!("Area: {:.2}", area),
            None => println!("Area: Not applicable"),
        }

        match shape.perimeter() {
            Some(perimeter) => println!("Perimeter: {:.2}", perimeter),
            None => println!("Perimeter: Not applicable"),
        }

        match shape.volume() {
            Some(volume) => println!("Volume: {:.2}", volume),
            None => println!("Volume: Not applicable"),
        }
        // {:.2} F  Some(36.0)  FIX -> 36.0
        press_to_continue();
    }
}

fn press_to_continue() {
    println!("\nPress Enter to continue...");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
}

fn read_f64() -> f64 {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    input.trim().parse().expect("Invalid number")
}

fn read_u32() -> u32 {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    input.trim().parse().expect("Invalid number")
}
