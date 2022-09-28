use mongodb::{bson::extjson::de::Error, sync::Collection};

use crate::models::cargo::{Cargo, CreateCargo};

#[derive(Debug)]
pub struct CargoRepository {
    collection: Collection<Cargo>,
}

impl CargoRepository {
    pub fn new(collection: Collection<Cargo>) -> CargoRepository {
        CargoRepository { collection }
    }

    pub fn create(&self, input: CreateCargo) -> Result<Cargo, Error> {
        let mut cargo = Cargo {
            _id: None,
            codigo: input.codigo,
            nome: input.nome,
        };

        let data = self
            .collection
            .insert_one(&cargo, None)
            .ok()
            .expect("Error creating cargo");

        cargo._id = data.inserted_id.as_object_id();

        Ok(cargo)
    }

    pub fn get_all(&self) -> Result<Vec<Cargo>, Error> {
        let cursors = self
            .collection
            .find(None, None)
            .ok()
            .expect("Error getting list of cargos");
        let cargos = cursors.map(|doc| doc.unwrap()).collect();
        Ok(cargos)
    }
}
