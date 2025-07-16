# Define variables

SERVER_IMAGE := "grpc-server:latest"
CLIENT_IMAGE := "grpc-client:latest"

# setup minikube
setup-minikube:
    minikube config set driver podman
    minikube config set rootless true
    minikube addons enable ingress
    minikube start --container-runtime=containerd

remove-minikube:
    minikube stop
    minikube delete

# Build the server Docker image
build-server:
    podman build -t {{ SERVER_IMAGE }} -f Dockerfile.server .
    podman save {{ SERVER_IMAGE }} -o image.tar
    minikube image load image.tar
    rm image.tar

# Build the client Docker image
build-client:
    podman build -t {{ CLIENT_IMAGE }} -f Dockerfile.client .
    podman save {{ CLIENT_IMAGE }} -o image.tar
    minikube image load image.tar
    rm image.tar

reset-deployment:
    kubectl delete deployment client-deployment
    kubectl create -f k8s/client-deployment.yaml

debug-run:
    kubectl run test-pod -it --rm --image=busybox --restart=Never -- sh

# Run the server container using the host network
run-server:
    podman run --rm --name grpc-server --network host {{ SERVER_IMAGE }}

# Run the client container using the host network
run-client:
    podman run --rm --name grpc-client --network host {{ CLIENT_IMAGE }}
