use std::ops::Mul;

///exercicio da pagina 99 do slide
///  1)   Crie 3 estruturas, Triangle, Circle e Square. 
/// crie um Trait chamado Geometry, 
/// o trait deve conter calculo de perimetro, área e volume (deve receber altura de parametro) ]
/// e implemente para cada estrutura. 
/// Além disso, implemente o Display trait para as estruturas.

/// faça uma função que possa receber qualquer tipo que implemente Geometry que printe o objeto, a área, perimetro 
/// e volume do objeto com um parâmetro random() e retorne a String formatada.


struct Triangle{
    base: f32,
    height: f32,
}

struct Circle{
    radius: f32,
}

struct Square{
    side: f32,
}
/// Trait que estruturas geometricas devem implementar
/// contendo métodos para calcular perimetro, área e volume

trait Geometry {
    fn perimeter(&self) -> f32;
    fn area(&self) -> f32;
    fn volume(&self, height: f32) -> f32;
}

impl Geometry for Circle {

    fn perimeter(&self) -> f32 {
        2.0 * std::f32::consts::PI * self.radius
    }

    fn area(&self) -> f32 {
        std::f32::consts::PI * self.radius * self.radius
    }

    fn volume(&self, _height: f32) -> f32 {
        0.0
    }
}

impl Geometry for Triangle {
    fn perimeter(&self) -> f32 {
        self.base + self.height + (self.base.powi(2) + self.height.powi(2)).sqrt()
    }

    fn area(&self) -> f32 {
        (self.base * self.height) / 2.0
    }

    fn volume(&self, height: f32) -> f32 {
        self.area() * height
    }
}


impl Geometry for Square{
    fn perimeter(&self) -> f32 {
        self.side * 4_f32
    }

    fn area(&self) -> f32 {
        self.side * self.side
    }

    fn volume(&self, height: f32) -> f32 {
        self.area() * height
    }
}
//funçao genérica restringida a tipos que implementam Geometry
fn print_geometric_object<T> (object: T, height: f32) -> String
where 
    T: Geometry
     {
        format!("Perimeter: {}\nArea: {}\nVolume: {}", object.perimeter(), object.area(), object.volume(height))
    }



fn main(){

}