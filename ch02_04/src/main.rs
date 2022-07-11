use std::collections::HashMap;

fn main() {
    let mut mon_vecteur = vec![2usize; 5];
    let mon_autre_vec: Vec<usize> = vec![1, 2, 3, 4];
    println!("egalite ? {}", mon_vecteur == mon_autre_vec);

    println!("mon_vecteur avant dedup {:?}", mon_vecteur);
    mon_vecteur.dedup();
    println!("mon_vecteur après dedup {:?}", mon_vecteur);
    mon_vecteur.push(5);
    mon_vecteur.push(5);
    mon_vecteur.push(5);
    mon_vecteur.push(5);
    mon_vecteur.push(6);

    println!("mon_autre_vec {:#?}", mon_autre_vec);
    for mon_element in mon_autre_vec.iter().skip(2).rev() {
        println!("mon element {mon_element}");
    }

    let mut animaux: HashMap<String, i32> = HashMap::new();
    animaux.insert("chien".to_string(), 42);
    animaux.insert("chat".to_string(), 3);

    println!(
        "nombre de chats = {}",
        animaux.get(&"chat".to_string()).unwrap()
    );

    // ---------------------------
    for (animal, nombre) in animaux {
        println!("animal '{animal}' = {nombre}");
    }
}
