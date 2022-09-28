use crate::models::cargo::{Cargo, CreateCargo};
use mongodb::{
    bson::{doc, extjson::de::Error, oid::ObjectId},
    sync::Collection,
};

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

    pub fn get_by_id(&self, id: String) -> Result<Cargo, Error> {
        let obj_id = ObjectId::parse_str(id).unwrap();
        let filter = doc! {"_id": obj_id};
        let cargo = self
            .collection
            .find_one(filter, None)
            .ok()
            .expect("Error getting cargo");
        Ok(cargo.unwrap())
    }

    pub fn get_all(&self) -> Result<Vec<Cargo>, Error> {
        let cursors = self
            .collection
            .find(None, None)
            .ok()
            .expect("Error getting list of cargo");
        let cargos = cursors.map(|doc| doc.unwrap()).collect();
        Ok(cargos)
    }
}
