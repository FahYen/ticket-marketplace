#!/bin/bash
# Reset database for development
# WARNING: This deletes all data!
docker-compose down -v
docker-compose up --build
