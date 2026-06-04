use std::ffi::CString;
use std::path::{Path, PathBuf};
use std::ptr::null;
use sysinfo::System;
use tokio::fs;
use tokio::io::AsyncWriteExt;

enum NodeType {
    FOLDER(String,String,i8),
    FILE(String)
}

 pub enum StatusFM {
    OK,
    ERROR(String),
    AEXIST,
    NOTFOUND,
}
struct Metadata {
    Type: NodeType,
}

struct FmstNode<T> {
    pub relative_path: String, //format expected: /**char**
    pub data: T, //most likely will be used to hold Metadata about the node
    pub children: Vec<FmstNode<T>>,
}
impl<T> FmstNode<T> {
    pub fn new(relative_path: String, data: T) -> Self {
        Self {
            relative_path, data, children: Vec::new()
        }
    }
    pub fn add_child(&mut self, child: FmstNode<T>) {
        self.children.push(child);
    }
    pub fn get_relative_path(&self) -> &str {
        &self.relative_path.as_str() //fixme need to convert to something useful: fixed
    }
}



trait PhFunction {
    fn ph_logic1(&self);
}

trait GetPath {
    fn get_path(&self) -> String;
}

struct FMS_Tree {
    root: FmstNode<String>,

}
impl FMS_Tree {
    pub fn new(r:FmstNode<String>) -> FMS_Tree {
        Self {
            root: r,
        }
    }
    pub fn add_child(&mut self, child: FMS_Tree) {

    }
}
impl PhFunction for FMS_Tree {
    fn ph_logic1(&self) {
        println!("ph logic1");
    }
}





//-----MIAN CORE LOGIC-----//
// --Doc--
//
///one of the first functions that sould be ran to init the program's ability to interact to the local storage
//fn start_up() -> Fms {}

trait StartUp {
    fn start_up_check();
    fn init_root();
}
///add more system info when need here
struct SystemMetaDta {
    ostype: String,
    max_threads: i8,
}
pub struct Fms {
    ph_tree: Option<Box<FMS_Tree>>,
    local_dir: String,
    sys_info: SystemMetaDta
}
macro_rules! create_pathing_check {
    ($name: expr, dir) => {{
        let p = Path::new(std::env::current_dir().unwrap().as_path()).join($name);
        if (p.is_dir()) {return StatusFM::AEXIST;}
        p
    }};
    ($name: expr, dir, $path: expr) => {{
        let mut p = $path.as_ref().to_path_buf();
        p.push($name);
        if (p.is_dir()) {return StatusFM::AEXIST; }
        p
    }};
    ($name: expr, file) => {{
        let p = Path::new(std::env::current_dir().unwrap().as_path()).join($name);
        if (p.is_file()) {return StatusFM::AEXIST;}
        p
    }};
    ($name: expr, file, $path: expr) => {{
        let mut p = $path.as_ref().to_path_buf();
        p.push($name);
        if (p.is_file()) {return StatusFM::AEXIST; }
        p
    }};
}
macro_rules! write_file {
    ($result: expr, $data: expr) => {
        match $result {
            Ok(_) => {
                match $result.unwrap().write_all($data.as_ref()).await {
                    Ok(_) =>  return StatusFM::OK,
                    Err(e) => StatusFM::ERROR(e.to_string())
                };
                return StatusFM::OK;}
            Err(e) => StatusFM::ERROR(e.to_string())
        }
    };
}
impl Fms {
    pub fn new() -> Fms {
        let path_string = std::env::current_dir();
        let os = os_info::get();
        let sys = System::new_all();
        Self {
            ph_tree: None,
            local_dir: path_string.unwrap().to_string_lossy().parse().unwrap(),
            sys_info: SystemMetaDta {
                ostype: os.os_type().to_string(),
                max_threads: sys.cpus().len() as i8,
            },
        }
    }

    //create other controtors
    ///one of the two main function to use to create new user dir and item dir to items in.
    /// this version uses the working dir as the base pathing for dir creation, if you want to create a new dir at an exact loction then use create_item_dir
    /// the default pathing should be the working dir/common/users
    async fn create_item_dir(name: &str) -> StatusFM {

        let p = create_pathing_check!(name, dir);


        match fs::create_dir(&p).await {
            Ok(_) =>  return StatusFM::OK,
            Err(e) => StatusFM::ERROR(e.to_string())
        };
        return StatusFM::OK;
    }
    ///
    async fn create_item_dir_at(name: &str, path: impl AsRef<Path>) -> StatusFM {
        let p = create_pathing_check!(name, dir, path);

        match fs::create_dir(&p).await {
            Ok(_) =>  return StatusFM::OK,
            Err(e) => StatusFM::ERROR(e.to_string())
        };
        return StatusFM::OK;
    }
    ///
    async fn create_item_file(name: &str, data: &str) -> StatusFM {
        let p = create_pathing_check!(name, file);
        let result = fs::File::create(&p).await;
        write_file!(result, data)
    }
    ///
    async fn create_item_file_at(name: &str, path: impl AsRef<Path>, data: &str) -> StatusFM {
        let p = create_pathing_check!(name, file, path);

        let result = fs::File::create(&p).await;
        write_file!(result, data)
    }
    ///this method is no safe as it at the moment just overrides the file
    /// todo update the method to use the temporary file pattern ( read, Modify, Write, Rename)
    async fn update_item_file(name: &str, data: &str) -> StatusFM {
        //path checking
        let p = Path::new(std::env::current_dir().unwrap().as_path()).join(name);
        if (!p.is_file()) {
            return StatusFM::NOTFOUND
        }

        match fs::write(&p,data).await {
            Ok(_) =>  return StatusFM::OK,
            Err(e) => StatusFM::ERROR(e.to_string())
        }
    }

    async fn update_item_file_at(name: &str, path: impl AsRef<Path>, data: &str) -> StatusFM {
        let mut p = path.as_ref().to_path_buf();
        p.push(name);
        if (!p.is_file()) {return StatusFM::NOTFOUND}

        match fs::write(&p,data).await {
            Ok(_) =>  return StatusFM::OK,
            Err(e) => StatusFM::ERROR(e.to_string())
        }
    }
    //todo create patch fn

}
impl StartUp for Fms {
    fn start_up_check() {

    }

    fn init_root() {

    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        //let result = add(2, 2);
        //assert_eq!(result, 4);
    }
}
