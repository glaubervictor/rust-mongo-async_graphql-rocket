mod constants;
mod graphql;
mod models;
mod repositories;
mod auth;

use crate::auth::jwt::UserClaim;
use async_graphql::{
    http::{playground_source, GraphQLPlaygroundConfig},
    EmptySubscription, Schema,
};
use async_graphql_rocket::{GraphQLQuery, GraphQLRequest, GraphQLResponse};
use graphql::model::{Mutation, MySchema, Query};
use repositories::MongoRepository;
use rocket::{response::content, routes, State};
use auth::role_guard::get_role;

#[rocket::get("/hello")]
async fn hello(_schema: &State<MySchema>) -> String {
    "🚀 Aplicação no ar!".to_string()
}

#[rocket::get("/playground")]
fn graphql_playground() -> content::RawHtml<String> {
    content::RawHtml(playground_source(GraphQLPlaygroundConfig::new("/graphql")))
}

#[rocket::get("/graphql?<query..>")]
async fn graphql_query(schema: &State<MySchema>, query: GraphQLQuery) -> GraphQLResponse {
    query.execute(schema).await
}

#[rocket::post("/graphql", data = "<request>", format = "application/json")]
async fn graphql_request(
    schema: &State<MySchema>,
    user_claim: UserClaim,
    request: GraphQLRequest,
) -> GraphQLResponse {
    let role = get_role(user_claim.role);
    let query = request.data(role);
    query.execute(schema).await
}

#[rocket::launch]
async fn rocket() -> _ {
    let repository = MongoRepository::init();
    let schema = Schema::build(Query, Mutation, EmptySubscription)
        .data(repository)
        .finish();

    rocket::build().manage(schema).mount(
        "/",
        routes![graphql_query, graphql_request, graphql_playground, hello],
    )
}
