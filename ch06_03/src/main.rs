use std::io::Read;

// Améliorer sa gestion d'erreur
#[derive(Debug)]
pub enum MonErreur {
    IoError(std::io::Error),
    Indisponible,
    Autre(String),
}

impl From<std::io::Error> for MonErreur {
    fn from(io_error: std::io::Error) -> Self {
        Self::IoError(io_error)
    }
}

fn ouvrir() -> Result<String, MonErreur> {
    let mut fichier = std::fs::File::open("test")?;
    let mut contenu = String::new();
    fichier.read_to_string(&mut contenu)?;

    Ok(contenu)
}

fn main() {
    match ouvrir() {
        Ok(_) => {}
        Err(err) => eprintln!("une erreur est survenue {err:?}"),
    }
}
