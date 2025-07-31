# Docker Deployment Guide

This guide explains how to build and deploy the Ox application using Docker.

## Quick Start

### Using Docker Compose (Recommended)

```bash
# Build and start the application
docker-compose up --build

# Run in background
docker-compose up -d --build

# Stop the application
docker-compose down
```

### Using Docker directly

```bash
# Build the image
docker build -t ox-app .

# Run the container
docker run -p 8080:8080 ox-app

# Run in background with name
docker run -d --name ox-app -p 8080:8080 ox-app
```

## Access the Application

Once running, the application will be available at:
- http://localhost:8080

## Docker Configuration

### Environment Variables

- `RUST_LOG`: Log level (default: `info`)
- `DIOXUS_PORT`: Port to bind to (default: `8080`)
- `DIOXUS_HOST`: Host to bind to (default: `0.0.0.0`)

### Ports

- `8080`: Main application port

## Production Deployment

For production, consider:

1. **Use a reverse proxy** (nginx, traefik, etc.)
2. **Enable HTTPS** with SSL certificates
3. **Set up health checks**
4. **Configure logging and monitoring**
5. **Use container orchestration** (Docker Swarm, Kubernetes)

### Example with nginx reverse proxy

```yaml
# Add to docker-compose.yml
nginx:
  image: nginx:alpine
  ports:
    - "80:80"
    - "443:443"
  volumes:
    - ./nginx.conf:/etc/nginx/nginx.conf:ro
    - ./ssl:/etc/nginx/ssl:ro
  depends_on:
    - ox
```

## Build Optimization

The Dockerfile uses multi-stage builds to:
- Keep the final image small (~50MB vs ~2GB build image)
- Include only runtime dependencies
- Run as non-root user for security

## Troubleshooting

### Build Issues

```bash
# Clean build (no cache)
docker build --no-cache -t ox-app .

# View build logs
docker-compose up --build --no-deps ox
```

### Runtime Issues

```bash
# View logs
docker-compose logs ox

# Access container shell
docker-compose exec ox sh

# Check if port is bound
docker-compose ps
```

### Common Problems

1. **Port already in use**: Change the port mapping in docker-compose.yml
2. **Build fails**: Ensure you have enough disk space and memory
3. **Can't access app**: Check firewall settings and port forwarding

## Development

For development with hot reload, use the native `dx serve` command instead of Docker, as Docker builds are optimized for production deployment.