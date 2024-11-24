use crate::model::user::User;

pub fn index() -> Vec<User> {
    User::all()
}

pub fn show(id: u32) -> Option<User> {
    
    let users = User::all();

    for user in users {
        if id == user.id {
            return Some(user);
        }
    }

    return None;
}
