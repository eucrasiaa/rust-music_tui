use std::process::Command;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;
// use std::collections::BTreeMap;
use std::collections::btree_map::{BTreeMap,Entry};
use std::ffi::OsStr;


#[derive(Debug, Default)]
struct FolderNode {
    // files of folder
    files: Vec<PathBuf>,
    // count for ease
    // subfolders of "folder name, node"
    subfolders: BTreeMap<String, FolderNode>,
    num_files: u64,
    num_subfolders: u64,
    open_view:bool,
}
//
// impl FolderNode{
//     fn new() -> Self{
//         // FolderNode{Default::default(),0,..Default::default()} 
//
//
//
//     }
// }
//
struct DirectoryFriend{
    root_node: FolderNode,
    num_songs:u64,
    num_albums:u64,
    open_direct: *mut FolderNode,
}

const VALID_EXTS: &[&str] = &["mp3", "flac", "wav", "aac", "aiff", "ogg", "opus"];

fn is_valid(ext:&OsStr) -> bool {
    VALID_EXTS.iter().any(|&v| OsStr::new(v) == ext)
}



fn main(){

    // let root_dir = Path::new("./Music");
    let root_dir = Path::new("/home/willc/Music/Sort/");
    let mut root_node = FolderNode::default();
    // let mut music_directory = {FolderNode::default()};
    for entry in WalkDir::new(root_dir).into_iter().filter_map(|e| e.ok()){
        // println!("{}",entry.path().to_str().unwrap());
        // match entry {
        //     Ok(path) => println!("{}", path.path().to_str().unwrap() ),
        //
        //     Err(_) => panic!(),
        //  }
        let path = entry.path();
        if path == root_dir{
            continue
        }
        // make relative to root:
        let rel_path = path.strip_prefix(root_dir).unwrap();
        // basic traverse concept:
        //   1. point at top of FolderNode = root:
        //   2. .components() = iterator of chunks of path:
        //      Music/fol1/fol1b => Music, fol1, fol1b
        //      /Music/fol2/fol2c => /, Music, fol2, fol2c
        //  3. entry could be file OR folder, if file (with is_file()), get parent?
        //  4a) conv component into as_os_str (native)
        //  4b) into to_string_lossy (for handling non utf8?)
        //  4c) into just a string?
        //  
        //  5) for components (from root):
        //     attempt to traverse down 
        //     check if it already exists within the structure, otherwise add it
        //  6) go back to the raw entry path, and if its a file, add it to that located node
        //  7) otherwise, return
        //  
        //
        let mut nav_ptr = &mut root_node;

        let is_file = path.is_file();
        // split off file to not create a foler NAMED after a file lol
        let components_iter = if is_file {
            match rel_path.parent() {
                Some(rel_path_parent) => rel_path_parent.components(),
                None => {
                    println!("oops! on {}",path.to_str().unwrap());
                    continue
                }
            }
         }else{
            rel_path.components()
        };

        for path_comp in components_iter{
            let folder_name = path_comp.as_os_str().to_string_lossy().to_string();
            nav_ptr = nav_ptr.subfolders.entry(folder_name).or_insert_with(FolderNode::default);
            // println!("nav_ptr: {:?}",nav_ptr);
            // returns entry for folder_name. if not exist, insert a defualt FolderNode
        }
        println!("nav_ptr: {:?}",nav_ptr);
        if is_file{
            if let Some(ext) = path.extension() {
                if is_valid(ext){
                    nav_ptr.num_files +=1;
                    nav_ptr.files.push(path.to_path_buf());
                }
            }
            else{
                continue
            }
        }


    }

    println!("Tree built successfully!");
    // root_node.print_recursive(String::from("Music"));
}

