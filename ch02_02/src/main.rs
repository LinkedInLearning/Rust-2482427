fn main() {
    // Toujours UTF8
    let ma_string = String::from("ma string");
    let ma_str = "ma str";

    let mut ma_format = format!("ceci est une string avec ma_str {ma_string}");
    ma_format.push_str("!!!");
    ma_format.push('!');

    let ma_string_format_str = ma_string.as_str();

    let mon_caractere = ma_string.chars().nth(2).unwrap();
    for lettre in ma_string_format_str.chars() {
        println!("lettre {lettre}");
    }
}

//                      buffer
//                    /   capacity
//                  /   /  length
//                /   /   /
//             +–––+–––+–––+
// stack frame │ • │ 16 │ 9 │ <- ma_string: String
//             +–│–+–––+–––+
//               │
//             [–│–––––––– capacity –––––––––––––––––––––––––––––––––––––––––––]
//               │
//             +–V–+–––+–––+–––+–––+–––+–––+–––+–––+–––+–––+–––+–––+–––+–––+–––+
//        heap │ m │ a │   │ s │ t │ r │ i │ n │ g │   │   │   │   │   │   │   │
//             +–––+–––+–––+–––+–––+–––+–––+–––+–––+–––+–––+–––+–––+–––+–––+–––+
//             [––––––– length ––––––––––––––––––––]

//             ma_str: &str
//             [–––––––––––]
//             +–––+–––+
// stack frame │ • │ 6 │
//             +–│–+–––+
//               │
//               +––+
//                  │
//  preallocated  +–V–+–––+–––+–––+–––+–––+
//  read-only     │ m │ a │   │ s │ t │ r │
//  memory        +–––+–––+–––+–––+–––+–––+
