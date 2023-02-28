use std::fs::rename;
use std::path::{Path, PathBuf};

pub fn run(config: Config) -> Result<bool, String> {
    // filter the input
    let salt = config.salt;
    let files = config.files;

    // filter earch file in the selected files
    for file in files {
        let file_path = Path::new(&file);

        if file_path.exists() {
            match get_file_type(file_path).unwrap() {
                FileTypes::File => rename_file(file_path, &salt),

                FileTypes::Dir => {
                    let dir_content = read_dir_recur(&file_path);

                    for file in dir_content {
                        rename_file(&file.as_path(), &salt)
                    }
                }
                FileTypes::Symlink => (),
            }
        } else {
            return Err("file don't found".to_string());
        }
    }
    Ok(true)
}

fn read_dir_recur(dir: &Path) -> Vec<PathBuf> {
    let mut recur_files: Vec<PathBuf> = Vec::new();
    let files = read_dir(dir);

    for file in files {
        if file.is_file() {
            recur_files.push(file);
        } else if file.is_dir() {
            let mut files_in_dir = read_dir_recur(&file);
            recur_files.append(&mut files_in_dir);
        }
    }

    return recur_files;
}

fn read_dir(dir: &Path) -> Vec<PathBuf> {
    let mut files: Vec<PathBuf> = Vec::new();

    let dir_files = dir.read_dir().unwrap();
    for file in dir_files {
        let file_path = file.unwrap().path();
        files.push(file_path);
    }

    return files;
}

fn have_extension(file_path: &Path) -> bool {
    let file_extension = file_path.extension();

    match file_extension {
        Some(_) => true,
        None => false,
    }
}

fn salt_file(file: &Path, salt: &String) -> String {
    let full_name = file.file_name().unwrap().to_string_lossy().to_string();
    let mut final_name = String::from(".");

    let len_of_file = full_name.len();
    let full_path = file.to_string_lossy().to_string();
    let back_path = file.to_string_lossy()[0..full_path.len() - len_of_file].to_string();

    if have_extension(file) {
        let name = file.file_stem().unwrap().to_string_lossy().to_string();
        let extension = file.extension().unwrap().to_string_lossy().to_string();

        final_name.push_str(format!("{}{}.{}", name, salt, extension).as_str());
    } else {
        final_name.push_str(format!("{}{}", full_name, salt).as_str())
    }

    final_name = format!("{back_path}{final_name}");
    final_name
}

fn rename_file(file: &Path, salt: &String) {
    let final_name = salt_file(file, salt);
    rename(file, final_name).expect("error renaming the file");
}

fn get_file_type(file: &Path) -> Option<FileTypes> {
    if file.is_file() {
        return Some(FileTypes::File);
    } else if file.is_dir() {
        return Some(FileTypes::Dir);
    } else if file.is_symlink() {
        return Some(FileTypes::Symlink);
    } else {
        return None;
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

#[cfg(test)]
mod tests {

    use rand::{thread_rng, Rng};
    use std::env::temp_dir;
    use std::fs::write;
    use std::path::{Path, PathBuf};

    const BASE_FILE_NAME: &str = "FILE_";

    fn random_text_generator(length: usize) -> String {
        let mut rng = thread_rng();

        let char_set = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
        let char_set_len = char_set.len();

        let rand_text: String = (0..length)
            .map(|_| {
                let rand_index = rng.gen_range(0..char_set_len);

                let mut temp_char = String::new();

                for (index, char) in char_set.chars().enumerate() {
                    if rand_index == index {
                        temp_char.push(char);
                    }
                }

                temp_char
            })
            .collect();

        rand_text
    }

    /*

        fn create_file(dir:&Path, file_name:&String) -> &Path{

            let file = dir.join(file_name);

            write(file.as_path(), "").expect("error writing to file");

            let file = file.as_path();

            file

        }

    */

    #[test]
    fn test_single_file_with_extension() {
        let temp_dir = temp_dir();

        let temp_file_name = format!("{}{}", BASE_FILE_NAME, "1");

        //        let temp_file = create_file(&temp_dir, &temp_file_name);

        let rand_name = random_text_generator(15);
        println!("{rand_name}")
    }
}
