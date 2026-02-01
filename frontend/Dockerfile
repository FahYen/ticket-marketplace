FROM node:20-alpine

WORKDIR /app

# Install dependencies first (cached layer)
COPY package.json package-lock.json* ./
RUN npm install

# Copy source files
COPY . .

# Expose port
EXPOSE 3001

# Enable polling for hot reload in Docker
ENV WATCHPACK_POLLING=true

# Run development server
CMD ["npm", "run", "dev"]
