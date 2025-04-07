
use sqlx::Row;
use sqlx::mysql::MySqlPoolOptions;
use sqlx::Pool;

pub struct Db {
    pub connected: bool,
    executor: Pool<sqlx::MySql>
}

impl Db {

    pub async fn new() -> Self {
        let db_uri = "mysql://api_user:xiaobaixiong@localhost:3306/mydb";
        let mut db = Db { connected: false, executor: MySqlPoolOptions::new()
            .max_connections(5)
            .connect(&db_uri)
            .await.unwrap() };
        db.connected = true;
        return db;
    }

    pub async fn query1(&self) -> Vec<i32> {
        let rows = sqlx::query("select * from ratings r join users u2 on r.rated_id = u2.id where r.rater_id = 5;").fetch_all(&self.executor).await.unwrap();
        let mut return_vector: Vec<i32> = Vec::new();
        for row in rows {
            return_vector.push(row.try_get("id").unwrap());
            return_vector.push(row.try_get("mark").unwrap());
        }
        return return_vector;
    
    }

    // @ TODO -> queries needed in app (to be defined)
}

