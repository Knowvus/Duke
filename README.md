# SUMMARY

Duke is a middleware microservice Server that facilitates communication between:

- Frontend: Raynor [Website]
- Backend: Kerrigan [Postgres]

# GOAL

Save Token to infisical
Sync to Github Secrets
Kerrigan yaml to recall secret to join swarm as member

# Table of Contents

- [LOCAL TESTING](#local-testing)
- [NEXT STEP](#next-step)
- [DIRECTORY STRUCTURE](#directory-structure)
- [OPERATIONS](#operations)

# STATUS

[![Build and Deploy Docker Image](https://github.com/Knowvus/Duke_rs/actions/workflows/deploy.yml/badge.svg)](https://github.com/Knowvus/Duke_rs/actions/workflows/deploy.yml)

# MILESTONES

[] Create_User Integrate with Postgres
   [] Error Message for Duplicate Email
[] Create_Task Integrate with Postgres
   [] Non Null String

these endpoints should be callable via CLI [this is how we can test since it's a microservice]
Call the endpoint on the server - server servces the DB, DB responses, server serves DB response

# LOCAL TESTING
    ```
    Build Time: ~4 Mins
    ```

1) **Run the Server Locally:**
    ```
    cargo clean
    cargo build
    cargo run
    ```

2) **Test the YAML Locally**
    ```
    act -j deploy --container-architecture linux/amd64
    ```

2) **Endpints:**
    ```
    - curl http://localhost:8080/create_user -d "mkaminski1337@gmail.com"
    - curl http://localhost:8080/create_task -d "Reverse this String!"
    - curl -f http://localhost:8080/health
    ```
4
5) **CLI**

    ### Docker
     ```
    docker ps -a
    docker ps
    docker inspect [service_name]
    docker logs [service_name]
    docker rm [container_id]
    docker exec -it duke ping kerrigan
    ```

    ### Docker Swarm
    ```
    docker swarm join --token <SWARM_TOKEN> <MANAGER_IP>>:<PORT>
    docker swarm init --advertise-addr <MANAGER_IP>
    ```

    ### GENERAL
    ```
    ls
    ls -a
    sudo lsof -i :8080
    ```

    ### UPDATE PORT
    ```
    sudo nano /etc/ssh/sshd_config
    uncommment #Port=22
    ```

---

# COMING SOON

1) **API Documentation**
    ```
    OpenAPI JSON: http://<your-droplet-ip>:8080/api-doc/openapi.json
    Swagger UI: http://<your-droplet-ip>:8080/docs
    - curl -f http://localhost:8080/docs
 
    ```
   