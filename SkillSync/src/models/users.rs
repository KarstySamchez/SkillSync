use serde::{Deserialize, Serialize};
use bcrypt::{hash, verify, DEFAULT_COST};
use uuid::Uuid;
use std::collections::{HashMap, HashSet};
use std::sync::{Mutex, Arc};

// Define a unique ID type
pub type UserID = Uuid;

// The User struct defines the properties of a user
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: UserID,
    pub username: String,
    pub email: String,
    pub password_hash: String,
    pub profile_picture: Option<String>,
    pub bio: Option<String>,
    pub following: HashSet<UserID>,
}

// Define the application state to hold users and followings
#[derive(Debug, Default)]
pub struct AppState {
    pub users: Mutex<HashMap<UserID, User>>,
}

impl User {
    // Signup a new user, hashing their password before storing
    pub fn signup(
        username: &str,
        email: &str,
        password: &str,
        state: Arc<AppState>,
    ) -> Option<User> {
        let mut users = state.users.lock().unwrap();

        // Check if username or email already exists
        if users.values().any(|u| u.username == username || u.email == email) {
            return None; // User already exists
        }

        // Hash the password
        let password_hash = hash(password, DEFAULT_COST).unwrap();

        // Create the new user
        let user = User {
            id: Uuid::new_v4(),
            username: username.to_string(),
            email: email.to_string(),
            password_hash,
            profile_picture: None,
            bio: None,
            following: HashSet::new(),
        };

        users.insert(user.id, user.clone());

        Some(user)
    }

    // Login method to verify user credentials
    pub fn login(email: &str, password: &str, state: Arc<AppState>) -> Option<UserID> {
        let users = state.users.lock().unwrap();

        for user in users.values() {
            if user.email == email && verify(password, &user.password_hash).unwrap() {
                return Some(user.id);
            }
        }

        None // Authentication failed
    }

    // Follow another user
    pub fn follow(&mut self, followee_id: UserID, state: Arc<AppState>) -> bool {
        let users = state.users.lock().unwrap();

        if users.contains_key(&followee_id) && followee_id != self.id {
            self.following.insert(followee_id);
            return true; // Successfully followed
        }

        false // Follow failed
    }

    // Unfollow another user
    pub fn unfollow(&mut self, followee_id: UserID) -> bool {
        if self.following.remove(&followee_id) {
            return true; // Successfully unfollowed
        }

        false // Unfollow failed
    }
}

