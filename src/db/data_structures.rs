use serde::{Deserialize, Serialize};

pub struct Rating {
    pub id: u64,
    pub mark: i32,
    pub rater_id: i32,
    pub rater_username: String
}

#[derive(Clone)]
#[derive(Deserialize, Debug, Serialize)]
pub struct User {
    pub id: u64,
    pub username: String,
    pub password: String,
    pub nationality: String,
    pub sexual_orientation: String,
    pub race: String,
    pub pics_urls: Vec<String>
}

impl std::fmt::Display for User {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "User {{\n  id: {},\n  username: {},\n  nationality: {},\n  race: {},\n orientation: {}\n,  pics_urls: [{}]\n}}",
            self.id,
            self.username,
            self.nationality,
            self.race,
            self.sexual_orientation,
            self.pics_urls.join(", ")
        )
    }
}

impl std::fmt::Display for Rating {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Rating {{ id: {}, mark: {}, rater_id: {}, rater_username: {} }}",
            self.id,
            self.mark,
            self.rater_id,
            self.rater_username
        )
    }
}
