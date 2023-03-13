use std::fs::rename;
use std::path::{Path, PathBuf};

pub fn run(config: Config) -> Result<bool, String> {
    // filter the input
    let salt = config.salt;
    let targets = config.files;

    // filter earch file in the selected files
    for target in targets {
        let file_path = Path::new(&target);
        let mut file = File::from(&target);

        if file.exists() {
            match file.get_file_type().unwrap() {
                FileTypes::File => file.add_salt(&salt),
                FileTypes::Dir => (),
                FileTypes::Symlink => (),
            }
        } else {
            return Err("file don't found".to_string());
        }
    }
    Ok(true)
}

fn read_directory_recur(dir: &Path) -> Vec<File> {
    let mut recur_files: Vec<File> = Vec::new();
    let files = read_directory(dir);

    for file in files {
        if file.is_file() {
            recur_files.push(File::from(file.to_string_lossy().to_string().as_str()));
        } else if file.is_dir() {
            let mut files_in_dir = read_directory_recur(&file);
            recur_files.append(&mut files_in_dir);
        }
    }

    return recur_files;
}

pub fn read_directory(dir: &Path) -> Vec<PathBuf> {
    let mut files: Vec<PathBuf> = Vec::new();

    let dir_files = dir.read_dir().unwrap();
    for file in dir_files {
        let file_path = file.unwrap().path();
        files.push(file_path);
    }

    return files;
}

pub struct File {
    path: PathBuf,
    name: String,
    extension: Extension,
}

#[derive(Clone)]
pub enum Extension {
    Some(String),
    None,
}

impl File {
    pub fn from(file: &str) -> File {
        let file_path = PathBuf::from(file);
        File::from_path_buff(file_path)
    }

    pub fn from_path_buff(file: PathBuf) -> File {
        let file_path = file;

        let file_name = file_path
            .file_name()
            .expect("Error getting file name")
            .to_string_lossy()
            .to_string();
        let file_extension = match file_path.extension() {
            Some(extension) => Extension::Some(extension.to_string_lossy().to_string()),
            None => Extension::None,
        };

        File {
            path: file_path,
            name: file_name,
            extension: file_extension,
        }
    }

    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    pub fn get_pathbuff(&self) -> PathBuf {
        self.path.clone()
    }

    pub fn get_extension(&self) -> Extension {
        self.extension.clone()
    }

    pub fn have_extension(&self) -> bool {
        let file_extension = &self.extension;
        match file_extension {
            Extension::Some(_) => true,
            Extension::None => false,
        }
    }

    fn salt(&self, salt: &String) -> String {
        let full_name = &self.name;
        let mut final_name = String::new();

        let len_of_file = full_name.len();
        let full_path = self.path.to_string_lossy().to_string();
        let back_path = full_path[0..full_path.len() - len_of_file].to_string();

        if self.have_extension() {
            let name = self.path.file_stem().unwrap().to_string_lossy().to_string();
            let extension = self.path.extension().unwrap().to_string_lossy().to_string();

            final_name.push_str(format!("{}{}.{}", name, salt, extension).as_str());
        } else {
            final_name.push_str(format!("{}{}", full_name, salt).as_str())
        }

        final_name = format!("{back_path}{final_name}");
        final_name
    }

    pub fn add_salt(&mut self, salt: &String) {
        let final_name = self.salt(salt);
        let backpath = self.path.parent().unwrap().to_str().unwrap();
        rename(&self.path, &final_name).expect("error renaming the file");

        let new_file = File::from(format!("{final_name}").as_str());

        self.name = new_file.name;
        self.path = new_file.path;
        self.extension = new_file.extension;
    }

    fn get_file_type(&self) -> Option<FileTypes> {
        if self.path.is_file() {
            return Some(FileTypes::File);
        } else if self.path.is_dir() {
            return Some(FileTypes::Dir);
        } else if self.path.is_symlink() {
            return Some(FileTypes::Symlink);
        } else {
            return None;
        }
    }

    fn exists(&self) -> bool {
        self.path.exists()
    }
}

enum FileTypes {
    File,
    Dir,
    Symlink,
}

pub struct Config {
    pub salt: String,
    pub files: Vec<String>,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &str> {
        if args.len() < 3 {
            return Err("Not enough arguments");
        }

        let salt = args[1].clone();
        let files = args[2..args.len()].to_vec();

        Ok(Config { salt, files })
    }
}
