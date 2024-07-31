use std::{env::var, string};

use rand::random;

///Ownership determina até quando um valor pode ser acessado conforme
///  ... o tempo de vida de uma variavel 
/// verifique a funçao lifetime()
fn main() {// ---> lifetime 'a
    let mut pointer:&str;
    let string1 = "hello".to_string();
    
    { //lifetime de string2 começa aq
        let string2 = "outra string".to_string();
        pointer = &string2;
    }// string2 morre e a memória e desalocada

    //neste momento, se Rust me peritisse utilizar pointer 
    // ocorreria um dangling pointer, uma vez q ele aponta para memoria desalocada
 
    println!("{}", pointer);
    // ao adicionar essa linha, o compilador não permmite a atribuiçao de pointer
    //a string2, pois ambos tem lifetime diferentes
    

}



fn lifetime(){ //inicio de lifetime a
    let _a:i32 = 2;

    {//inicio de lifetime b

        let _b:i32 = 10;

        println!("{_a} {_b}");
    } //fim de lifetime de b

    //println!("{_b}"); //erro pois _b não vive nesse escopo
    

}//fim de lifetime de a
