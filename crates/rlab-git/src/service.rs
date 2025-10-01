use crate::errors::GitError;
use crate::storage::GitStorage;
use std::process::Stdio;
use tokio::io::AsyncWriteExt;
use tokio::process::Command;

pub struct GitService {
    storage: GitStorage,
}

impl GitService {
    pub fn new(storage: GitStorage) -> Self {
        Self { storage }
    }

    pub async fn handle_upload_pack(
        &self,
        owner: &str,
        name: &str,
        input: &[u8],
    ) -> Result<Vec<u8>, GitError> {
        self.run_git_command("upload-pack", owner, name, input)
            .await
    }

    pub async fn handle_receive_pack(
        &self,
        owner: &str,
        name: &str,
        input: &[u8],
    ) -> Result<Vec<u8>, GitError> {
        self.run_git_command("receive-pack", owner, name, input)
            .await
    }

    async fn run_git_command(
        &self,
        subcommand: &str,
        owner: &str,
        name: &str,
        input: &[u8],
    ) -> Result<Vec<u8>, GitError> {
        let repo_path = self.storage.get_repo_path(owner, name);
        if !repo_path.exists() {
            return Err(GitError::RepoNotFound);
        }

        let mut child = Command::new("git")
            .arg(subcommand)
            .arg("--stateless-rpc")
            .arg(repo_path)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()?;
        let mut stdin = child.stdin.take().unwrap();
        stdin.write_all(input).await?;
        stdin.flush().await?;
        drop(stdin);
        let output = child.wait_with_output().await?;
        if !output.status.success() {
            return Err(GitError::GitCommandFailed(
                String::from_utf8_lossy(&output.stderr).to_string(),
            ));
        }
        Ok(output.stdout)
    }

    pub async fn advertise_refs(&self, owner: &str, name: &str) -> Result<Vec<u8>, GitError> {
        let repo_path = self.storage.get_repo_path(owner, name);
        if !repo_path.exists() {
            return Err(GitError::RepoNotFound);
        }

        let output = Command::new("git")
            .arg("upload-pack")
            .arg("--stateless-rpc")
            .arg("--advertise-refs")
            .arg(repo_path)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()?
            .wait_with_output()
            .await?;

        if !output.status.success() {
            return Err(GitError::GitCommandFailed(
                String::from_utf8_lossy(&output.stderr).to_string(),
            ));
        }

        Ok(output.stdout)
    }
}
