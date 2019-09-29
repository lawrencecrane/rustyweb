# Building own web server implementation with Rust

Build container image with `./build.sh` and run it with `./run.sh` (using either Docker or first running `alias docker=podman`). Web app should be available at port 8080 on your host.

## Development

To silence your IDEs error about bundle.js that is generated in build time, you may want to run `touch app/client/dist/bundle.js` to just create an empty file.
