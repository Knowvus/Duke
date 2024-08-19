# LOCAL TESTING

1) Run the Server Locally:
    ```
    cargo run
    ```

2) Send Endpoint POST
   - Access `http://localhost:3030/reverse` via Postman or `curl` to test.


3) Review Secrets
    ```
    infisical secrets --env=prod

    Other Envs:

    staging
    dev
    ```
# NEXT STEP

1) Resolve docker deployment issue - after pushing to github, docker image pulls then application immediatly exits
```
docker ps -a
docker ps
docker rm [image]
ls
ls -a
```

---

# TECHNICAL RESOURCES
## DIRECTORY STRUCTURE
```
src/
│
├── main.rs              # Entry point of the application
├── handlers/
│   ├── mod.rs           # Module file for the handlers
│   └── task.rs          # Task-related handler functions
│   └── user.rs          # User-related handler functions
├── routes/
│   └── mod.rs           # Module file for routes
└── apidoc/              # Directory for OpenAPI documentation
    ├── mod.rs           # Module file for OpenAPI documentation
    └── openapi.rs       # Defines the OpenAPI schema using utoipa
```

## OPERATIONS
```
src/routes/:

Purpose: This folder contains all the route handlers for your API endpoints.
Example: items.rs defines the route handling logic for the /items endpoint.
src/schemas/:

Purpose: This folder contains all your schema definitions that are used in the OpenAPI documentation.
Example: item.rs defines the Item struct and implements the necessary traits (ToSchema or Apiv2Schema).
src/apidoc/:

Purpose: This folder is dedicated to OpenAPI documentation-related logic.
mod.rs: This file exposes the OpenAPI documentation logic.
openapi.rs: This file is where you define your OpenAPI schema using utoipa or paperclip. It will pull in schemas from the schemas/ directory and routes from routes/ to build the complete OpenAPI documentation.
src/utils/:

Purpose: This folder can store utility functions or modules that are shared across different parts of your application (e.g., error handling).
```