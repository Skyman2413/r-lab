use crate::errors::CreateErrorEnum;
use crate::errors::GitError;
use git2::Repository;
use std::path::PathBuf;
pub struct GitStorage {
    base_path: PathBuf,
}

impl GitStorage {
    pub fn new(base_path: PathBuf) -> GitStorage {
        GitStorage { base_path }
    }

    pub fn get_repo_path(&self, owner: &str, name: &str) -> PathBuf {
        let mut path = self.base_path.clone();
        path.push(owner);
        path.push(format!("{}.git", name));
        path
    }

    pub fn create_repo(&self, owner: &str, name: &str) -> Result<(), GitError> {
        let mut dir = self.base_path.clone();
        dir.push(owner);
        std::fs::create_dir_all(dir);
        let path = self.get_repo_path(owner, name);
        if path.try_exists()? {
            return Err(GitError::CreateError(CreateErrorEnum::RepoAlreadyExists {
                owner: owner.to_string(),
                repo_name: name.to_string(),
            }));
        };
        let repo = Repository::init_bare(path);
        match repo {
            Ok(_) => Ok(()),
            Err(e) => Err(e.into()),
        }
    }
}
