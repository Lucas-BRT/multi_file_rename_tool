#[cfg(test)]
mod tests {

    use multi_file_rename_tool::File;
    use rand::{thread_rng, Rng};
    use std::{
        env::temp_dir,
        fs::write,
        path::{Path, PathBuf},
    };

    fn random_text_generator(length: usize) -> String {
        let char_set = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ123456789";
        let char_set_len = char_set.len();

        let mut rng = thread_rng();

        let rand_text: String = (0..length)
            .map(|_| {
                let random_index = rng.gen_range(0..char_set_len);

                let mut temp_char = String::new();

                for (index, char) in char_set.chars().enumerate() {
                    if random_index == index {
                        temp_char.push(char);
                    }
                }

                temp_char
            })
            .collect();
        rand_text
    }

    fn create_random_file(dir: &Path) -> PathBuf {
        let rand_name = random_text_generator(32);
        let tempfile = dir.join(rand_name);
        write(&tempfile, "").expect("Error writing the file:{}");

        tempfile
    }

    #[test]
    fn test_single_file() {
        let tempdir = temp_dir();
        let rand_file_pathbuff = create_random_file(&tempdir);
        
        let mut rand_file = File::from_path_buff(rand_file_pathbuff);

        let random_salt = format!(" - {}", random_text_generator(5));

        println!("{:?}", rand_file.get_name());
        println!("{:?}\n", rand_file.get_pathbuff());

        rand_file.add_salt(&random_salt);

        println!("{:?}", rand_file.get_name());
        println!("{:?}\n", rand_file.get_pathbuff());
    }

    #[test]
    fn test_multiple_files() {
        for _ in 0..100_000 {
            test_single_file()
        }
    }


}
