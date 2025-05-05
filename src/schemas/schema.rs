use juniper::{EmptyMutation, EmptySubscription, FieldResult, RootNode, GraphQLObject};

#[derive(GraphQLObject)]
#[graphql(description = "Ping me!")]
struct Ping {
    pong: String,
    pong2: String,
}

pub struct QueryRoot;

#[juniper::graphql_object]
impl QueryRoot {
    fn ping() -> FieldResult<Ping> {
        Ok(Ping {
            pong: "pong".to_string(),
            pong2: "pong2".to_string(),
        })
    }
}

pub type Schema = RootNode<'static, QueryRoot, EmptyMutation<()>, EmptySubscription<()>>;

pub fn create_schema() -> Schema {
    Schema::new(QueryRoot {}, EmptyMutation::new(), EmptySubscription::new())
}