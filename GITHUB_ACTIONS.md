# GitHub Actions & Container Registry Setup

This repository automatically builds and publishes Docker images to GitHub Container Registry (GHCR) using GitHub Actions.

## Workflow Overview

The `docker-publish.yml` workflow:
- **Triggers**: Pushes to `main`/`develop`, tags starting with `v*`, PRs to `main`, or manual dispatch
- **Builds**: Multi-platform Docker images (linux/amd64, linux/arm64)
- **Publishes**: To `ghcr.io/[username]/ox`
- **Caches**: Uses GitHub Actions cache for faster builds
- **Attestation**: Generates build provenance for security

## Image Tags

The workflow creates these tags automatically:
- `latest` - Latest build from main branch
- `main` - Latest build from main branch  
- `develop` - Latest build from develop branch
- `v1.2.3` - Semantic version tags
- `v1.2` - Major.minor version
- `v1` - Major version only
- `pr-123` - Pull request builds

## Setup Requirements

### Repository Permissions
1. Go to **Settings** → **Actions** → **General**
2. Set **Workflow permissions** to "Read and write permissions"
3. Check "Allow GitHub Actions to create and approve pull requests"

### Package Permissions  
1. Go to **Settings** → **Actions** → **General**
2. Under **Fork pull request workflows**, select appropriate permissions

## Using the Published Images

### Pull and Run
```bash
# Latest version
docker run -p 8080:8080 ghcr.io/[username]/ox:latest

# Specific version
docker run -p 8080:8080 ghcr.io/[username]/ox:v1.0.0

# With podman
podman run -p 8080:8080 ghcr.io/[username]/ox:latest
```

### Docker Compose
```yaml
version: '3.8'
services:
  ox:
    image: ghcr.io/[username]/ox:latest
    ports:
      - "8080:8080"
    environment:
      - RUST_LOG=info
```

### Kubernetes Deployment
```yaml
apiVersion: apps/v1
kind: Deployment
metadata:
  name: ox-app
spec:
  replicas: 3
  selector:
    matchLabels:
      app: ox
  template:
    metadata:
      labels:
        app: ox
    spec:
      containers:
      - name: ox
        image: ghcr.io/[username]/ox:latest
        ports:
        - containerPort: 8080
        env:
        - name: RUST_LOG
          value: "info"
---
apiVersion: v1
kind: Service
metadata:
  name: ox-service
spec:
  selector:
    app: ox
  ports:
  - port: 80
    targetPort: 8080
  type: LoadBalancer
```

## Authentication for Private Images

If the repository is private, authenticate with GHCR:

```bash
# Create a personal access token with 'read:packages' scope
echo $GITHUB_TOKEN | docker login ghcr.io -u USERNAME --password-stdin

# Or with podman
echo $GITHUB_TOKEN | podman login ghcr.io -u USERNAME --password-stdin
```

## Local Development

For local development, continue using:
```bash
# Build locally
docker build -t ox-local .

# Or with podman
podman build -t ox-local .

# Use docker-compose for development
docker-compose up --build
```

## Monitoring Builds

- **Actions Tab**: View build status and logs
- **Packages**: See published images at `https://github.com/[username]/ox/pkgs/container/ox`
- **Security**: Build attestations provide supply chain security

## Troubleshooting

### Build Failures
1. Check the Actions tab for detailed logs
2. Common issues:
   - Dockerfile syntax errors
   - Missing dependencies
   - Insufficient permissions
   - Network timeouts during Rust compilation

### Permission Issues
1. Verify repository has package write permissions
2. Check if organization has package restrictions
3. Ensure GITHUB_TOKEN has correct scopes

### Image Pull Issues
1. Verify image exists: `docker pull ghcr.io/[username]/ox:latest`
2. Check if image is public or if you're authenticated
3. Try different tags (latest, main, specific versions)

## Security Best Practices

The workflow includes:
- ✅ Minimal required permissions
- ✅ Build provenance attestation
- ✅ Multi-platform builds
- ✅ Layer caching for efficiency
- ✅ Automatic vulnerability scanning (via GitHub)

## Manual Trigger

You can manually trigger builds:
1. Go to **Actions** tab
2. Select "Build and Publish Docker Image"
3. Click "Run workflow"
4. Choose branch and click "Run workflow"