use rlab_db::models::NewUser;
use rlab_db::pool::create_pool;
use rlab_db::repositories::user::DbUserRepository;


#[tokio::main]
async fn main() {
    // TODO take from ENV
    let pool =
        create_pool("postgresql://r-lab:2413@localhost:5432/r-lab").await.unwrap();

    let userRepo = DbUserRepository::new(pool);
    let _ = userRepo.create(
        &NewUser {
            name: "Noob".to_string(),
            email: "itsme@rust.rs".to_string(),
        }
    ).await;
    let noob_user = userRepo.find_by_email("itsme@rust.rs").await.unwrap();
    println!("{:?}", noob_user);

}
