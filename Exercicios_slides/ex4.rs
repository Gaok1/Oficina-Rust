/// Desafio do slide 76:
/// 1- Crie uma função que realize uma divisão de 2 números de ponto flutuante.
///  Mas caso o denominador seja 0, então a divisão não deve ocorrer e um valor nulo deve ser enviado

fn division(num1: f32, num2: f32) -> Option<f32> {
    if num2 == 0.0 {
        None
    } else {
        Some(num1 / num2)
    }
}

///2- Implemente uma função chamada find_first_even que recebe um slice de números inteiros 
/// e retorna uma referência para o primeiro numero par, caso não haja numeros pares um valor nulo deve ser enviado.
fn find_first_even(numbers: &[i32]) -> Option<&i32> {
    for number in numbers {
        if number % 2 == 0 {
            return Some(number);
        }
    }
    None
}

fn main() {

}