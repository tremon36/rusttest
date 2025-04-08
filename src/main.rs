use db::data_structures::User;
use db::Db;

mod db;
mod server;

#[tokio::main]
async fn main() {



    // Connect to db

    let dbv: Db = Db::new().await;

    let r = dbv.get_user_data(3).await;
    let mut user = User::new();

    match r {
        Ok(data) => user = data,
        Err(e) => println!("Unknown User ({e})")
    }

    let r = dbv.get_user_pics(3).await;
    
    match r {
        Ok(data) => user.pics_urls = data,
        Err(e) => println!("Unknown User ({e})")
    }

    println!("{}",user);

    // Create a server

    server::create_server().await;

    
}
