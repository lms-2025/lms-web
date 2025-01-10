#!/bin/bash

# Function to check if Docker is installed
check_docker_installed() {
    if ! command -v docker &> /dev/null; then
        echo "Docker is not installed. Please install Docker first."
        exit 1
    fi
}

# Function to check and start Docker daemon
start_docker_daemon() {
    echo "Checking if Docker daemon is running..."
    docker info > /dev/null 2>&1
    if [ $? -ne 0 ]; then
        echo "Docker daemon is not running. Attempting to start it..."
        
        # Check if systemctl is available
        if command -v systemctl &> /dev/null; then
            # Try to start Docker using systemctl
            sudo systemctl start docker 2>/dev/null
            if [ $? -eq 0 ]; then
                echo "Docker daemon started using systemctl."
            else
                echo "Failed to start Docker with systemctl. Trying Docker Desktop for Linux..."
                systemctl --user start docker-desktop
                if [ $? -eq 0 ]; then
                    echo "Docker Desktop for Linux started successfully."
                else
                    echo "Failed to start Docker. Please check your installation."
                    exit 1
                fi
            fi
        else
            # If systemctl is not available, try starting dockerd manually
            echo "Systemctl not found. Starting dockerd manually..."
            dockerd &
            sleep 5
            if [ $? -eq 0 ]; then
                echo "Docker daemon started manually."
            else
                echo "Failed to start Docker manually. Please check your setup."
                exit 1
            fi
        fi
    else
        echo "Docker daemon is running."
    fi

    # Wait until Docker daemon is fully operational
    echo "Waiting for Docker daemon to be ready..."
    until docker info > /dev/null 2>&1; do
        sleep 2
    done
    echo "Docker daemon is now ready."
}

# Main script logic
check_docker_installed
start_docker_daemon

# Run Docker Compose
echo "Starting Docker Compose..."
docker compose up -d

# Check if Docker Compose was successful
if [ $? -eq 0 ]; then
    echo "Docker Compose started successfully."

    # Run cargo watch after Docker Compose is finished
    echo "Starting Cargo watch..."
    cargo watch -w src -x 'run'
else
    echo "Failed to start Docker Compose. Exiting."
    exit 1
fi
