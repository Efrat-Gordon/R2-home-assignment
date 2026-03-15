# R2 Lucky Draw

A full-stack web app where users log in and try their luck at winning a prize. Built with Next.js, Rust (Axum), PostgreSQL, and Redis.

---

## Quick Start

Install [Docker Desktop](https://www.docker.com/products/docker-desktop/), then run:

```sh
docker compose --project-name r2 up --build
```

Open [http://localhost:8080](http://localhost:8080).

---

## How It Works

### User Flow

```
┌─────────┐        POST /api/login         ┌────────────┐
│ Browser │ ─────────────────────────────► │   Rust BE  │
│         │ ◄───────────────────────────── │            │
│         │     { "token": "<uuid>" }      │            │
│         │                                │            │
│         │   POST /api/try_luck           │            │
│         │   Authorization: Bearer <token>│            │
│         │ ─────────────────────────────► │            │
│         │ ◄───────────────────────────── │            │
│         │     { "win": true/false }      └────────────┘
└─────────┘
```

### Win Probability

Every `/api/try_luck` call rolls a random number. The probability depends on how many wins have happened today:

| Daily wins so far | Win probability |
|---|---|
| < 30 | 70% |
| ≥ 30 | 40% |

---

## Architecture

```
                        ┌─────────────────────────────────────────┐
                        │              Docker Compose             │
                        │                                         │
Browser ──► :8080 ──►   │  nginx (gateway)                        │
                        │    ├── /api/*  ──► be (Rust :4000)      │
                        │    └── /*      ──► web (Next.js :3000)  │
                        │                                         │
                        │  PostgreSQL :5432                       │
                        │  Redis      :6379                       │
                        └─────────────────────────────────────────┘
```

### Services

| Service | Image | Role                                                         |
|---|---|--------------------------------------------------------------|
| `gateway` | nginx | Reverse proxy - routes `/api/*` to backend, `/*` to frontend |
| `web` | Next.js 13 | Login page + lucky draw UI                                   |
| `be` | Rust / Axum | REST API - auth, luck logic                                  |
| `db` | PostgreSQL 17 | Persistent storage for users and win history                 |
| `redis` | Redis 7 | Daily win counter cache                                      |

---

## API

Swagger UI is available at [http://localhost:8080/api/swagger-ui](http://localhost:8080/api/swagger-ui).

| Endpoint | Method | Auth | Description |
|---|---|---|---|
| `/api/login` | POST | None | Validates credentials, returns a bearer token |
| `/api/logout` | POST | Bearer token | Invalidates the token |
| `/api/try_luck` | POST | Bearer token | Rolls the dice, returns `{ "win": true/false }` |

### Login

```
POST /api/login
Content-Type: application/json

{ "email": "a@gmail.com", "password": "1234" }

→ 200 { "token": "550e8400-e29b-41d4-a716-446655440000" }
→ 401 invalid credentials
```

### Try Luck

```
POST /api/try_luck
Authorization: Bearer <token>

→ 200 { "win": true }
→ 401 not authenticated
```

---

## Database Schema

```
┌──────────────────────────────┐       ┌───────────────────────────────────────┐
│           users              │       │              win_logs                 │
├──────────────────────────────┤       ├───────────────────────────────────────┤
│ email     TEXT  PRIMARY KEY  │◄──┐   │ id         SERIAL       PRIMARY KEY   │
│ password  TEXT  NOT NULL     │   └── │ user_email TEXT         NOT NULL  FK  │
└──────────────────────────────┘       │ created_at TIMESTAMPTZ  DEFAULT NOW() │
                                       └───────────────────────────────────────┘
                                         index: idx_win_logs_created_at
```

- **`users`** - one row per registered user. Passwords are stored as **argon2 hashes** (never plaintext).
- **`win_logs`** - one row per win event, used to compute the daily win count when Redis is unavailable.

---

## Cache Layer (Redis)

Redis stores the daily win count so that `/api/try_luck` does not need to hit Postgres on every request.

```
POST /api/try_luck
        │
        ▼
┌───────────────────┐   hit   ┌─────────────────────────────┐
│  Redis            │ ──────► │  key: daily:wins:YYYY-MM-DD │
│  get_wins()       │         │  value: integer count       │
└───────────────────┘         │  TTL: 48 hours (auto-expire)│
        │                     └─────────────────────────────┘
        │ miss / error
        ▼
┌───────────────────┐
│  PostgreSQL       │
│  COUNT(*) wins    │  ← fallback, only on Redis failure
│  WHERE today      │
└───────────────────┘
        │
        ▼
  decide probability
  roll dice
        │
       win?
      ┌─┴─┐
     yes   no
      │
      ├──► INSERT INTO win_logs (PostgreSQL - source of truth)
      │
      └──► INCR daily:wins:YYYY-MM-DD (Redis - best effort, failure ignored)
```

**Key properties:**
- Redis failure is non-fatal - the request falls back to a DB count and still succeeds.
- A write to Postgres always happens before the Redis increment, so the DB is always the source of truth.
- Keys are date-scoped (`daily:wins:2026-03-15`) and expire automatically after 48 hours - no manual cleanup needed.

---

## Security

- Passwords are hashed with **argon2id** (via the `argon2` crate) before being stored. The raw password never touches the database.
- Authentication uses **UUID bearer tokens** stored in-process. Tokens are invalidated on logout.
- Password verification runs in `spawn_blocking` so it does not block the async executor.

---

## Future Work

- **TOCTOU race on daily win threshold** - concurrent requests may all read the same win count from Redis before any of them has written their win back, causing a small burst of requests to use the wrong probability at the threshold boundary. A proper fix would use a Redis Lua script to atomically read and increment the counter in a single operation, so each request gets a consistent view of the count.
- **Redis/DB divergence on win logging** - the DB write and Redis increment are two separate operations. If the Redis increment fails after a successful DB write, the cache underreports the win count until it is rebuilt from the DB on the next cache miss. A more robust approach would verify the Redis increment succeeded, or accept the divergence and always recompute from the DB as the source of truth.
---

## CI

GitHub Actions runs on every push and pull request to `master`:

| Job | Checks |
|---|---|
| `frontend` | ESLint · TypeScript (via `next build`) |
| `backend` | `cargo fmt` · `cargo clippy -D warnings` · `cargo test` |

The backend job spins up real PostgreSQL and Redis service containers so integration tests run against live dependencies.

---

## Local Development

### Frontend

```sh
cd web
npm install
npm run dev     # http://localhost:3000
npm run lint
```

### Backend

The backend is built and run inside Docker. To iterate on it quickly:

```sh
docker compose --project-name r2 up --build be
```
