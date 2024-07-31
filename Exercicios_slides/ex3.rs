/// Desafio do slide 72:
/// `1)`   Enums e Pattern Matching: Defina um enum para representar as direções (Norte, Sul, Leste, Oeste) 
/// cada variante com uma unidade de medida 
/// e escreva uma função que receba esse enum e retorne uma mensagem correspondente à direção. 

/// `Desafio adicional`: Adicione um novo enum Vehicle com diferentes tipos de veículos (Carro, Bicicleta, Caminhão) 
/// que devem possuir uma direção do enum acima 
/// e crie uma função que utilize pattern matching para determinar o tipo de veículo e a direção,
///  retornando uma mensagem adequada para cada combinação.

fn main() {

}



enum Direcao{
    Norte(u32),
    Sul(u32),
    Leste(u32),
    Oeste(u32),
}

impl Direcao {
    fn get_message(&self) -> String {
        match self {
            Direcao::Norte(_) => String::from("Norte"),
            Direcao::Sul(_) => String::from("Sul"),
            Direcao::Leste(_) => String::from("Leste"),
            Direcao::Oeste(_) => String::from("Oeste"),
        }
    }
}

enum Veiculo {
    Carro(Direcao),
    Bicicleta(Direcao),
    Caminhao(Direcao),
}

impl Veiculo {
    fn get_message(&self) -> String {
        match self {
            Veiculo::Carro(direcao) => format!("Carro indo para {}", direcao.get_message()),
            Veiculo::Bicicleta(direcao) => format!("Bicicleta indo para {}", direcao.get_message()),
            Veiculo::Caminhao(direcao) => format!("Caminhao indo para {}", direcao.get_message()),
        }
    }
}


