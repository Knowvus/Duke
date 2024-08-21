
# Table of Contents

- [LOCAL TESTING](#local-testing)
- [NEXT STEPS](#next-steps)
- [DIRECTORY STRUCTURE](#directory-structure)
- [OPERATIONS](#operations)

# STATUS

[![Build and Deploy Docker Image](https://github.com/Knowvus/Duke_rs/actions/workflows/deploy.yml/badge.svg)](https://github.com/Knowvus/Duke_rs/actions/workflows/deploy.yml)

# LOCAL TESTING

```
Build Time: ~4 Mins
```

1) **Run the Server Locally:**

    ```bash
    cargo clean
    cargo build
    cargo run
    ```

2) **Endpoints:**

    ```bash
    # Create Task Endpoint
    curl -X POST http://localhost:8080/create_task -d '{"body": "Hello World"}' -H "Content-Type: application/json"

    # Create User Endpoint
    curl -X POST http://localhost:8080/create_user

    # Health Check Endpoint
    curl -f http://localhost:8080/health
    ```

3) **Docker Commands:**

    ```bash
    docker ps -a
    docker ps
    docker inspect [service_name]
    docker logs [service_name]
    docker rm [container_id]
    ls
    ls -a
    sudo lsof -i :8080
    ```

## COMING SOON

4) **Swagger API Documentation:**

    ```bash
    # OpenAPI JSON
    http://<your-droplet-ip>:8080/api-doc/openapi.json

    # Swagger UI
    http://<your-droplet-ip>:8080/docs

    # Local Swagger UI Setup
    curl -f http://localhost:8080/docs
    curl -LO https://github.com/swagger-api/swagger-ui/archive/refs/heads/master.zip
    unzip master.zip
    ```

---

# TECHNICAL RESOURCES

## DIRECTORY STRUCTURE

```plaintext
C:\Users\michael\Documents\Duke_rs\
│
├── .github           # GitHub workflows and actions
├── .vscode           # Visual Studio Code settings
├── logging           # Logging-related modules
├── logs              # Log files
├── src               # Source code directory
│   ├── handlers.rs   # Request handlers for various endpoints
│   ├── main.rs       # Entry point of the application
│   └── routes.rs     # Route definitions and route handling logic
├── target            # Compiled files
├── .dockerignore     # Docker ignore file
├── .gitignore        # Git ignore file
├── Cargo.lock        # Cargo lock file
├── Cargo.toml        # Cargo configuration file
├── Dockerfile        # Docker configuration file
├── LICENSE           # License file
└── README.md         # Readme file (you are here)
```

## OPERATIONS

### `src/routes.rs`

**Purpose:** Defines the route handling logic and integrates the handlers for the API endpoints.

**Example:** The file contains logic to handle requests for creating tasks, creating users, and checking server health.

### `src/handlers.rs`

**Purpose:** Contains the handler functions for various endpoints.

**Example:** Defines functions like `create_task` and `create_user`, which are used to handle incoming API requests.

### `src/main.rs`

**Purpose:** The entry point of the application.

**Example:** Initializes the server, sets up logging, and starts the application with the configured routes.
