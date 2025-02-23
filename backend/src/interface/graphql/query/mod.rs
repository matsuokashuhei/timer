pub mod user;
use async_graphql::MergedObject;
use user::UserQuery;

#[derive(Default, MergedObject)]
pub struct Query(UserQuery);
