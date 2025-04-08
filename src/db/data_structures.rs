pub struct Rating {
    pub id: i32,
    pub mark: i32,
    pub rater_id: i32,
    pub rater_username: String
}

pub struct User {
    pub id: i32,
    pub username: String,
    pub nationality: String,
    pub race: String,
    pub pics_urls: Vec<String>
}

impl User {
    pub fn new() -> Self {
        return User { id: -1, username: "".into(), nationality: "".into(), race: "".into(), pics_urls: Vec::new() }
    }
}

impl std::fmt::Display for User {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "User {{\n  id: {},\n  username: {},\n  nationality: {},\n  race: {},\n  pics_urls: [{}]\n}}",
            self.id,
            self.username,
            self.nationality,
            self.race,
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
