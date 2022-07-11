// &mut T = &mut Vec<String> → On détient une référence mutable EXCLUSIVE du vecteur

fn main() {
    let mon_vec = vec!["ceci", "est", "un", "test"];

    for elt in &mon_vec {
        println!("elt : {}", elt);
        // mon_vec.push("!");
    }

    println!("{:?}", mon_vec);
}
