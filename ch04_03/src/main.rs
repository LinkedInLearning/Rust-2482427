// Implémenter un trait sur une struct
struct Rectangle {
    largeur: f64,
    longueur: f64,
}

trait Aire {
    fn aire(&self) -> f64;
    fn double_aire(&self) -> f64 {
        self.aire() * 2.0
    }
}

impl Aire for Rectangle {
    fn aire(&self) -> f64 {
        self.largeur * self.longueur
    }
}

impl Aire for String {
    fn aire(&self) -> f64 {
        42.0
    }
}

fn main() {
    let rectangle = Rectangle {
        largeur: 2.0,
        longueur: 3.0,
    };
    println!("Aire du rectangle = {}", rectangle.aire());
    println!("Double aire du rectangle = {}", rectangle.double_aire());
    println!("Aire d'une String = {}", String::from("ma string").aire());
}
