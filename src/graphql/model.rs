use async_graphql::{Context, EmptySubscription, Object, Schema, ID};
use mongodb::bson::{extjson::de::Error};


use crate::{repositories::MongoRepository, models::cargo::{Cargo, CreateCargo}};

pub struct Query;

#[Object]
impl Query {
    async fn hello(&self, _ctx: &Context<'_>) -> String {
        "GraphQL says hello!".to_string()
    }

    async fn cargo_by_id(&self, _ctx: &Context<'_>, id: ID) -> Result<Cargo, Error> {
        let repository = &_ctx.data_unchecked::<MongoRepository>();
        repository.cargo.get_by_id(id.as_str().to_owned())
    }

    async fn cargo_get_all(&self, _ctx: &Context<'_>) -> Result<Vec<Cargo>, Error> {
        let repository = &_ctx.data_unchecked::<MongoRepository>();
        repository.cargo.get_all()
    }
}

pub struct Mutation;

#[Object]
impl Mutation {
    async fn cargo_create(&self, _ctx: &Context<'_>, input: CreateCargo) -> Result<Cargo, Error> {
        let repository = &_ctx.data_unchecked::<MongoRepository>();
        repository.cargo.create(input)
    }
}

pub type MySchema = Schema<Query, Mutation, EmptySubscription>;
