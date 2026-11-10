# Этап 1: сборка фронтенда (Node.js)

# FROM node:20-alpine AS frontend-builder

# WORKDIR /app/front

# Копируем package.json и package-lock.json для кэширования зависимостей
# COPY front/package.json front/package-lock.json ./

# Устанавливаем зависимости
# RUN npm ci

# Копируем исходный код фронтенда
# COPY front ./

# Собираем фронтенд (с передачей переменной окружения)
# RUN npm run build

# Этап 2: сборка бэкенда (Rust)

# ─── Этап 1: Шеф готовит «рецепт» зависимостей ───────────────────────
FROM rust:1.99-slim AS chef
RUN cargo install cargo-chef
WORKDIR /app

# ─── Этап 2: Считаем рецепт (инвалидируется только при изменении Cargo.toml) ──
FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

# ─── Этап 3: Собираем зависимости (долгий слой, кешируется) ──────────
FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json
# К этому моменту все крейты скачаны и скомпилированы в кеше Docker


# Теперь копируем исходники — этот слой инвалидируется при каждом изменении кода,
# но зависимости уже в кеше, пересобирается только ваш код
COPY . .
ENV SQLX_OFFLINE=true
RUN cargo build --release

# ─── Этап 4: Рантайм — минимальный образ ─────────────────────────────
FROM debian:bookworm-slim AS runtime
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*
# Создаём непривилегированного пользователя
RUN useradd -m -u 1000 appuser

WORKDIR /app
COPY --from=builder /app/target/release/app /usr/local/bin/app
COPY --from=builder /app/static /app/static
COPY --from=builder /app/migrations /app/migrations
COPY --from=builder /app/.env.example /app/.env

# .env не копируем внутрь образа — переменные передаются при запуске (env_file)

VOLUME ["/app/data"]

USER appuser
EXPOSE 5005

# CMD ["app"]
