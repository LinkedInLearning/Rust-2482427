// Découvrir les différents types de visibilités
mod sous_module;
pub mod sous_module_2;

use sous_module::fonction_crate as sous_fonction_crate;

// pub use mon_module::fonction_publique_crate; // Error
// pub use mon_module::fonction_publique_super; // Error

// Reexports
pub use mon_module::fonction_publique;
pub use sous_module_2::*;

pub struct MaStruct {
    pub(crate) ma_string: String,
    pub mon_numero: usize,
    mon_bool: bool,
}

// sous module
mod mon_module {
    // Import du module parent
    use super::sous_fonction_crate;

    pub fn fonction_publique() {
        // Utilisation avec path complet en partant de crate
        crate::sous_module::fonction_crate()
    }

    pub(crate) fn fonction_publique_crate() {
        // Utilisation de l'import du use
        sous_fonction_crate()
    }

    pub(super) fn fonction_publique_super() {}
}
