use crate::file_tree::{FileTree, EntryNode};
use std::path::{Path, PathBuf};

impl FileTree {
    // Fonction publique pour afficher l'arbre
    pub fn show(&self, lexicographic_sort: bool, file_extension: Option<&str>) {
        let root = self.get_root();  // Obtient la racine de l'arbre
        self.display_tree(&root, 0, lexicographic_sort, file_extension);  // Appelle la fonction récursive pour afficher l'arbre
    }

    // Fonction récursive pour afficher l'arbre
    fn display_tree(
        &self,
        current_path: &Path,
        depth: usize,
        lexicographic_sort: bool,
        file_extension: Option<&str>,
    ) {
        // Vérifie s'il y a des enfants pour le chemin actuel
        if let Some(children) = self.get_children(current_path, lexicographic_sort, file_extension) {
            // Obtient la taille du répertoire actuel
            let size = self.get_size(&current_path);
            // Calcule l'indentation en fonction de la profondeur
            let indentation = "  ".repeat(depth);
            // Affiche le répertoire actuel avec sa taille
            println!(
                "{} Directory: {} ({})",
                indentation,
                current_path.to_string_lossy(),
                size.map_or_else(|| "N/A".to_string(), |s| s.to_string())
            );

            // Parcourt les enfants et affiche leurs informations
            for child_path in children.iter() {
                if child_path.is_dir() {
                    // Si l'enfant est un répertoire, affiche ses informations
                    let size = self.get_size(&child_path);
                    let indentation = "  ".repeat(depth + 1);
                    println!(
                        "{} Directory: {} ({})",
                        indentation,
                        child_path.to_string_lossy(),
                        size.map_or_else(|| "N/A".to_string(), |s| s.to_string())
                    );

                    // Récupère les fichiers du sous-répertoire et les affiche
                    let sub_files = self.files(child_path.to_path_buf());
                    for file in sub_files {
                        let indentation = "  ".repeat(depth + 2);
                        let file_size = self.get_size(&file);
                        println!(
                            "{} File: {} ({})",
                            indentation,
                            file.to_string_lossy(),
                            file_size.map_or_else(|| "N/A".to_string(), |s| s.to_string())
                        );
                    }
                } else if child_path.is_file() {
                    // Si l'enfant est un fichier, affiche ses informations
                    let indentation = "  ".repeat(depth + 1);
                    let child_size = self.get_size(&child_path);
                    println!(
                        "{} File: {} ({})",
                        indentation,
                        child_path.to_string_lossy(),
                        child_size.map_or_else(|| "N/A".to_string(), |s| s.to_string())
                    );
                } else {
                    // Cas où l'enfant n'est ni un répertoire ni un fichier
                    println!(" ");
                }
            }
        }
    }
}
