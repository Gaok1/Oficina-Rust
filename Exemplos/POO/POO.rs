use std::fmt::Display;

/// Uma estrutura genérica que armazena um valor genérico e um código.
/// `Shulambs` é o tipo genérico, que deve implementar `Display` e `Clone`.
struct Generica<Shulambs>
where
    Shulambs: Display + Clone,
{
    generico: Shulambs,
    code: u32,
}

/// Uma trait que define o comportamento de um Animal.
pub trait Animal {
    /// Obtém o nome do animal.
    fn get_name(&self) -> String;

    /// Faz o animal emitir um som. Por padrão, retorna uma mensagem genérica.
    fn make_sound(&self) -> String {
        "This animal is making a sound!".to_string()
    }

    /// Verifica se o animal está morto.
    fn is_dead(&self) -> bool;

    /// Mata o animal, alterando seu estado para morto.
    fn kill(&mut self);
}

/// Uma estrutura que representa um cachorro.
pub struct Dog {
    name: String,
    age: i32,
    is_alive: bool,
}

impl Dog {
    /// Cria uma nova instância de `Dog`.
    pub fn new(name: String, age: i32) -> Self {
        Dog {
            name,
            age,
            is_alive: true,
        }
    }
}

impl Animal for Dog {
    fn get_name(&self) -> String {
        self.name.clone()
    }

    fn is_dead(&self) -> bool {
        !self.is_alive
    }

    fn kill(&mut self) {
        self.is_alive = false;
    }
}

impl Display for Dog {
    /// Implementa `Display` para `Dog` para permitir a formatação.
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{} - {} years old, ", self.name, self.age).and_then(|_| {
            match self.is_alive {
                true => write!(f, "alive and good!"),
                false => write!(f, "dead :("),
            }
        })
    }
}

/// Uma estrutura que representa um gato.
pub struct Cat {
    name: String,
    age: i32,
    is_alive: bool,
}

impl Cat {
    /// Cria uma nova instância de `Cat`.
    pub fn new(name: String, age: i32) -> Self {
        Cat {
            name,
            age,
            is_alive: true,
        }
    }
}

impl Display for Cat {
    /// Implementa `Display` para `Cat` para permitir a formatação.
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{} - {} years old, ", self.name, self.age).and_then(|_| {
            match self.is_alive {
                true => write!(f, "alive and good!"),
                false => write!(f, "dead :("),
            }
        })
    }
}

impl Animal for Cat {
    fn get_name(&self) -> String {
        self.name.clone()
    }

    fn make_sound(&self) -> String {
        "Miau".to_string()
    }

    fn is_dead(&self) -> bool {
        !self.is_alive
    }

    fn kill(&mut self) {
        self.is_alive = false;
    }
}

/// Faz um animal emitir seu som característico.
fn make_sound(animal: &dyn Animal) -> String {
    animal.make_sound()
}

/// Mata todos os animais em uma coleção.
fn kill_all<T>(manada: &mut [T])
where
    T: Animal,
{
    for animal in manada {
        animal.kill();
    }
}

fn main() {
    // Criando instâncias de Dog e Cat
    let mut dog = Dog::new("Rex".to_string(), 5);
    let mut cat = Cat::new("Whiskers".to_string(), 3);

    // Exibindo informações e sons dos animais
    println!("Dog: {}", dog);
    println!("Cat: {}", cat);

    println!("Dog sound: {}", make_sound(&dog));
    println!("Cat sound: {}", make_sound(&cat));

    // Criando um vetor de animais e matando todos
    let mut animals: Vec<&mut dyn Animal> = vec![&mut dog, &mut cat];
    kill_all(&mut animals);

    // Exibindo o estado dos animais após a chamada de kill_all
    for animal in &animals {
        println!(
            "{} is dead: {}",
            animal.get_name(),
            if animal.is_dead() { "Yes" } else { "No" }
        );
    }
}
