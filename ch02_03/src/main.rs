fn main() {
    let mut tableau = [0, 1, 2, 3];
    let mut slice = &mut tableau[..];
    slice.fill(42);
    println!("tableau => {tableau:#?}");
    println!("est égal ? {}", tableau == [1, 2, 3, 4]);
    let mut tableau_bis = [0, 1, 2, 3];
    slice = &mut tableau_bis[..];

    for element in tableau {
        println!("element du tableau: {}", element);
    }
}
