use crate::models::user::{User, AppState};
use uuid::Uuid;
use std::sync::Arc;

pub fn signup(
    username: &str,
    email: &str,
    password: &str,
    state: Arc<AppState>,
) -> Option<User> {
    User::signup(username, email, password, state)
}

pub fn login(
    email: &str,
    password: &str,
    state: Arc<AppState>,
) -> Option<Uuid> {
    User::login(email, password, state)
}

pub fn follow(
    user_id: Uuid,
    followee_id: Uuid,
    state: Arc<AppState>,
) -> bool {
    let mut users = state.users.lock().unwrap();
    if let Some(user) = users.get_mut(&user_id) {
        user.follow(followee_id, state)
    } else {
        false
    }
}

pub fn unfollow(
    user_id: Uuid,
    followee_id: Uuid,
    state: Arc<AppState>,
) -> bool {
    let mut users = state.users.lock().unwrap();
    if let Some(user) = users.get_mut(&user_id) {
        user.unfollow(followee_id)
    } else {
        false
    }
}
