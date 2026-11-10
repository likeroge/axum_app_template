#!/bin/bash

print_message() {
    echo "=================================="
    echo "$(date '+%Y-%m-%d %H:%M:%S') - $1"
    echo "=================================="
}

SOURCE_FILE=".env.example"
DEST_FILE=".env"

# Проверяем, что источник существует и это файл (а не папка)
if [ -f "$SOURCE_FILE" ]; then
  # Проверяем, что файл ещё не существует в целевом месте
  if [ ! -f "$DEST_FILE" ]; then
    # Копируем файл
    cp "$SOURCE_FILE" "$DEST_FILE"
    echo "Файл скопирован: $DEST_FILE"
  else
    echo "Файл уже существует: $DEST_FILE (пропуск)"
  fi
else
  echo "Исходный файл не найден: $SOURCE_FILE (пропуск)"
fi

print_message "Обновление кода"
git pull

print_message "Остановка и удаление контейнеров"
docker compose down

print_message "Пересборка образов с обновленным кодом"
docker compose up -d --build

# Финальное сообщение
print_message "Обновление завершено"
echo "Все контейнеры успешно обновлены и запущены"
