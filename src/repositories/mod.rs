pub mod cargo;
pub mod usuario;

use dotenv::dotenv;
use mongodb::sync::Client;
use std::env;

use self::{cargo::CargoRepository, usuario::UsuarioRepository};
use crate::models::{cargo::Cargo, usuario::Usuario};

#[derive(Debug)]
pub struct MongoRepository {
    pub cargo: CargoRepository,
    pub usuario: UsuarioRepository,
}

impl MongoRepository {
    pub fn init() -> Self {
        dotenv().ok();
        let uri = match env::var("SERVER_DATABASE") {
            Ok(v) => v.to_string(),
            Err(_) => format!("Error loading env variable"),
        };
        let client = Client::with_uri_str(uri).unwrap();
        let db = client.database("rustPontoDB");

        //collections
        let cargo_collection = db.collection::<Cargo>("cargos");
        let usuario_collection = db.collection::<Usuario>("usuarios");

        Self {
            cargo: CargoRepository::new(cargo_collection),
            usuario: UsuarioRepository::new(usuario_collection),
        }
    }
}
