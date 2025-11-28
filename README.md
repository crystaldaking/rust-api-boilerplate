# Rust API Boilerplate

This is a boilerplate project for building web APIs in Rust, inspired by Laravel. It provides a structured architecture with authentication, database integration, and Docker support.

## English

### Features- **Axum** for fast HTTP server
- **JWT authentication** with bcrypt password hashing
- **PostgreSQL** with Sqlx for database operations
- **Redis** for caching/session storage
- **Docker** setup with Nginx reverse proxy
- **Migrations** for database schema
- **Logging** with Tracing
- **Modular architecture** (controllers, services, repositories, models)

### Installation
1. Install Rust (1.70+) and Cargo.
2. Install `cargo-generate` for easy template cloning:
   ```
   cargo install cargo-generate
   ```
3. Generate a new project from this boilerplate:
   ```
   cargo generate --git https://github.com/crystaldaking/rust-api-boilerplate.git --name my-api-project
   ```
   Replace `your-username` with your GitHub username. If the repo is public, this will create a local copy.

### Setup
1. Create a `.env` file (copy from `.env.example` if provided):
   ```
   DATABASE_URL=postgres://postgres:password@localhost:5432/my_db
   REDIS_URL=redis://127.0.0.1:6379
   JWT_SECRET=your-secret-key
   JWT_EXP_MINUTES=60
   APP_HOST=0.0.0.0
   APP_PORT=8000
   ```
2. Run migrations:
   ```
   cargo sqlx migrate run
   ```

### Running
- **Locally**:
  - Start Postgres and Redis (e.g., via Docker or locally).
  - `cargo run`
  - API at http://localhost:8000

- **With Docker**:
  - `docker-compose up --build`
  - Nginx at http://localhost (port 80), API at http://localhost:8000

### API Examples
- `POST /auth/register` - Register user: `{"email": "user@example.com", "password": "pass"}`
- `POST /auth/login` - Login: `{"email": "user@example.com", "password": "pass"}` → Returns JWT token
- `GET /auth/me` - Get user info (requires `Authorization: Bearer <token>`)

### Extending the Boilerplate
To add new controllers, models, etc.:
1. **Add a Model**: Create `src/models/new_model.rs`, e.g., for posts:
   ```rust
   #[derive(Debug, Serialize, Deserialize, FromRow)]
   pub struct Post { id: Uuid, title: String, content: String, user_id: Uuid, created_at: DateTime<Utc> }
   ```
2. **Add Repository**: `src/repositories/new_repository.rs` with CRUD methods using Sqlx.
3. **Add Service**: `src/services/new_service.rs` for business logic.
4. **Add Controller**: `src/http/controllers/new.rs` with handlers.
5. **Register Controller**: 
   - Add to `src/http/controllers.rs`: `pub mod new;`
   - In `src/http/routes.rs`, add routes: `Router::new().route("/new", post(controllers::new::handler))`
   - If protected, use in `protected_routes()` or add new middleware.

Add new migrations in `migrations/` and run `cargo sqlx migrate run`.

For tests, add `#[cfg(test)]` modules.

### License
MIT

## Русский

### Особенности
- **Axum** для быстрого HTTP-сервера
- **JWT-аутентификация** с хэшированием паролей через bcrypt
- **PostgreSQL** с Sqlx для работы с базой данных
- **Redis** для кэширования/сессий
- **Docker** с Nginx reverse proxy
- **Миграции** для схемы БД
- **Логирование** с Tracing
- **Модульная архитектура** (контроллеры, сервисы, репозитории, модели)

### Установка
1. Установите Rust (1.70+) и Cargo.
2. Установите `cargo-generate` для клонирования шаблона:
   ```
   cargo install cargo-generate
   ```
3. Сгенерируйте новый проект из этого boilerplate:
   ```
   cargo generate --git https://github.com/crystaldaking/rust-api-boilerplate.git --name my-api-project
   ```
   Замените `your-username` на ваш GitHub username. Если репо публичное, это создаст локальную копию.

### Настройка
1. Создайте `.env` файл (скопируйте из `.env.example`, если есть):
   ```
   DATABASE_URL=postgres://postgres:password@localhost:5432/my_db
   REDIS_URL=redis://127.0.0.1:6379
   JWT_SECRET=your-secret-key
   JWT_EXP_MINUTES=60
   APP_HOST=0.0.0.0
   APP_PORT=8000
   ```
2. Запустите миграции:
   ```
   cargo sqlx migrate run
   ```

### Запуск
- **Локально**:
  - Запустите Postgres и Redis (например, через Docker или локально).
  - `cargo run`
  - API на http://localhost:8000

- **Через Docker**:
  - `docker-compose up --build`
  - Nginx на http://localhost (порт 80), API на http://localhost:8000

### Примеры API
- `POST /auth/register` - Регистрация: `{"email": "user@example.com", "password": "pass"}`
- `POST /auth/login` - Вход: `{"email": "user@example.com", "password": "pass"}` → Возвращает JWT токен
- `GET /auth/me` - Получить инфо о пользователе (нужен `Authorization: Bearer <token>`)

### Расширение Boilerplate
Чтобы добавить новые контроллеры, модели и т.д.:
1. **Добавить Модель**: Создайте `src/models/new_model.rs`, например, для постов:
   ```rust
   #[derive(Debug, Serialize, Deserialize, FromRow)]
   pub struct Post { id: Uuid, title: String, content: String, user_id: Uuid, created_at: DateTime<Utc> }
   ```
2. **Добавить Репозиторий**: `src/repositories/new_repository.rs` с CRUD методами через Sqlx.
3. **Добавить Сервис**: `src/services/new_service.rs` для бизнес-логики.
4. **Добавить Контроллер**: `src/http/controllers/new.rs` с обработчиками.
5. **Зарегистрировать Контроллер**: 
   - Добавьте в `src/http/controllers.rs`: `pub mod new;`
   - В `src/http/routes.rs` добавьте маршруты: `Router::new().route("/new", post(controllers::new::handler))`
   - Если защищено, используйте в `protected_routes()` или добавьте новый middleware.

Добавьте новые миграции в `migrations/` и запустите `cargo sqlx migrate run`.

Для тестов добавьте модули `#[cfg(test)]`.

### Лицензия
MIT
