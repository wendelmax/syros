# Syros Platform Helm Chart

This Helm chart deploys the Syros Platform - a distributed coordination service built in Rust.

## Prerequisites

- Kubernetes 1.19+
- Helm 3.0+
- Redis (included as dependency)
- PostgreSQL (included as dependency)
- Consul (included as dependency)

## Installing the Chart

To install the chart with the release name `syros-platform`:

```bash
helm repo add syros https://charts.syros.com
helm install syros-platform syros/syros-platform
```

## Uninstalling the Chart

To uninstall/delete the `syros-platform` deployment:

```bash
helm delete syros-platform
```

## Configuration

The following table lists the configurable parameters and their default values.

| Parameter | Description | Default |
|-----------|-------------|---------|
| `replicaCount` | Number of replicas | `3` |
| `image.repository` | Image repository | `syros-platform` |
| `image.tag` | Image tag | `1.0.0` |
| `image.pullPolicy` | Image pull policy | `IfNotPresent` |
| `service.type` | Service type | `ClusterIP` |
| `service.port` | HTTP port | `8080` |
| `service.grpcPort` | gRPC port | `9090` |
| `service.websocketPort` | WebSocket port | `8081` |
| `ingress.enabled` | Enable ingress | `false` |
| `resources.limits.cpu` | CPU limit | `1000m` |
| `resources.limits.memory` | Memory limit | `1Gi` |
| `resources.requests.cpu` | CPU request | `250m` |
| `resources.requests.memory` | Memory request | `512Mi` |
| `autoscaling.enabled` | Enable HPA | `false` |
| `autoscaling.minReplicas` | Min replicas | `3` |
| `autoscaling.maxReplicas` | Max replicas | `10` |
| `config.serviceDiscovery.enabled` | Enable service discovery | `true` |
| `config.serviceDiscovery.consulUrl` | Consul URL | `http://consul:8500` |
| `redis.enabled` | Enable Redis | `true` |
| `postgresql.enabled` | Enable PostgreSQL | `true` |
| `consul.enabled` | Enable Consul | `true` |

## Examples

### Basic Installation

```bash
helm install syros-platform ./helm/syros-platform
```

### With Custom Configuration

```bash
helm install syros-platform ./helm/syros-platform \
  --set replicaCount=5 \
  --set config.serviceDiscovery.enabled=true \
  --set resources.limits.memory=2Gi
```

### With Ingress

```bash
helm install syros-platform ./helm/syros-platform \
  --set ingress.enabled=true \
  --set ingress.hosts[0].host=syros.example.com \
  --set ingress.hosts[0].paths[0].path=/ \
  --set ingress.hosts[0].paths[0].pathType=Prefix
```

### With External Dependencies

```bash
helm install syros-platform ./helm/syros-platform \
  --set redis.enabled=false \
  --set postgresql.enabled=false \
  --set consul.enabled=false \
  --set config.storage.redis.url=redis://external-redis:6379 \
  --set config.database.url=postgresql://external-postgres:5432/syros \
  --set config.serviceDiscovery.consulUrl=http://external-consul:8500
```

## Monitoring

The chart includes optional Prometheus and Grafana monitoring:

```bash
helm install syros-platform ./helm/syros-platform \
  --set monitoring.enabled=true
```

## Security

For production deployments, make sure to:

1. Change default passwords and secrets
2. Enable TLS/SSL
3. Configure network policies
4. Use proper RBAC

```bash
helm install syros-platform ./helm/syros-platform \
  --set config.security.jwtSecret=your-secure-jwt-secret \
  --set config.security.apiKeyEncryptionKey=your-secure-api-key \
  --set postgresql.auth.postgresPassword=your-secure-password
```

## Troubleshooting

### Check Pod Status

```bash
kubectl get pods -l app.kubernetes.io/name=syros-platform
```

### View Logs

```bash
kubectl logs -l app.kubernetes.io/name=syros-platform
```

### Check Service Discovery

```bash
kubectl exec -it deployment/syros-platform -- curl http://localhost:8080/health
```

### Port Forward for Testing

```bash
# HTTP API
kubectl port-forward svc/syros-platform 8080:8080

# gRPC API
kubectl port-forward svc/syros-platform 9090:9090

# WebSocket API
kubectl port-forward svc/syros-platform 8081:8081
```
