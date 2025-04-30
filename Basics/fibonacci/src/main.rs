/*
 * =======================================================================
 * Author:     Rita Ferreira
 * File:       filter.rs
 * Purpose:    returns the nth fibonacci number
 * =======================================================================
 */


/* 
fn fibonacci(n: u32) -> u32 {

    if n == 0 || n == 1 {
        return n;
    }

    let mut a : u32 = 0; //Fibonacci(0)
    let mut b : u32 = 1; //Fibonacci(1)
    let mut posi : u32 = 2;

    for _ in 2..=n {
        let fib = a + b;
                a = b;
        b = fib;
      }
    b
}*/

//recursive version
fn fibonacci(n: u32) -> u32 {
    if n == 0 {
        return 0;
    } else if n == 1 {
        return 1;
    } else {
        return fibonacci(n - 1) + fibonacci(n - 2);
    }
}

#[cfg(test)]
mod fibonacci_test {

    #[test]
    fn test_fibonacci() {
        assert_eq!(super::fibonacci(0), 0);
        assert_eq!(super::fibonacci(1), 1);
        assert_eq!(super::fibonacci(2), 1);
        assert_eq!(super::fibonacci(3), 2);
        assert_eq!(super::fibonacci(4), 3);
        assert_eq!(super::fibonacci(5), 5);
        assert_eq!(super::fibonacci(6), 8);
        assert_eq!(super::fibonacci(7), 13);
        assert_eq!(super::fibonacci(8), 21);
        assert_eq!(super::fibonacci(9), 34);
        assert_eq!(super::fibonacci(10), 55);
    }
}

fn main() {}
