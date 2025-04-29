#[warn(dead_code)]
fn bubble_sort<T: Ord>(v: &mut Vec<T>) {
    let mut change = true;

    while change {
        change = false;
        for i in 0..v.len() - 1 {
            if v[i] > v[i + 1] {
                v.swap(i, i + 1);
                change = true;
            }
        }
    }
}

fn select_sort<T: Ord>(v: &mut Vec<T>) {
    let len = v.len();

    for i in 0..len - 1 {
        let mut min = i;

        for j in i + 1..len {
            if v[j] < v[min] {
                min = j;
            }
        }
        v.swap(i, min);
    }
}

#[cfg(test)]
mod sort_tests {
    use super::*;

    fn is_sorted<T: Ord>(v: &[T]) -> bool {
        v.windows(2).all(|w| w[0] <= w[1])
    }

    #[test]
    fn test_bubble_sort() {
        let mut v = vec![5, -2, 11, 901, 29];
        bubble_sort(&mut v);
        assert!(is_sorted(&v));

        let mut v = vec![1, 2, 8, 78, 45, 0, -9, -22, 9, 8, 3, 4, 5];
        bubble_sort(&mut v);
        assert!(is_sorted(&v));

        let mut v = vec![1, 2, 3, 4, 5];
        bubble_sort(&mut v);
        assert!(is_sorted(&v));
    }

    #[test]
    fn test_select_sort() {
        let mut v = vec![5, -2, 11, 901, 29];
        select_sort(&mut v);
        assert!(is_sorted(&v));

        let mut v = vec![5, -2, 11, 901, 29];
        select_sort(&mut v);
        assert!(is_sorted(&v));

        let mut v = vec![1, 2, 3, 4, 5];
        select_sort(&mut v);
        assert!(is_sorted(&v));
    }

    #[test]
    fn test_bubble_equals_selection() {
        let mut v1 = vec![9, 7, 5, 3, 1];
        let mut v2 = v1.clone();

        bubble_sort(&mut v1);
        select_sort(&mut v2);

        assert!(v1 == v2);
    }
}

fn main() {
    let mut vector = vec![1, 9, 0, 26, 79, 7, 4];
    //bubble_sort(&mut vector);
    select_sort(&mut vector);
    println!("{:?}", vector);
}
