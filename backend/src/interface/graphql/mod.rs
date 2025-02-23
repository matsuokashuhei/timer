pub mod object;
pub mod query;

use async_graphql::{
    http::{playground_source, GraphQLPlaygroundConfig, GraphiQLSource},
    EmptyMutation, EmptySubscription, Schema,
};
use async_graphql_axum::GraphQL;
use axum::response::{Html, IntoResponse};
use query::Query;

pub async fn playground() -> impl IntoResponse {
    // Html(playground_source(GraphQLPlaygroundConfig::new(
    //     "http://localhost:3000",
    // )))
    Html(
        GraphiQLSource::build()
            .endpoint("http://localhost:3000")
            .finish(),
    )
}

async fn build_schema() -> Schema<Query, EmptyMutation, EmptySubscription> {
    Schema::build(Query::default(), EmptyMutation, EmptySubscription).finish()
}

pub async fn service() -> GraphQL<Schema<Query, EmptyMutation, EmptySubscription>> {
    let schema = build_schema().await;
    GraphQL::new(schema)
}
