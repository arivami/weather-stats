export $(grep -v '^#' ../.env | xargs)
DOCKERFILE="../DockerfileMulti"

BRANCH_NAME="ari-main-2"
IMAGE_NAME="app-image"


# Clone the repository
echo "Cloning repository..."
git clone -b $BRANCH_NAME https://github.com/arivami/weather-stats.git /tmp/repository

# Build the App Docker image
echo "Building Docker image..."
docker build -t $IMAGE_NAME -f $DOCKERFILE /tmp/repository

# Clean up cloned repository
echo "Cleaning up..."
rm -rf /tmp/repository

echo "Docker image build complete."
