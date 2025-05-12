use crate::schema::{Post, User};

#[derive(Clone)]
pub struct Context {
    pub users: Vec<User>,
    pub posts: Vec<Post>,
}

impl juniper::Context for Context {}

impl Context {
    pub fn new() -> Self {
        Context {
            users: vec![
                User {
                    id: "1".to_string(),
                    name: "Alice".to_string(),
                    email: "alice@example.com".to_string(),
                    posts: vec![],
                },
                User {
                    id: "2".to_string(),
                    name: "Bob".to_string(),
                    email: "bob@example.com".to_string(),
                    posts: vec![],
                },
            ],

            posts: vec![
                Post {
                    id: "1".to_string(),
                    title: "First Post".to_string(),
                    content: "Hello, GraphQL!".to_string(),
                    author_id: "1".to_string(),
                },
                Post {
                    id: "2".to_string(),
                    title: "Second Post".to_string(),
                    content: "Learning Rust!".to_string(),
                    author_id: "2".to_string(),
                },
            ],
        }
    }
}
