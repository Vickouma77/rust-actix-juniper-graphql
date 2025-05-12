use juniper::{EmptySubscription, RootNode};
use crate::context::Context;
use crate::resolver::{Query, Mutation};

#[derive(Clone)]
pub struct User {
    pub id: String,
    pub name: String,
    pub email: String,
    pub posts: Vec<Post>,   
}

#[derive(Clone)]
pub struct Post {
    pub id: String,
    pub title: String,
    pub content: String,
    pub author_id: String,
}

pub type Schema = RootNode<'static, Query, Mutation, EmptySubscription<Context>>;

pub fn schema() -> Schema {
    Schema::new(Query, Mutation, EmptySubscription::new())
}