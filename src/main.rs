use anyhow::Result;
use walkdir::{DirEntry, WalkDir};
use std::path::{Path, PathBuf};
use std::fs::read_dir;
use std::collections::HashMap;

const MAX_SEARCH_DEPTH: usize = 3;





fn main() {
    println!("Init!");
    // let _ = walk_a_dir();
    // let mut working_path: Vec<PathBuf> = Vec::new();
    let mut folder_cache: HashMap<PathBuf, Vec<PathBuf>> = HashMap::new();
    let root_path = Path::new("./Music");
    let dirs = walk_path(root_path, &mut folder_cache);
    println!("Found {} subdirectories", dirs.len());
    let cached_dirs = walk_path(root_path, &mut folder_cache);
    println!("Found {} subdirectories from cache", cached_dirs.len());
    
    // folder_cache.get(&selection)

    for (pbuff, vec) in folder_cache.iter(){
        println!("{}", pbuff.display());
        for pbb in vec.iter(){
            println!("|_ {}", pbb.display());
        } 
    }
}
fn walk_path(root:&Path, cache:&mut HashMap<PathBuf, Vec<PathBuf>>) -> Vec<PathBuf> {

    let mut subdirs = Vec::new();
    if let Some(cached_dirs) = cache.get(root) {
        return cached_dirs.clone();
    }
    for entry in WalkDir::new(root)
        .min_depth(1)
        .max_depth(1) 
        .into_iter()
        .filter_entry(|e| e.file_type().is_dir())
        .filter_map(|e| e.ok())
    {
        subdirs.push(entry.path().to_path_buf());
    }
    cache.insert(root.to_path_buf(), subdirs.clone());
    subdirs
}

//
// fn walk_path(curr_path:Vec<PathBuf>) -> Result<()> {
//
//     Ok(())
// }


fn get_dirstat(path: PathBuf) -> Result<()>{


    Ok(())
} 

fn walk_a_dir() -> Result<()> {
    println!("walking dir");
    for entry in WalkDir::new("Music").min_depth(1).max_depth(MAX_SEARCH_DEPTH) {
        println!("{}", entry?.path().display());
    }
    Ok(())
}

