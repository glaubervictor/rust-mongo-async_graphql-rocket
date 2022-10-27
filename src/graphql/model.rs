use async_graphql::{Context, EmptySubscription, Error as GraphError, Object, Schema, ID};
use mongodb::bson::extjson::de::Error;

use crate::models::usuario::LoginUsuario;
use crate::auth::role_guard::{AuthRole, RoleGuard};
use crate::{
    models::cargo::{Cargo, CreateCargo},
    models::usuario::{CreateUsuario, Usuario},
    repositories::MongoRepository,
};

pub struct Query;

#[Object]
impl Query {
    async fn hello(&self, _ctx: &Context<'_>) -> String {
        "GraphQL says hello!".to_string()
    }

    async fn cargo_by_id(&self, _ctx: &Context<'_>, id: ID) -> Result<Cargo, Error> {
        let repository = &_ctx.data_unchecked::<MongoRepository>();
        repository.cargo.get_by_id(id.to_string())
    }

    //#[graphql(guard = "RoleGuard::new(AuthRole::Admin)")]
    async fn cargo_get_all(&self, _ctx: &Context<'_>) -> Result<Vec<Cargo>, Error> {
        let repository = &_ctx.data_unchecked::<MongoRepository>();
        repository.cargo.get_all()
    }

    async fn usuario_by_id(&self, _ctx: &Context<'_>, id: ID) -> Result<Usuario, Error> {
        let repository = &_ctx.data_unchecked::<MongoRepository>();
        repository.usuario.get_by_id(id.to_string())
    }

    #[graphql(guard = "RoleGuard::new(AuthRole::Admin)")]
    async fn usuario_get_all(&self, _ctx: &Context<'_>) -> Result<Vec<Usuario>, Error> {
        let repository = &_ctx.data_unchecked::<MongoRepository>();
        repository.usuario.get_all()
    }
}

pub struct Mutation;

#[Object]
impl Mutation {
    async fn cargo_create(&self, _ctx: &Context<'_>, input: CreateCargo) -> Result<Cargo, Error> {
        let repository = &_ctx.data_unchecked::<MongoRepository>();
        repository.cargo.create(input)
    }

    async fn usuario_create(
        &self,
        _ctx: &Context<'_>,
        input: CreateUsuario,
    ) -> Result<Usuario, Error> {
        let repository = &_ctx.data_unchecked::<MongoRepository>();
        repository.usuario.create(input)
    }

    async fn usuario_login(
        &self,
        _ctx: &Context<'_>,
        input: LoginUsuario,
    ) -> Result<String, GraphError> {
        let repository = &_ctx.data_unchecked::<MongoRepository>();
        repository.usuario.login(input.email, input.senha)
    }
}

pub type MySchema = Schema<Query, Mutation, EmptySubscription>;
