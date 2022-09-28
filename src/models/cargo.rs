use async_graphql::{SimpleObject, InputObject};
use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, SimpleObject)]
pub struct Cargo {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub _id: Option<ObjectId>,
    pub codigo: i32,
    pub nome: String,
}

#[derive(InputObject)]
pub struct CreateCargo {
    pub codigo: i32,
    pub nome: String,
}
