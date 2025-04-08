use data_structures::{Rating, User};
use sqlx::Pool;
use sqlx::mysql::{MySqlPoolOptions, MySqlRow};
use sqlx::{Row};

pub mod data_structures;

pub struct Db {
    pub connected: bool,
    executor: Pool<sqlx::MySql>,
}

impl Db {
    pub async fn new() -> Self {
        let db_uri = "mysql://api_user:xiaobaixiong@localhost:3306/mydb";
        let mut db = Db {
            connected: false,
            executor: MySqlPoolOptions::new()
                .max_connections(5)
                .connect(&db_uri)
                .await
                .unwrap(),
        };
        db.connected = true;
        return db;
    }

    pub async fn get_user_data(&self, user_id: i32) -> Result<User, sqlx::Error> {
        // Doesn't return ratings, just user data

        let query = String::from("select u.id,u.username,u.nationality,u.race,p.path_to_pic from users u join pictures p on p.owner_id = u.id where u.id = ?;");

        let rows = sqlx::query(&query)
            .bind(user_id)
            .fetch_all(&self.executor)
            .await?;

        let first_row;

        match rows.first() {
            Some(f) => first_row = f,
            None => return Err(sqlx::Error::RowNotFound),
        }

        let mut pics: Vec<String> = Vec::new();

        for row in &rows {
            pics.push(row.try_get("path_to_pic").unwrap());
        }


        let fetched: User = User {
            id: user_id,
            username: first_row.try_get("username")?,
            nationality: first_row.try_get("nationality")?,
            race: first_row.try_get("race")?,
            pics_urls: pics,
        };

        return Ok(fetched);
    }

    pub async fn get_user_pics(&self, user_id: i32) -> Result<Vec<String>, sqlx::Error> {
        let query = String::from(
            "select path_to_pic from users u join pictures p on p.owner_id = u.id where u.id = ?",
        );

        let rows = sqlx::query(&query)
            .bind(user_id)
            .fetch_all(&self.executor)
            .await?;

        let mut r: Vec<String> = Vec::new();

        for row in rows {
            r.push(row.try_get("path_to_pic")?);
        }

        return Ok(r);
    }

    // Update user data

    // Create new user

    // Delete user

    // Add pics to user

    // remove pics from user

    // add rating to user

    // remove all ratings from user
}
