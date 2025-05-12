use juniper::{graphql_object, graphql_value, FieldResult, FieldError};
use log::{info, debug, error};
use crate::auth::login;
use crate::context::Context;
use crate::schema::{User, Post};

pub struct Query;

#[graphql_object(context = Context)]
impl Query {
    fn users(context: &Context) -> FieldResult<Vec<User>> {
        info!("Fetching all users");
        Ok(context.users.clone())
    }

    fn user(context: &Context, id: String) -> FieldResult<Option<User>> {
        debug!("Fetching user with id: {}", id);
        Ok(context.users.iter().find(|user| user.id == id).cloned())
    }

    fn posts(context: &Context) -> FieldResult<Vec<Post>> {
        info!("Fetching all posts");
        Ok(context.posts.clone())
    }
}

#[graphql_object(context = Context)]
impl User {
    fn posts(&self, context: &Context) -> FieldResult<Vec<Post>> {
        debug!("Fetching posts for user: {}", self.id);
        Ok(context
            .posts
            .iter()
            .filter(|post| post.author_id == self.id)
            .cloned()
            .collect())
    }
}

#[graphql_object(context = Context)]
impl Post {
    fn author(&self, context: &Context) -> FieldResult<Option<User>> {
        debug!("Fetching author for post: {}", self.id);
        Ok(context.users.iter().find(|user| user.id == self.author_id).cloned())
    }
}

pub struct Mutation;

#[graphql_object(context = Context)]
impl Mutation {
    fn create_user(context: &Context, name: String, email: String) -> FieldResult<User> {
        if name.is_empty() || email.is_empty() {
            error!("Failed to create user: Name and email are required");
            return Err(FieldError::new(
                "Name and email are required",
                graphql_value!({"code": "INVALID_INPUT"}),
            ));
        }
        info!("Creating user: {} <{}>", name, email);
        let user = User {
            id: (context.users.len() + 1).to_string(),
            name,
            email,
            posts: vec![],
        };
        Ok(user)
    }

    fn create_post(
        context: &Context,
        title: String,
        content: String,
        author_id: String,
    ) -> FieldResult<Post> {
        if title.is_empty() {
            error!("Failed to create post: Title is required");
            return Err(FieldError::new(
                "Title is required",
                graphql_value!({"code": "INVALID_INPUT"}),
            ));
        }
        info!("Creating post: {} for author: {}", title, author_id);
        let post = Post {
            id: (context.posts.len() + 1).to_string(),
            title,
            content,
            author_id,
        };
        Ok(post)
    }

    fn login(context: &Context, email: String) -> FieldResult<String> {
        login(context, email)
    }
}