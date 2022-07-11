// Implémenter une fonction générique
use std::fmt::Display;

fn main() {
    let max_usize = max(5usize, 19usize);
    dbg!(max_usize);

    let max_f64 = max(5f64, 19f64);
    dbg!(max_f64);
}

fn max<T>(gauche: T, droite: T) -> T
where
    T: PartialOrd + Display,
{
    println!("Comparaison entre gauche = {gauche} et droite = {droite}");
    if gauche >= droite {
        gauche
    } else {
        droite
    }
}
