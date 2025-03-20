use std::{
    collections::HashSet,
    fs, io,
    path::{Path, PathBuf},
};

use mime_guess::{self, mime};

fn is_video(file_path: &Path) -> bool {
    let guess = mime_guess::from_path(file_path);
    guess
        .first()
        .is_some_and(|guessed| guessed.type_() == mime::VIDEO)
}

pub fn get_video_files<P: AsRef<Path>>(dir_path: P) -> io::Result<HashSet<PathBuf>> {
    let mut paths: HashSet<PathBuf> = HashSet::new();

    let entries = fs::read_dir(dir_path)?;
    for entry in entries {
        let entry = entry?;
        let file_type = entry.file_type()?;
        if file_type.is_file() {
            let file_path = entry.path();
            if is_video(&file_path) {
                paths.insert(file_path);
            }
        }
    }

    Ok(paths)
}

#[cfg(test)]
mod tests {
    use super::*;
    use fs::File;
    use rstest::*;

    #[rstest]
    #[case::mp4("a.mp4", true)]
    #[case::MP4("a.MP4", true)]
    #[case::mov("a.mov", true)]
    #[case::MOV("a.MOV", true)]
    #[case::txt("a.txt", false)]
    #[case::m4a("a.m4a", false)]
    #[case::png("a.png", false)]
    #[case::gif("a.gif", false)]
    #[case::DS_Store(".DS_Store", false)]
    #[allow(non_snake_case)]
    fn test_is_video(#[case] filename: &str, #[case] expected: bool) {
        let sut = Path::new(filename);

        let result = is_video(sut);

        assert_eq!(result, expected);
    }

    struct TmpDir {
        path: PathBuf,
    }
    impl Drop for TmpDir {
        fn drop(&mut self) {
            let _ = fs::remove_dir_all(&self.path);
        }
    }

    #[rstest]
    fn test_get_video_files() {
        let tmp_dir = TmpDir {
            path: PathBuf::from(".tmp"),
        };

        fs::create_dir_all(tmp_dir.path.join("dir/empty")).unwrap();
        let file_paths: HashSet<PathBuf> = [".DS_Store", "foo.mp4", "bar.mov", "xxx.png"]
            .into_iter()
            .map(|file_path| tmp_dir.path.join(file_path))
            .collect();
        file_paths.iter().for_each(|file_path| {
            File::create(file_path).unwrap();
        });

        let result = get_video_files(&tmp_dir.path).unwrap();

        let expected = ["foo.mp4", "bar.mov"]
            .into_iter()
            .map(|file_path| tmp_dir.path.join(file_path))
            .collect();
        assert_eq!(result, expected);
    }
}
