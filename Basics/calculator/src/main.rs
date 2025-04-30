/*
 * =======================================================================
 * Author:     Rita Ferreira
 * File:       main.rs
 * Purpose:    Implementation of a calculator
 * =======================================================================
 */

use std::io;

fn main() {
    let mut input = String::new();

    println!("Insert an operation (for example '1 + 1'):");

    io::stdin().read_line(&mut input).expect("Erro");

    let result = calculator_str(&input);
    println!("Result: {}", result);
}

fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn minus(a: i32, b: i32) -> i32 {
    a - b
}

fn times(a: i32, b: i32) -> i32 {
    a * b
}

fn divide(a: i32, b: i32) -> i32 {
    a / b
}

// trim() : remove whitespaces in the begining and endiing of string
// split_whitesapce() : divides the string by the whitesapces
// colect() : collects in a vector of tsrings
// parse() : convert
// unwrap() : get value or crash.

fn calculator_str(string: &str) -> i32 {
    let parts: Vec<&str> = string.trim().split_whitespace().collect();

    if parts.len() != 3 {
        println!("Ups :(, wrong input.");
        println!("Example of valid input : 1 + 1");
    }
    return calculator_str_list(&parts);
}

fn calculator_str_list(string: &[&str]) -> i32 {
    let num1 = string[0].parse::<i32>().unwrap();
    let num2 = string[2].parse::<i32>().unwrap();

    if string[1] == "+" {
        return add(num1, num2);
    } else if string[1] == "-" {
        return minus(num1, num2);
    } else if string[1] == "*" {
        return times(num1, num2);
    } else if string[1] == "/" {
        return divide(num1, num2);
    } else {
        println!("Invallid Operator.");
        return 0;
    }
}

#[cfg(test)]
pub mod calculator_test {

    #[test]
    fn test_calculator_str() {
        assert_eq!(super::calculator_str("1 + 1"), 2);
        assert_eq!(super::calculator_str("2 * 2"), 4);
        assert_eq!(super::calculator_str("2 / 2"), 1);
        assert_eq!(super::calculator_str("2 - 2"), 0);
    }

    #[test]
    fn test_calculator_str_list() {
        assert_eq!(super::calculator_str_list(&vec!["2", "*", "3"]), 6);
        assert_eq!(super::calculator_str_list(&vec!["2", "+", "3"]), 5);
        assert_eq!(super::calculator_str_list(&vec!["3", "-", "2"]), 1);
        assert_eq!(super::calculator_str_list(&vec!["6", "/", "3"]), 2);
    }
}
