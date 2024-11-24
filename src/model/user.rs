use rocket::serde::{json::Json, Serialize};

#[derive(Serialize)]
pub enum Gender {
    male,
    female,
}

#[derive(Serialize)]
pub struct User {
    pub id: u32,
    pub name: String,
    pub last_name: String,
    pub username: String,
    pub email: String,
    pub is_active: bool,
    pub gender: Gender,
}

impl User {
    pub fn new(
        id: u32,
        name: String,
        last_name: String,
        username: String,
        email: String,
        is_active: bool,
        gender: Gender,
    ) -> Self {
        return User {
            id,
            name,
            last_name,
            username,
            email,
            is_active,
            gender,
        };
    }

    pub fn all() -> Vec<User> {
        User::get_all_test_users()
    }

    fn get_all_test_users() -> Vec<User> {
        vec![
            User::new(
                1,
                "John".to_string(),
                "Doe".to_string(),
                "johndoe".to_string(),
                "john@example.com".to_string(),
                true,
                Gender::male,
            ),
            User::new(
                2,
                "Jane".to_string(),
                "Doe".to_string(),
                "janedoe".to_string(),
                "jane@example.com".to_string(),
                true,
                Gender::female,
            ),
            User::new(
                3,
                "Alex".to_string(),
                "Smith".to_string(),
                "alexsmith".to_string(),
                "alex@example.com".to_string(),
                false,
                Gender::male,
            ),
        ]
    }
}
