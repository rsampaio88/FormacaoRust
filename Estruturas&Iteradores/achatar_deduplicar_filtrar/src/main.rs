fn achatar_deduplicar_filtrar(v: Vec<Vec<u32>>) -> Vec<u32> {
    todo!("Implemente aqui a logica de achatamento, deduplicacao e filtragem")
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