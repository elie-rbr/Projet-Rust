use crate::size::Size;
use crate::size;
use std::fs;
use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};
use std::io::{Error, ErrorKind, Result};
use crypto_hash::{hex_digest, Algorithm};

pub struct FileTree {
    root: PathBuf,
    pub map: HashMap<PathBuf, EntryNode>,
}

#[allow(dead_code)]
pub enum EntryNode {
    File(Size),
    Directory(Size),
}

pub fn calculate_size(chemin: &Path) -> Result<u64> {
    if !chemin.is_file() {
        return Err(Error::new(ErrorKind::InvalidInput, "Not a file"));
    }
    let chemin = chemin.to_path_buf();
    let file_size: u64 = fs::metadata(&chemin)?.len();
    Ok(file_size)
}

pub fn calculate_directory_size(directory: &Path) -> Result<u64> {
    if !directory.is_dir() {
        return Err(Error::new(ErrorKind::InvalidInput, "Not a directory"));
    }

    let mut total_size: u64 = 0;

    for entry in fs::read_dir(directory)? {
        let entry = entry?;
        let path: PathBuf = entry.path();

        if path.is_dir() {
            // Recursive call for subdirectories
            total_size += calculate_directory_size(&path)?;
        } else {
            // Add the size of the file
            total_size += fs::metadata(&path)?.len();
        }
    }

    Ok(total_size)
}


impl FileTree {
    // creation d'un nouvel arbre de fichier
    pub fn new(root: &Path) -> Result<Self> {
            let root = root.to_path_buf() ; // mettre root en PathBuf pour faciliter la manipulation
            // Verifier si root est un repertoire , sinon afficher une erreur
            if root.is_dir() {
                let size = size::Size(calculate_directory_size(&root).unwrap());

                let mut map = HashMap::new(); // stocker les nœuds de l'arbre 
                //let root_path = root.as_path(); // Obtenir une référence &Path du chemin racine.
                let root_size = Size::new(size.0).map_err(|e| {
                    Error::new(ErrorKind::InvalidData, format!("Invalid size: {}", e))
                })?;
                let root_node = EntryNode::Directory(root_size); // Créeer le noeud racine
                map.insert(root.clone(), root_node);
        
                Ok(FileTree { root, map }) // Retourne une nouvelle instance avec le chemin racine et la HashMap
            } else {
                Err(Error::new(ErrorKind::InvalidInput, "Le chemin racine n'est pas un répertoire"))
            }
    }

    pub fn get_root(&self) -> &PathBuf {
        &self.root
    }

    pub fn get_children(&self, path: &Path, lexicographic_sort: bool, file_extension: Option<&str>) -> Option<Vec<PathBuf>> {
            if let Some(node) = self.map.get(path) {
            //let mut vec: Vec<PathBuf> = vec![];
            match node {
                EntryNode::File(_) => Some(vec![]),
                EntryNode::Directory(_) => {
                    let children: Vec<PathBuf> = fs::read_dir(path)
                        .ok()?
                        .filter_map(|entry| {
                            entry.ok().map(|e| e.path())
                        })
                        .filter(|child| child.is_dir())
                        .collect();
                        
                    let mut normalized_children: Vec<PathBuf> = children
                        .iter()
                        .map(|child| child.strip_prefix(".").unwrap_or(child))
                        .map(|child| child.to_path_buf())
                        .collect();

                    let files = self.files(path.to_path_buf());
                    for file in files {
                        normalized_children.push(file)
                    }
                    
                    // Apply filtering based on file extension
                    if let Some(file_extension) = file_extension {
                        normalized_children.retain(|child| {
                            child.extension().map_or(false, |ext| ext == file_extension)
                        });
                    }

                    // Apply sorting
                    if lexicographic_sort {
                        normalized_children.sort();

                    // sort the children in increasing order
                    // so la fonction dispay_tree() de print_tree.rs
                    // can print them in decreasing order
                    } else {
                        normalized_children.sort_by(|a, b| {
                            let size_a = self.get_size(a).unwrap().0;
                            let size_b = self.get_size(b).unwrap().0;
                            size_a.cmp(&size_b)
                        });
                    } 
                    //return the children
                    Some(normalized_children)
                }
            }
        } else {
            None
        }
    }
    

    pub fn get_size(&self, path: &Path) -> Option<Size> {
        if path.is_file() {
            let size = calculate_size(path).unwrap() ;
            Some(size::Size(size))
        }
        else if path.is_dir() {
            let size = calculate_directory_size(path).unwrap();
            Some(size::Size(size))
        }
        else {
            None
        }
    }
    

    pub fn files(&self, path: PathBuf) -> Vec<PathBuf> {
        let mut vec: Vec<PathBuf> = vec![];
        if let Ok(paths) = fs::read_dir(path) {
            for entry in paths {
                if let Ok(entry) = entry {
                    if entry.file_type().map_or(false, |t| t.is_file()) {
                        let entry = entry.path();
                        vec.push(PathBuf::from(entry));
                        }
                    }
                }
            }
        vec
    }


    
    pub fn doublons(&self, path: &Path) -> HashMap<String, Vec<PathBuf>> {
        let mut seen_files: HashSet<Vec<u8>> = HashSet::new();
        let mut duplicates: HashMap<String, Vec<PathBuf>> = HashMap::new();

        self.check_duplicates_recursive(path, &mut seen_files, &mut duplicates);

        duplicates
    }

    fn check_duplicates_recursive(
        &self,
        path: &Path,
        seen_files: &mut HashSet<Vec<u8>>,
        duplicates: &mut HashMap<String, Vec<PathBuf>>,
    ) {
        if let Some(children) = self.get_children(path, false, None) {
            for child in children {
                match self.map.get(&child) {
                    Some(EntryNode::File(_)) => {
                        if let Ok(file_content) = fs::read(&child) {
                            if seen_files.contains(&file_content) {
                                let entry = duplicates.entry(hex_digest(Algorithm::MD5, &file_content))
                                    .or_insert_with(Vec::new);
                                entry.push(child);
                            } else {
                                seen_files.insert(file_content);
                            }
                        }
                    }
                    Some(EntryNode::Directory(_)) => {
                        // Recursive call
                        self.check_duplicates_recursive(&child, seen_files, duplicates);
                    }
                    None => (),
                }
            }
        }
    }
}



#[cfg(test)]
mod tests {
    use super::*; 
    use std::fs::{self, File};
    use std::io::Write;
    use std::env;

    // Helper function to create a temporary directory and return its path
    fn create_temp_dir() -> PathBuf {
        let mut dir = env::temp_dir();
        dir.push("test_dir");
        fs::create_dir_all(&dir).expect("Failed to create temp dir");
        dir
    }

    // Helper function to create a temporary file with content
    fn create_temp_file(dir: &Path, name: &str, content: &[u8]) -> PathBuf {
        let mut file_path = dir.to_path_buf();
        file_path.push(name);
        let mut file = File::create(&file_path).expect("Failed to create temp file");
        file.write_all(content).expect("Failed to write to temp file");
        file_path
    }

    #[test]
    fn test_doublons_with_content_comparison() {
        // Create a temporary directory and files for testing
        let temp_dir = create_temp_dir();
        let file1_path = create_temp_file(&temp_dir, "file1.txt", b"content");
        let file2_path = create_temp_file(&temp_dir, "file2.txt", b"content");
        let file3_path = create_temp_file(&temp_dir, "file3.txt", b"different_content");

        // Create a FileTree instance
        let file_tree = FileTree::new(&temp_dir).expect("Failed to create FileTree");

        // Test the doublons method
        let duplicates = file_tree.doublons(&temp_dir);

        // Verify the results
        assert_eq!(duplicates.len(), 1);
        assert!(duplicates.contains_key(&hex_digest(Algorithm::MD5, b"content")));

        let duplicate_files = duplicates.get(&hex_digest(Algorithm::MD5, b"content")).unwrap();
        assert_eq!(duplicate_files.len(), 2);
        assert!(duplicate_files.contains(&file1_path));
        assert!(duplicate_files.contains(&file2_path));
        assert!(!duplicate_files.contains(&file3_path));
    }

    #[test]
    fn test_doublons_without_content_comparison() {
        // Create a temporary directory and files for testing
        let temp_dir = create_temp_dir();
        let file1_path = create_temp_file(&temp_dir, "file1.txt", b"content");
        let file2_path = create_temp_file(&temp_dir, "file2.txt", b"different_content");

        // Create a FileTree instance
        let file_tree = FileTree::new(&temp_dir).expect("Failed to create FileTree");

        // Test the doublons method
        let duplicates = file_tree.doublons(&temp_dir);

        // Verify the results
        assert!(duplicates.is_empty());
    }
 
    #[test]
    fn test_create_file_tree_with_valid_directory() {
        let root_path = Path::new("test_directory");
        fs::create_dir(root_path).unwrap(); // Create a test directory

        let file_tree_result = FileTree::new(root_path);

        assert!(file_tree_result.is_ok());

        fs::remove_dir(root_path).unwrap(); // Remove the test directory
    }

    #[test]
    fn test_create_file_tree_with_invalid_directory() {
        let root_path = Path::new("nonexistent_directory");

        let file_tree_result = FileTree::new(root_path);

        assert!(file_tree_result.is_err());
    }

    #[test]
    fn test_get_root() {
        let root_path = Path::new("test_directory");
        fs::create_dir(root_path).unwrap(); // Create a test directory

        let file_tree = FileTree::new(root_path).unwrap();

        assert_eq!(file_tree.get_root(), root_path);

        fs::remove_dir(root_path).unwrap(); // Remove the test directory
    }

    #[test]
    fn test_get_size() {
        let root_path = Path::new("test_directory");
        fs::create_dir(root_path).unwrap(); // Create a test directory

        let file_tree = FileTree::new(root_path).unwrap();

        let size = file_tree.get_size(root_path);
        assert!(size.is_some());

        fs::remove_dir(root_path).unwrap(); // Remove the test directory
    }

    #[test]
    fn test_get_files() {
        let root_path = Path::new("test_directory");
        fs::create_dir(root_path).unwrap(); // Create a test directory
        let file_path: PathBuf = root_path.join("test_file.txt");
        let file_path_2: PathBuf = root_path.join("test_file_2.txt");

        fs::File::create(&file_path).unwrap(); // Create a test file in the directory
        fs::File::create(&file_path_2).unwrap(); // Create a test file in the directory

        let file_tree = FileTree::new(root_path).unwrap();

        let files = file_tree.files(root_path.to_path_buf());
        assert_eq!(files, vec![PathBuf::from("test_file_2.txt"), PathBuf::from("test_file.txt")]);

        fs::remove_file(file_path).unwrap(); // Remove the test files
        fs::remove_file(file_path_2).unwrap(); // Remove the test files
        fs::remove_dir(root_path).unwrap(); // Remove the test directory
    }

    #[test]
    fn test_get_children() {
        let root_path = Path::new("test_directory");
        fs::create_dir(root_path).unwrap(); // Create a test directory
        let file_path: PathBuf = root_path.join("test_file.txt");
        let file_path_2: PathBuf = root_path.join("test_file_2.txt");
        let dir_path: PathBuf = root_path.join("test_dir");
        let file_path_3: PathBuf = root_path.join("test_dir/test_file_2.txt");


        fs::File::create(&file_path).unwrap(); // Create a test file in the directory
        fs::File::create(&file_path_2).unwrap(); // Create a test file in the directory
        fs::create_dir(&dir_path).unwrap();
        fs::File::create(&file_path_3).unwrap(); // Create a test file in the directory

        let file_tree = FileTree::new(root_path).unwrap();

        let children = file_tree.get_children(root_path, false, None);
        assert!(children.is_some());
        assert_eq!(children.unwrap(), vec![PathBuf::from("test_file.txt")]);

        fs::remove_file(file_path_3).unwrap();
        fs::remove_dir(dir_path).unwrap();
        fs::remove_file(file_path).unwrap(); // Remove the test file
        fs::remove_file(file_path_2).unwrap(); // Remove the test file
        fs::remove_dir(root_path).unwrap(); // Remove the test directory
    }
}
