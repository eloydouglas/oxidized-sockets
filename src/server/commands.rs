use std::io::Error;
use std::{fs, io};
use std::path::{Path, PathBuf};

pub(crate) struct MakeDir;
impl MakeDir {
    pub fn call(curr_path: &String, new_dir: String) {
        let dir_name = curr_path.to_owned() + "/" + &new_dir;
        fs::create_dir_all(dir_name).unwrap();
    }
}
pub(crate) struct ChangeDir;
impl ChangeDir {
    pub fn call(curr_path: &String, new_path: String) -> String {
        if new_path.is_empty() {
            return "./".to_owned();
        }

        let mut path = PathBuf::from(curr_path);

        if new_path == ".." {
            if path == Path::new("./") {
                return "./".to_owned();
            }
            path.pop();
        } else {
            path.push(new_path);

            if let Ok(resolved) = path.canonicalize() {
                let root = Path::new("./").canonicalize().unwrap();
                if !resolved.starts_with(root) {
                    return curr_path.clone();
                }
            } else {
                return curr_path.clone();
            }
        }

        path.to_str().unwrap_or("./").to_string()
    }
}
pub(crate)struct ListDir;
impl ListDir {
    pub fn call(curr_path: &String) -> Vec<String>{
        let mut paths_list: Vec<String> = Vec::new();

        let paths = fs::read_dir(curr_path).unwrap();

        for entry in paths {
            let entry = entry.unwrap();
            let path = entry.path();

            if path.is_dir() {
                paths_list.push(path.display().to_string() + "/");
            } else {
                paths_list.push(path.display().to_string());
            }
        }
        paths_list
    }
}
pub(crate) struct RemovePath;
impl RemovePath {
    pub fn call(curr_path: &String, path: String) -> io::Result<()> {
        if path == "./" {
            return Err(Error::new(io::ErrorKind::InvalidInput, "Cannot delete root path"))
        }
        let full_path = curr_path.to_owned() + "/" + &path;

        if Path::new(&full_path).is_dir() {
            return fs::remove_dir_all(full_path)
        } else {
            return fs::remove_file(full_path)
        }

    }
}

pub(crate) struct Help;
impl Help {
    pub fn call() -> Vec<String> {
        let mut messages: Vec<String> = Vec::new();
        messages.push("Commands: ".to_string());
        messages.push("help - Show available commands".to_string());
        messages.push("ls - List directory files".to_string());
        messages.push("mkdir <directory_name> - Create a new folder".to_string());
        messages.push("cd <path> - Navigate to given path".to_string());
        messages.push("upload <filepath> - Uploads a file to the current directory".to_string());
        messages.push("rm <path> - Deletes folders and files recursively in the given path".to_string());
        messages.push("exit - Close client".to_string());

        messages
    }
}