enum Operation {
    Add(i32, i32),
    Subtract(i32, i32),
    Multiply(i32, i32),
    Divide(i32, i32),
}

fn main() {
    let addition = Operation::Add(10, 5);
    let subtraction = Operation::Subtract(20, 10);
    let multiplication = Operation::Multiply(3, 4);
    let division = Operation::Divide(10, 2);

    // Usando `match` para diferenciar as operações
    match addition {
        Operation::Add(a, b) => println!("Soma: {} + {} = {}", a, b, a + b),
        Operation::Subtract(a, b) => println!("Subtração: {} - {} = {}", a, b, a - b),
        Operation::Multiply(a, b) => println!("Multiplicação: {} * {} = {}", a, b, a * b),
        Operation::Divide(a, b) => println!("Divisão: {} / {} = {}", a, b, a / b),
    }

    // Usando `if let` para uma operação específica
    if let Operation::Divide(a, b) = division {
        println!("Resultado da divisão: {}", a / b);
    }

    // Usando `while let` para iterar sobre uma lista de operações
    let mut operations = vec![
        Operation::Add(1, 2),
        Operation::Subtract(5, 3),
        Operation::Multiply(2, 2),
        Operation::Divide(9, 3),
    ];

    // Processando a lista de operações
    while let Some(op) = operations.pop() {
        match op {
            Operation::Add(a, b) => println!("Add: {} + {} = {}", a, b, a + b),
            Operation::Subtract(a, b) => println!("Subtract: {} - {} = {}", a, b, a - b),
            Operation::Multiply(a, b) => println!("Multiply: {} * {} = {}", a, b, a * b),
            Operation::Divide(a, b) => println!("Divide: {} / {} = {}", a, b, a / b),
        }
    }
}
