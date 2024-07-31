fn find_element(arr: &[i32], target: i32) -> Option<usize> {
    // Itera sobre o array `arr` com índices e valores.
    for (index, &value) in arr.iter().enumerate() {
        // Se o valor for igual ao `target`, retorna o índice dentro de Some().
        if value == target {
            return Some(index);
        }
    }
    // Se o loop terminar sem encontrar o `target`, retorna None.
    None
}

fn main() {
    let numbers = [10, 20, 30, 40, 50];
    
    // Busca pelo número 30 no array.
    match find_element(&numbers, 30) {
        // Se encontrar, imprime o índice.
        Some(index) => println!("Elemento encontrado na posição: {}", index),
        // Se não encontrar, imprime que não foi encontrado.
        None => println!("Elemento não encontrado"),
    }
    
    // Busca por um número que não está no array, por exemplo, 100.
    match find_element(&numbers, 100) {
        Some(index) => println!("Elemento encontrado na posição: {}", index),
        None => println!("Elemento não encontrado"),
    }
}
