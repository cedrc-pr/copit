FROM oven/bun:alpine AS libs

WORKDIR /app

COPY package.json bun.lock ./
RUN bun install

FROM oven/bun:alpine AS builder

WORKDIR /app

COPY --from=libs /app/node_modules/ node_modules/
COPY src/ src/
RUN sed -i 's|http://localhost:3000|/api|g' src/shared/config.ts
COPY package.json index.html tsconfig.json tsconfig.app.json tsconfig.node.json vite.config.ts ./
RUN bun run build

FROM nginx:alpine

RUN apk add --no-cache curl
RUN rm -rf /usr/share/nginx/html/*
COPY --from=builder /app/dist /usr/share/nginx/html

COPY nginx.conf /etc/nginx/conf.d/default.conf

EXPOSE 80

CMD ["nginx", "-g", "daemon off;"]

