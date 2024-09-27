# SUMMARY

Duke is a middleware microservice Server that facilitates communication between:

- Frontend: Raynor [Website]
- Backend: Kerrigan [Postgres]

# GOAL

1) Refactor to Actions Github repo
2) Kerrigan Deployment
3) Duke Communicate with Kerrigan
4) Raynor Deployment

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
    sudo mkdir -p /run/sshd
    sudo chmod 0755 /run/sshd
    sudo systemctl restart ssh
    sudo systemctl status ssh
    ```

    ### BASIC FIREWALL

    INSTALL
    ```
    sudo apt update
    sudo apt install fail2ban
    sudo systemctl enable fail2ban
    sudo systemctl start fail2ban
    ```

    UPDATE CONFIG: add to /etc/ssh/sshd_config
    ```
    Match Address 218.92.0.81
        PermitRootLogin yes
    ```
    RUN CMD
    ```
    sudo systemctl restart ssh
    ```

    UPDATE CONFIG: add to /etc/fail2ban/jail.local on droplet:
    ```
    sudo ufw status
    sudo ufw allow ssh
    sudo ss -tuln | grep :22
    [sshd]
    enabled = true
    ignoreip = 127.0.0.1/8 218.92.0.81
    [sshd]
    enabled = true
    port = ssh
    logpath = %(sshd_log)s
    maxretry = 5
    bantime = 10m
    findtime = 10m
    ```
    RUN: CMD
    ```
    sudo systemctl restart fail2ban
    sudo fail2ban-client status sshd
    ```

---

# COMING SOON

1) **API Documentation**
    ```
    OpenAPI JSON: http://<your-droplet-ip>:8080/api-doc/openapi.json
    Swagger UI: http://<your-droplet-ip>:8080/docs
    - curl -f http://localhost:8080/docs
 
    ```
   