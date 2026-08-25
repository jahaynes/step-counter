#!/bin/bash
podman run -d --name step_counter_postgres -e POSTGRES_PASSWORD=mysecretpassword -p5432:5432 docker.io/library/postgres:latest
