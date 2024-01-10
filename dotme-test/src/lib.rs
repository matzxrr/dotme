use directories::BaseDirs;
use git2::Repository;

pub fn create_temp_repo() -> Repository {
    let base_dirs = match BaseDirs::new() {
        Some(p) => p,
        None => {
            panic!("couldin't find base_dir");
        }
    };
    match Repository::init(base_dirs.home_dir().join(".tmp")) {
        Ok(repo) => repo,
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
