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