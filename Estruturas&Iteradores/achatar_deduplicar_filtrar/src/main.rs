/*
 * =======================================================================
 * Author:     Rita Ferreira
 * File:       main.rs
 * Purpose:    Implementation of the logic for flattening,
 *            deduplication, and filtering
 * =======================================================================
 */

/*

    .into_iter(): creates an iterator Vec<u32> for the Vec<Vec<u32>>
    .flat_map(): maps acordding the iterator
    .inner_vec.into_iter( : will transform   vec![vec![1, 2], vec![3, 4]] in vec![1, 2, 3, 4]
    .filter(): returns true or false acording condition
    .filter(|&x| x % 2 == 0 || x % 3 == 0)  :
        takes the u32 value , if divided by 3 or 2 the rest is 0
    .collect : collects the values of x that fits the filter
    HashSet : no duplicates
*/

use std::collections::HashSet;

#[warn(dead_code)]
fn achatar_deduplicar_filtrar(v: Vec<Vec<u32>>) -> Vec<u32> {
    v.into_iter()
        .flat_map(|inner_vec| inner_vec.into_iter())
        .filter(|&x| x % 2 == 0 || x % 3 == 0)
        .collect::<HashSet<_>>()
        .into_iter()
        .collect()
}

fn main() {}

#[cfg(test)]
mod achatar_deduplicar_filtrar_test {
    use std::collections::HashSet;

    #[test]
    fn test_func() {
        let vec = vec![vec![1, 2, 3], vec![3, 4, 5], vec![5, 6, 7]];

        let result = super::achatar_deduplicar_filtrar(vec);

        assert!(result.iter().all(|x| x % 2 == 0 || x % 3 == 0));

        let mut seen = HashSet::new();

        assert!(result.iter().all(|x| seen.insert(x)));
    }
}
