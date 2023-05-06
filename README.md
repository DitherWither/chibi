# chibi

Chibi is a simple URL shortener, and an api that helps you shorten urls

## Project Structure

 - `api/` Contains the backend, written in rust, and deployed to shuttle.rs
 - `frontend/` Contains the frontend, made with astro, and uses tailwind for styling
 - `justfile` is used for running tasks like running the project locally, or deploying it to shuttle

## Running locally

You need the rust toolchain, just, npm, and the shuttle cli, and docker(which is used by the shuttle cli to run locally) installed to build this project

 - Run `just dev` to run the project locally at `localhost:3030`
 - Run `just dev-host` to run the project locally, and at `0.0.0.0:8000
 - Run `just deploy` to deploy to shuttle. You should change the name in `api/Shuttle.toml` to something unique first, as I have already taken the `chibi` project name
