# BotFaz2077

[![GitHub Workflow Status](https://img.shields.io/github/workflow/status/DarthFaz2077/BotFaz2077/Build%20and%20push%20new%20version%20to%20DockerHub?style=for-the-badge)](https://github.com/DarthFaz2077/BotFaz2077/actions/workflows/push.yml)
[![Docker Pulls](https://img.shields.io/docker/pulls/darthfaz2077/botfaz2077?style=for-the-badge)](https://hub.docker.com/r/darthfaz2077/botfaz2077)
[![Docker Image Size](https://img.shields.io/docker/image-size/darthfaz2077/botfaz2077/latest?style=for-the-badge)](https://hub.docker.com/r/darthfaz2077/botfaz2077)
[![License](https://img.shields.io/github/license/DarthFaz2077/BotFaz2077?style=for-the-badge)](https://github.com/DarthFaz2077/BotFaz2077/blob/main/LICENSE)

Simple Bot for Discord.

## Usage

1. Copy docker-compose.yml.example to docker-compose.yml and update as needed. See example below:

```yaml
version: "3"

services:
  postgres:
    image: postgres:alpine
    volumes:
      - ./data/postgres:/var/lib/postgresql/data
    environment:
      POSTGRES_PASSWORD: "your_db_password"
    restart: unless-stopped

  botfaz2077:
    image: darthfaz2077/botfaz2077:latest
    environment:
      DISCORD_TOKEN: "your_token"
      PREFIX: "bf!"
      ACTIVITY: "with rust"
      POSTGRES_URL: "postgres://postgres:your_db_password@postgres/postgres"
      RUST_LOG: "info"
    depends_on:
      - postgres
    restart: unless-stopped
```

2. Run `docker-compose up --detach`.
