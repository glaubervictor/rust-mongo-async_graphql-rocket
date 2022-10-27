# rust-mongo-async_graphql-rocket
rust mongodb async_graphql and rocket

This repo demonstrates how to setup a Rust service using Rocket, async-graphql and mongodb. 

## Database Setup

1.  Create a `.env` file in the project root directory and add the following line:

    ```SERVER_DATABASE=mongodb://your_connection_here```

    This environment variable is used as a configuration input to the database connection pool to indicate where to find the database

## Running the service

The current configuration in `Rocket.toml` has been setup to run the service on port 8080. After you have `cargo build` and `cargo run`, you can browse the graphQL server playground here:

```
http://localhost:8080/playground/
```
