use axum::response::IntoResponse;
use data_structures::{Rating, User};
use sqlx::Pool;
use sqlx::Row;
use sqlx::mysql::{MySqlPoolOptions, MySqlRow};
use tokio::sync::OnceCell;

pub mod data_structures;

pub struct Db {
    pub connected: bool,
    executor: Pool<sqlx::MySql>,
}

static DB_INSTANCE: OnceCell<Db> = OnceCell::const_new();

pub async fn get_db() -> &'static Db {
    return DB_INSTANCE
        .get_or_init(|| async {
            let result = Db::new().await;
            match result {
                Ok(db) => {
                    return db;
                }
                Err(_e) => {
                    panic!("Error: can't connect to DB");
                }
            }
        })
        .await;
}

impl Db {
    pub async fn new() -> Result<Self, sqlx::Error> {
        let db_uri = "mysql://api_user:xiaobaixiong@localhost:3306/mydb";
        let mut db = Db {
            connected: false,
            executor: MySqlPoolOptions::new()
                .max_connections(5)
                .connect(&db_uri)
                .await?,
        };
        db.connected = true;
        return Ok(db);
    }

    pub async fn create_user(&self, user: &User) -> Result<User, sqlx::Error> {
        let query = String::from(
            "insert into users (username, passwd, nationality,race, sexual_orientation) values (?, ?, ?, ?, ?)",
        );

        let affected_rows = sqlx::query(&query)
            .bind(&user.username)
            .bind(&user.password)
            .bind(&user.nationality)
            .bind(&user.race)
            .bind(&user.sexual_orientation)
            .execute(&self.executor)
            .await?;

        let mut inserted_user = user.clone();
        inserted_user.id = affected_rows.last_insert_id();

        return Ok(inserted_user);
    }

    pub async fn update_user(&self, user: &User) -> Result<User, sqlx::Error> {
        let query = String::from(
            "update users set username = ?, passwd = ?, nationality = ?, race = ?, sexual_orientation = ? where id = ? and passwd = ?",
        );
        let affected_rows = sqlx::query(&query)
            .bind(&user.username)
            .bind(&user.password)
            .bind(&user.nationality)
            .bind(&user.race)
            .bind(&user.sexual_orientation)
            .bind(&user.id)
            .bind(&user.password)
            .execute(&self.executor)
            .await?;

        if affected_rows.rows_affected() <= 0 {
            return Err(sqlx::Error::RowNotFound);
        }

        return Ok(user.clone());
    }

    pub async fn get_user_data(&self, user: &User) -> Result<User, sqlx::Error> {
        let query = String::from(
            "select u.id,u.username,u.nationality,u.race,u.sexual_orientation,p.path_to_pic from users u left join pictures p on p.owner_id = u.id where u.id = ? and u.passwd = ?",
        );

        let rows = sqlx::query(&query)
            .bind(&user.id)
            .bind(&user.password)
            .fetch_all(&self.executor)
            .await?;

        let first_row;

        match rows.first() {
            Some(f) => first_row = f,
            None => return Err(sqlx::Error::RowNotFound),
        }

        let mut pics: Vec<String> = Vec::new();

        for row in &rows {
            let img_url: Option<String> = row.try_get("path_to_pic")?;

            if img_url.is_some() {
                pics.push(img_url.expect("this should never panic"));
            }
        }

        let fetched: User = User {
            id: user.id,
            password: "".into(),
            username: first_row.try_get("username")?,
            sexual_orientation: first_row.try_get("sexual_orientation")?,
            nationality: first_row.try_get("nationality")?,
            race: first_row.try_get("race")?,
            pics_urls: pics,
        };

        return Ok(fetched);
    }

    pub async fn get_user_to_rate(&self) -> Result<User, sqlx::Error> {
        let query = String::from( // Probably a better algorithm is needed...
            "SELECT u.id, u.username, 
            (COUNT(CASE WHEN r.rater_id = u.id THEN 1 END) - COUNT(CASE WHEN r.rated_id = u.id THEN 1 END)) AS rating_difference
            FROM users u
            LEFT JOIN ratings r ON u.id = r.rater_id OR u.id = r.rated_id
            GROUP BY u.id, u.username
            ORDER BY rating_difference DESC
            LIMIT 1;",
        );

        let first_row = sqlx::query(&query).fetch_one(&self.executor).await?;

        let user_id = first_row.try_get("id").unwrap(); // Since id is not null this is safe
        let user_pics = self.get_user_pics(user_id).await.unwrap(); // @TODO error handling

        let fetched: User = User {
            id: user_id,
            password: "".into(),
            username: first_row.try_get("username")?,
            sexual_orientation: "".into(),
            nationality: "".into(),
            race: "".into(),
            pics_urls: user_pics,
        };

        return Ok(fetched);
    }

    pub async fn get_user_pics(&self, user_id: u64) -> Result<Vec<String>, sqlx::Error> {
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
