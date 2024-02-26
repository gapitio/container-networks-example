# Container networks example

A small example on how to connect containers together using docker networks

## Create docker network

```sh
docker network create net-test
```

## Build docker images

### Build client docker image

```sh
docker build client -t client
```

### Build server docker image

```sh
docker build client -t client
```

## Start the containers

### Start the server container

```sh
docker compose -f ./server/compose.yaml up
```

### Start the client container

```sh
docker compose -f ./client/compose.yaml up
```
