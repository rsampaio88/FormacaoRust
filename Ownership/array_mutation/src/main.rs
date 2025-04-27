fn array_mut_ownership(array: [u32; 5], operation: char, other_member: u32) -> [u32; 5] {
    let mut new = array;

    for i in 0..5 {
        new[i] = match operation {
            '+' => new[i] + other_member,
            '-' => new[i] - other_member,
            '*' => new[i] * other_member,
            '/' => new[i] / other_member,
            _ => new[i],
        }
    }
    new
}

fn array_mut_mut(array: &mut [u32], operation: char, other_member: u32) {
    for i in 0..array.len() {
        array[i] = match operation {
            '+' => array[i] + other_member,
            '-' => array[i] - other_member,
            '*' => array[i] * other_member,
            '/' => array[i] / other_member,
            _ => array[i],
        }
    }
}

#[cfg(test)]
mod array_mutation_test {

    const OWNERSHIP_TEST_ARRAY: [u32; 5] = [1, 2, 3, 4, 5];

    #[test]
    fn test_ownership_mutation() {
        assert_eq!(
            super::array_mut_ownership(OWNERSHIP_TEST_ARRAY, '+', 1),
            [2, 3, 4, 5, 6]
        );
        assert_eq!(
            super::array_mut_ownership(OWNERSHIP_TEST_ARRAY, '-', 1),
            [0, 1, 2, 3, 4]
        );
        assert_eq!(
            super::array_mut_ownership(OWNERSHIP_TEST_ARRAY, '*', 2),
            [2, 4, 6, 8, 10]
        );
        assert_eq!(
            super::array_mut_ownership(OWNERSHIP_TEST_ARRAY, '/', 2),
            [0, 1, 1, 2, 2]
        );
    }

    #[test]
    fn test_mut_ref_mutation() {
        let mut array = OWNERSHIP_TEST_ARRAY.clone();

        super::array_mut_mut(&mut array, '+', 1);

        assert_eq!(array, [2, 3, 4, 5, 6]);

        let mut array = OWNERSHIP_TEST_ARRAY.clone();

        super::array_mut_mut(&mut array, '-', 1);

        assert_eq!(array, [0, 1, 2, 3, 4]);

        let mut array = OWNERSHIP_TEST_ARRAY.clone();

        super::array_mut_mut(&mut array, '*', 2);

        assert_eq!(array, [2, 4, 6, 8, 10]);

        let mut array = OWNERSHIP_TEST_ARRAY.clone();

        super::array_mut_mut(&mut array, '/', 2);

        assert_eq!(array, [0, 1, 1, 2, 2]);
    }
}

fn main() {}
