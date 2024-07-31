/// Desafio do slide 43:
/// 1) String e arrays: Crie um programa que declare e inicialize um vetor de inteiros.
/// Implemente funções que preencham o array e outra função que receba um slice da metade do array original
/// e printe seus elementos na tela.
///
/// Crie um programa que declare e inicialize uma string.
/// Implemente funções que modifiquem essa string e outra função que receba um slice dessa string e o printe.

fn main() {
    // Declara e inicializa um array de inteiros com 10 elementos
    let mut array: [i32; 10] = [0; 10];
    fill_array(&mut array); // Preenche o array com valores de 0 a 9
    print_half_array(&array[0..5]); // Printa a primeira metade do array

    // Declara e inicializa uma string
    let mut string = String::from("Hello");
    modify_string(&mut string); // Modifica a string adicionando um sufixo
    print_slice_string(&string); // Printa o conteúdo da string
}

/// Preenche um array de inteiros com valores de 0 a 9.
fn fill_array(array: &mut [i32; 10]) {
    for i in 0..10 {
        array[i] = i as i32;
    }
}

/// Printa os elementos da primeira metade de um array de inteiros.
fn print_half_array(array: &[i32]) {
    for i in 0..5 {
        println!("{}", array[i]);
    }
}

/// Modifica uma string adicionando o sufixo " - modified".
fn modify_string(string: &mut String) {
    string.push_str(" - modified");
}

/// Printa um slice de uma string.
fn print_slice_string(slice: &str) {
    println!("{}", slice);
}
