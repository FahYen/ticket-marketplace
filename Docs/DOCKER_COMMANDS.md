# Docker Commands

All commands must be run from the project root directory:
```bash
cd /Users/fah/Repos/ticket-marketplace
```

## Start Services

```bash
docker-compose up -d                    # Start all (frontend, backend, postgres)
docker-compose up -d frontend           # Start frontend only
docker-compose up -d backend            # Start backend only
docker-compose up -d --build            # Rebuild and start all
docker-compose up -d --build frontend   # Rebuild and start frontend only
```

## Stop Services

```bash
docker-compose down                     # Stop and remove all containers
docker-compose stop                     # Stop all (keep containers)
docker-compose stop frontend            # Stop frontend only
docker-compose stop backend             # Stop backend only
```

## Restart Services

```bash
docker-compose restart frontend
docker-compose restart backend
```

## View Logs

```bash
docker-compose logs -f                  # All services
docker-compose logs -f frontend         # Frontend only
docker-compose logs -f backend          # Backend only
```

## Check Status

```bash
docker-compose ps
```

## URLs

| Service  | URL                    |
|----------|------------------------|
| Frontend | http://localhost:3001  |
| Backend  | http://localhost:3000  |
| Postgres | localhost:5432         |

## Reset Database

```bash
./reset-db.sh
```
