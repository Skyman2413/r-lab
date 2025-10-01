use thiserror::Error;

#[derive(Error, Debug)]
pub enum GitError{
    #[error(transparent)]
    CreateError(#[from] CreateErrorEnum),
    #[error("os error {0}!")]
    OsError(std::io::Error),
    #[error("native git error {0}!")]
    NativeError(git2::Error),
    #[error("Bad full path error {0}")]
    PathError(String),
    #[error("repository not found")]
    RepoNotFound,
    #[error("git command failed: {0}")]
    GitCommandFailed(String),
}
#[derive(Error, Debug)]
pub enum CreateErrorEnum{
    #[error("{owner}'s repository {repo_name} already exists!")]
    RepoAlreadyExists{owner: String, repo_name: String},
}

impl From<git2::Error> for GitError{
    fn from(err: git2::Error) -> Self{
        GitError::NativeError(err)
    }
}

impl From<std::io::Error> for GitError{
    fn from(err: std::io::Error) -> Self{
        GitError::OsError(err)
    }
}