pub mod cargo;

use dotenv::dotenv;
use mongodb::sync::Client;
use std::env;

use self::cargo::CargoRepository;
use crate::models::cargo::Cargo;

#[derive(Debug)]
pub struct MongoRepository {
    pub cargo: CargoRepository,
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

        Self {
            cargo: CargoRepository::new(cargo_collection),
        }
    }
}
