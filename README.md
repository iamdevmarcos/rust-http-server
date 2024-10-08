# Rust Http Server

This is a basic example of a server built with Rust using the [Warp](https://github.com/seanmonstar/warp) web framework. This server has a single route (`/`) that responds with a JSON message.

## Features

- Uses the Warp framework for handling HTTP requests.
- Provides an endpoint (`/`) that returns a JSON response.
- Structured using modules for configuration, routes, handlers, and models.

## Modules

### `main.rs`
The entry point of the application:
- Loads the server configuration.
- Initializes and starts the server with the specified host and port.

### `config.rs`
Defines server configuration:
- Sets default values for the `HOST` and `PORT`.
- Provides a `load` function to initialize configuration.

### `server.rs`
Server setup:
- Imports the home route and binds it to the server.
- Parses the IP address and port to run the Warp server.

### `routes/home.rs`
Defines a route for the home path (`/`):
- Uses the `hello_handler` to respond with JSON data.

### `handlers/hello.rs`
Handler for the home route:
- Returns a JSON response with a "ping" message.

### `models/response.rs`
Defines the data model for the JSON response:
- Contains a single field, `message`, that will be serialized into JSON.

## Running the Server

To start the server, run the following command:

```bash
cargo run
```

By default, the server will be available at `http://127.0.0.1:8080`.

## API Endpoint

- **GET /** - Returns a JSON message confirming the route is reachable.

### Example Response

```json
{
  "message": "ping to hello_handler"
}
```

## Dependencies

- [Warp](https://crates.io/crates/warp) - For creating a lightweight, asynchronous web server.
- [Serde](https://crates.io/crates/serde) - For serializing the JSON response.
