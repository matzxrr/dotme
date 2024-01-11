use std::{env, fs, path::Path};

use git2::Repository;

pub struct TestRepo(Repository);

impl Drop for TestRepo {
    fn drop(&mut self) {
        let repo_path = self.0.path();
        println!("Destorying {}", repo_path.display());
        // if let Err(err) = std::fs::remove_dir_all(repo_path) {
        //     eprintln!("Unable to destory {} - {}", repo_path.display(), err);
        // };
    }
}

/// Creates a test repository that gets destroyed on `Drop`
pub fn create_temp_repo() -> TestRepo {
    let temp_dir = Path::new(".tmp");
    println!("creating dir {}", temp_dir.display());
    if let Err(err) = fs::create_dir_all(temp_dir) {
        panic!(
            "Unable to create temp directory {} - {}",
            temp_dir.display(),
            err
        );
    }
    let name = std::thread::current()
        .name()
        .expect("Can't get thread name")
        .to_owned();
    let temp_git_dir = temp_dir.join(name);
    match Repository::init_bare(temp_git_dir) {
        Ok(repo) => TestRepo(repo),
        Err(e) => panic!("failed to create temp repo: {}", e),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_repo() {
        create_temp_repo();
    }
}
