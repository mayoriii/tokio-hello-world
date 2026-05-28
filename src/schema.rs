use async_graphql::{EmptyMutation, EmptySubscription, Object, Schema, SimpleObject};

#[derive(SimpleObject)]
pub struct Status {
    pub ok: bool,
}

pub struct QueryRoot;

#[Object]
impl QueryRoot {
    async fn status(&self) -> Status {
        Status { ok: true }
    }
}

pub type AppSchema = Schema<QueryRoot, EmptyMutation, EmptySubscription>;

pub fn build_schema() -> AppSchema {
    Schema::build(QueryRoot, EmptyMutation, EmptySubscription).finish()
}
