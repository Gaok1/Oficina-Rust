//Desafio do slide 32
/// 1)   Tipos e Ownership-Borrow: Crie um programa que declare variáveis de tipos primitivos (inteiro, float, unsigned)
///  Em seguida, crie uma função que aceite esses tipos como parâmetros, 
/// modifique-os dentro da função por meio de ponteiros, por fim, 
/// a função deve retornar uma tupla com o valor original e o modificado


fn main() {
    let mut float:f32 = 10.0;
    let mut integer:i32 = 10;
    let mut unsigned:u32 = 10;

    let (old_values, new_values) = modify_values(&mut float, &mut integer, &mut unsigned);
    println!("Old values: {:?}", old_values);
    println!("New values: {:?}", new_values);

}

fn modify_values(float:&mut f32, integer:&mut i32, unsigned:&mut u32) -> ((f32, i32, u32),(f32, i32, u32)) {
    let old_float = *float; * float += 20.0;
    let old_integer = *integer; *integer -= 20;
    let old_unsigned = *unsigned; *unsigned += 20;
    ((old_float, old_integer, old_unsigned), (*float, *integer, *unsigned))
}


