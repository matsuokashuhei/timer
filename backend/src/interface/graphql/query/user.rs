use async_graphql::{Error, Object};

use crate::interface::graphql::object::user::UserObject;

#[derive(Debug, Default)]
pub struct UserQuery;

#[Object]
impl UserQuery {
    async fn user(&self, id: i32) -> Result<UserObject, Error> {
        let user = UserObject {
            id,
            name: "John".to_string(),
        };
        Ok(user)
    }
}
