use async_graphql::{SimpleObject, InputObject};
use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, SimpleObject)]
pub struct Usuario {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub _id: Option<ObjectId>,
    pub email: String,
    pub senha_hash: String,
    pub role: String,
}

#[derive(InputObject)]
pub struct CreateUsuario {
    pub email: String,
    pub senha: String,
    pub role: String,
}

#[derive(InputObject)]
pub struct LoginUsuario {
  pub email: String,
  pub senha: String,
}
