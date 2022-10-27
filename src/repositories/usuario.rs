use async_graphql::Error as GraphQLError;

use mongodb::{
    bson::{doc, extjson::de::Error as MongoDbError, oid::ObjectId},
    sync::Collection,
};

use crate::{
    constants::messages,
    models::usuario::{CreateUsuario, Usuario},
    auth::{
        jwt::generate_token,
        passwords::{hash_password, verify_password},
    },
};

#[derive(Debug)]
pub struct UsuarioRepository {
    collection: Collection<Usuario>,
}

impl UsuarioRepository {
    pub fn new(collection: Collection<Usuario>) -> UsuarioRepository {
        UsuarioRepository { collection }
    }

    pub fn create(&self, input: CreateUsuario) -> Result<Usuario, MongoDbError> {
        let mut usuario = Usuario {
            _id: None,
            email: input.email,
            senha_hash: hash_password(input.senha.as_str()).unwrap(),
            role: input.role.to_uppercase(),
        };

        let data = self
            .collection
            .insert_one(&usuario, None)
            .ok()
            .expect("Error creating usuario");

        usuario._id = data.inserted_id.as_object_id();

        Ok(usuario)
    }

    pub fn get_by_id(&self, id: String) -> Result<Usuario, MongoDbError> {
        let obj_id = ObjectId::parse_str(id).unwrap();
        let filter = doc! {"_id": obj_id};
        let usuario = self
            .collection
            .find_one(filter, None)
            .ok()
            .expect("Error getting cargo");
        Ok(usuario.unwrap())
    }

    pub fn get_all(&self) -> Result<Vec<Usuario>, MongoDbError> {
        let cursors = self
            .collection
            .find(None, None)
            .ok()
            .expect("Error getting list of cargo");
        let usuarios = cursors.map(|doc| doc.unwrap()).collect();
        Ok(usuarios)
    }

    pub fn login(&self, email: String, senha: String) -> Result<String, GraphQLError> {
        let filter = doc! {"email": email};
        if let Some(usuario) = self.collection.find_one(filter, None).unwrap_or(None) {
            if verify_password(&usuario.senha_hash, &senha).unwrap_or(false) {
                let new_token = generate_token(&usuario).unwrap_or("Invalid jwt".to_string());
                return Ok(new_token);
            }
        }

        Err(GraphQLError::new(messages::MESSAGE_LOGIN_FAILED))
    }
}
