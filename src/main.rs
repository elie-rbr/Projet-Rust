// Importe les modules locaux nécessaires
mod file_tree;
mod print_tree;
mod size;

// Importe la macro Parser et Subcommand de la bibliothèque clap
use clap::{Parser, Subcommand};

// Importe FileTree du module file_tree et les types Path et PathBuf du module std::path
use file_tree::FileTree;
use std::path::{Path, PathBuf};

// Définit la structure principale de la ligne de commande avec clap
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

// Définit les sous-commandes disponibles
#[derive(Subcommand)]
enum Commands {
    /// Affiche l'arbre d'utilisation du disque pour le chemin donné
    #[command(name = "Usage")]
    usage(UsageOptions),
}

// Définit les options de la sous-commande "Usage"
#[derive(Parser)]
struct UsageOptions {
    /// (par défaut '.')
    #[arg(long)]
    path: Option<PathBuf>,
    #[arg(long)]
    lexicographic_sort: bool,
    #[arg(long, short = 'e')]
    file_extension: Option<String>,
}

// Fonction principale
fn main() -> std::io::Result<()> {
    // Parse les arguments de ligne de commande
    let cli = Cli::parse();

    // Effectue des actions en fonction de la sous-commande spécifiée
    match &cli.command {
        Commands::usage(usage_options) => {
            let path = usage_options.path.as_deref().unwrap_or(Path::new("."));
            let lexicographic_sort = usage_options.lexicographic_sort;
            let file_extension = usage_options.file_extension.as_deref();
            FileTree::new(path)?.show(lexicographic_sort, file_extension);
        }
    }
    Ok(())
}