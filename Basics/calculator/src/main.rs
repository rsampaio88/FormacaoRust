/// Desafio da calculadora
/// Pode ser implementado com recurso à analise de apenas
/// uma string ou com cada elemento separado na sua propria string.
/// Podem ver exemplos de como vai ser utilizada a função nos testes disponíveis.
///
/// Devem apenas implementar uma das funções.
///
/// Podem comentar a função que não vão implementar para não haver problemas de compilação, incluindo os testes
/// para a mesma.

fn main() {
    todo!("Implementar a leitura do stdin")
}


fn calculator_str(string: &str) -> i32 {
    todo!("Implementar a calculadora que de uma string calcule o resultado")
}

fn calculator_str_list(string: &[&str]) -> i32 {
    todo!("Implementar a calculadora que de uma string ou de uma lista de strings calcule o resultado")
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
        assert_eq!(super::calculator_str_list(&vec!["3", "-", "2"]), 0);
        assert_eq!(super::calculator_str_list(&vec!["6", "/", "3"]), 2);
    }

}