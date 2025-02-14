# Stress HTTP Server

A simple, multi-threaded HTTP server implemented in Rust. This project is designed to handle basic HTTP requests and serve static files. This Projects was made for learning purpose so that I can get used to rust's Concurrency model and borrow system.

This is my first time with rust so it might be horrible.

## Table of Contents

- [Features](#features)
- [Getting Started](#getting-started)
  - [Prerequisites](#prerequisites)
  - [Installation](#installation)
- [Example Usage](#example-usage)
  - [Creating The Server](#creating-the-server)
  - [Defining Routes](#defining-routes)
  - [Serving Static Files](#serving-static-files)
  - [Middleware](#adding-middleware)
  - [Error Handling](#error-handling)
- [Limitations](#limitations)
- [RoadMap](#roadmap)
- [Project Structure](#project-structure)
- [Contributing](#contributing)
- [License](#license)
- [Acknowledgments](#acknowledgments)

## Features

- **Multi-threaded**: Uses a thread pool to handle multiple incoming connections concurrently.
- **Routing**: Supports basic routing for HTTP methods (GET, POST, PUT, DELETE, PATCH).
- **Middleware**: Allows global middleware to be applied to all routes.
- **Error Handling**: Custom error handlers for request parsing and route handling.
- **Static File Serving**: Can serve static files with proper MIME types.

## Getting Started

### Prerequisites

- Rust installed on your machine. If you don't have Rust installed, follow the instructions at [rustup.rs](https://rustup.rs/).

### Installation

1. Clone the repository:
   ```bash
   git clone https://github.com/yukiisen/stress.git
   ```

2. Include the module in your project:
    ```toml
    [dependencies]
    stress = { path = "./path/to/cloned/repository" }
    ```

3. Build the project:
   ```bash
   cargo build --release
   ```

### Example Usage

***Note:*** the request handlers but be passed as `Box<Fn(Request, Response) + 'static + Sync + Send>`, but this shall be changed later. 

#### Creating The Server
```rust
use stress::{HTTPServer, Request, Response};

fn main() {
    let mut server = HTTPServer::new(1);
    server.listen("127.0.0.1:8080").unwrap(); // Start Listening for connections.
}
```

#### Defining Routes
You can define routes in your Rust code like this:

```rust
use stress::*;
use request::Request;
use response::Response;

fn main() {
    let mut server = HTTPServer::new(4); // 4 worker threads

    server.get("/", |req: &mut Request, res: &mut Response| {
        res.send("Hello, World!").unwrap();
        Ok(true)
    });

    server.listen("127.0.0.1:8080").unwrap();
}
```

**Note that you can't define routes after running the `listen` method since It blocks the current thread.**

#### Serving Static Files
To serve static files, use the `send_file` method in a route handler:

```rust
server.get("/file", |req: &mut Request, res: &mut Response| {
    res.send_file("path/to/your/file.txt").unwrap();
    Ok(true)
});
```

Or use the built-in `static_server` middleware:
```rust
use stress::middlewares::static_serve::serve_static;
...
server.get("*", static_serve("./assets"));
```

#### The RouteHandler type
The route handler functions must return `Result<bool>`, where the returned value is either an error or a `bool` the determines whether to end the response here or continue to the next handler.

```rust
server.get("/", |req: &mut Request, res: &mut Response| {
    res.send("Hello, World!").unwrap();
    Ok(true) // true means end the response and close connection.
});
```

#### Adding Middleware
You can add global middleware that runs before all routes:

```rust
server.middleware("*", |req: &mut Request, res: &mut Response| {
    println!("Middleware: Request to {}", req.path);
    Ok(false) // Continue to the next handler
});
```

#### Finals
Final handlers work similar to middlewares but instead of running before everything, they run after everything is done.

```rust
server.last("*", |req: &mut Request, res: &mut Response| {
    println!("Final: Request to {}", req.path);
    Ok(true)
});
```

#### Error Handling
Though error handling is a little bit so horrible, I tried to implement some solutions.

If a middleware, route or a final returns an `Err`, the controller will pass the request directly to the error handling routes.

To define an error handling route use:
```rust
server.error_ware("*", |req: &mut Request, res: &mut Response| {
    eprintln!("{}", req.error); // Access the error using `req.error` property.
    Ok(true)
});
```

Any errors returned by an error handler are ignored.

### Configuration
The server can be configured by modifying the `HTTPServer` initialization:
- **Number of Worker Threads**: Pass the desired number of threads to `HTTPServer::new`.
- **Listening Address**: Change the address in the `listen` method.
- **Req/Res Parsing Errors**: use the `on_error` method to adda handler for such errors.

### Limitations

- **Error Handling**: The current error handling implementation is basic and may not cover all edge cases. Contributions to improve this are welcome!
- **Concurrency**: While the server uses a thread pool to handle concurrent connections, there may be room for optimization in how resources are shared between threads.
- **Performance**: This server is designed for learning purposes and may not be suitable for high-traffic production environments.


## Roadmap

- [ ] Improve error handling for request parsing and response generation.
- [ ] Add POST/PUT body parsing.
- [ ] Implement a proper shutdown instead of the default rust behavior.
- [ ] Add support for HTTPS.
- [ ] Implement a more efficient routing mechanism (using a trie data structure).
- [ ] Add benchmarks to measure and improve performance.

## Project Structure

- `src/`: Contains the Rust source code.
  - `lib.rs`: Main server logic, including the `HTTPServer` struct and route registration.
  - `controller.rs`: Handles incoming requests and applies middleware and route handlers.
  - `router.rs`: Defines the `Route` struct and route handlers.
  - `request.rs`: The Request struct, Parses incoming HTTP requests.
  - `response.rs`: The Response struct, Constructs and sends HTTP responses.
  - `pool.rs`: Manages the thread pool for handling concurrent connections.
- `tests/`: Contains unit tests for helper functions.

## Contributing

Contributions are welcome! If you'd like to fix this horrible code (I really wish it's not that horrible), Please read the steps above:

1. Fork the repository.
2. Create a new branch (`git checkout -b feature/YourFeature`).
3. Commit your changes (`git commit -m 'Add some feature'`).
4. Push to the branch (`git push origin feature/YourFeature`).
5. Open a pull request.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

## Acknowledgments

- This project has multiple problems with error handling and concurrency handling.
- If you're planning to use this in a project, please consider trying an actuall HTTP implementation.
