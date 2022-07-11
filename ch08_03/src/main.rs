fn main() {
    let ma_string = String::from("bonjour");
    let ma_string_partage = StringPartage {
        contenu: &ma_string,
    };
    // drop(ma_string);

    println!("ma string partagé : {}", ma_string_partage.contenu);
}

struct StringPartage<'a> {
    contenu: &'a String,
}

impl<'a> StringPartage<'a> {
    // fn creation_dans_fonction() -> StringPartage<'a> {
    //     let valeur_par_defaut = String::from("ma valeur par defaut");

    //     StringPartage {
    //         contenu: &valeur_par_defaut,
    //     }
    // }

    // fn creation_depuis_parametre(valeur_par_defaut: String) -> StringPartage<'a> {
    //     StringPartage {
    //         contenu: &valeur_par_defaut,
    //     }
    // }

    fn creation_depuis_parametre_ref(valeur_par_defaut: &'a String) -> StringPartage<'a> {
        StringPartage {
            contenu: &valeur_par_defaut,
        }
    }
}
