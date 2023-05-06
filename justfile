export CARGO_TERM_COLOR := "always"
export CARGO_REGISTRIES_CRATES_IO_PROTOCOL := "sparse"

build-frontend:
    rm -rf api/dist
    pnpm run  --dir frontend build
    cp -r dist api/dist

dev: build-frontend
    cargo shuttle run --working-directory api --port 3030

dev-host: build-frontend
    cargo shuttle run --working-directory api --external

deploy: build-frontend
    cargo shuttle deploy --working-directory api