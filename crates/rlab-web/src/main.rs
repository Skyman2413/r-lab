use rlab_git::service::GitService;
use rlab_git::storage::GitStorage;
use std::path::PathBuf;

#[tokio::main]
async fn main() {
    let storage = GitStorage::new(PathBuf::from(
        "/home/skyman/rust-project/r-lab/test/".to_string(),
    ));
    let service = GitService::new(storage);

    let refs = service.advertise_refs("noob", "test").await.unwrap();

    println!("Refs: {:?}", String::from_utf8_lossy(&refs));
}
