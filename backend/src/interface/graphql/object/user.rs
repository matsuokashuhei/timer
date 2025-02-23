use async_graphql::{ComplexObject, SimpleObject};

#[derive(SimpleObject)]
#[graphql(name = "User", complex)]
pub struct UserObject {
    pub id: i32,
    pub name: String,
}

#[ComplexObject]
impl UserObject {
    async fn some_field(&self) -> &str {
        "some_value"
    }
}
