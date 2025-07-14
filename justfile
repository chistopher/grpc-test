# Define variables

SERVER_IMAGE := "grpc-server:latest"
CLIENT_IMAGE := "grpc-client:latest"

# Build the server Docker image
build-server:
    podman build -t {{ SERVER_IMAGE }} -f Dockerfile.server .

# Build the client Docker image
build-client:
    podman build -t {{ CLIENT_IMAGE }} -f Dockerfile.client .

# Run the server container using the host network
run-server:
    podman run --rm --name grpc-server --network host {{ SERVER_IMAGE }}

# Run the client container using the host network
run-client:
    podman run --rm --name grpc-client --network host {{ CLIENT_IMAGE }}
