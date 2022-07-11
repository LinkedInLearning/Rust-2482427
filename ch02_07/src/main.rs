// fonctions

fn main() {
    let somme = additionne(5, 20);
    let division = divise(5, 0);

    println!("double tuple -- {:?}", double_tuple((5, 255)));

    let reponse = execute(additionne);
    dbg!(reponse);
}

fn additionne(left: i32, right: i32) -> i32 {
    // Tail expression
    left + right
}

fn divise(left: usize, right: usize) -> f64 {
    if right == 0 {
        return 0.0;
    }

    left as f64 / right as f64
}

fn double_tuple(val: (usize, i32)) -> (usize, i32) {
    (val.0 * 2, val.1 * 2)
}

fn execute(fonction: fn(i32, i32) -> i32) -> i32 {
    fonction(1, 3)
}
