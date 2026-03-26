# hello_rust

Backend API sederhana berbasis Rust + Axum dengan contoh CRUD `users`, validasi input, dan format response JSON yang konsisten.

## Tech Stack

- Rust `1.94.0`
- Cargo `1.94.0`
- Edition: `2024`
- Framework HTTP: `axum = 0.8`
- Async runtime: `tokio = 1`
- Serialization: `serde = 1`
- Logging: `tracing`, `tracing-subscriber`
- Password hashing: `bcrypt = 0.17`

## Fitur Saat Ini

- Struktur project modular (`app`, `config`, `routes`, `state`, `error`)
- Health check endpoint
- CRUD user (in-memory)
- Validasi email/username
- Cek unique email dan username
- Password opsional saat create user (jika diisi, di-hash bcrypt)
- Response API standar:
  - success: `"status": "success"`
  - error: `"status": "fail"` + `"code"` + `"message"`

## Prasyarat

Pastikan sudah install:

- `git`
- `rustup` (untuk Rust toolchain)

Cek versi:

```bash
rustc --version
cargo --version
```

## Cara Clone

```bash
git clone <URL_REPOSITORY_KAMU>
cd hello_rust
```

## Setelah Clone (Setup Awal)

1. Download dependency dan cek compile:

```bash
cargo check
```

2. (Opsional) Jalankan test:

```bash
cargo test
```

3. (Opsional) Cek lint:

```bash
cargo clippy --all-targets --all-features
```

## Konfigurasi Environment

Project ini membaca env berikut:

| Variable | Default | Keterangan |
|---|---|---|
| `APP_SERVICE_NAME` | `hello_rust` | Nama service |
| `APP_HOST` | `127.0.0.1` | Host bind server |
| `APP_PORT` | `3000` | Port server |
| `RUST_LOG` | `info,hello_rust=debug` | Level logging |

Contoh run dengan env custom:

```bash
APP_HOST=127.0.0.1 APP_PORT=8080 RUST_LOG=debug cargo run
```

Untuk PowerShell:

```powershell
$env:APP_HOST="127.0.0.1"
$env:APP_PORT="8080"
$env:RUST_LOG="debug"
cargo run
```

## Menjalankan Project

Jalankan server:

```bash
cargo run
```

Default URL:

- `http://127.0.0.1:3000`

## Auto Reload Saat Development

Install watcher:

```bash
cargo install cargo-watch
```

Run auto restart:

```bash
cargo watch -x run
```

## API Endpoints

### Root

- `GET /`

### Health

- `GET /health`

### Users

- `GET /users`
- `GET /users/{id}`
- `POST /users`
- `PUT /users/{id}`
- `DELETE /users/{id}`

## Contoh Request

Create user:

```bash
curl -X POST http://127.0.0.1:3000/users \
  -H "Content-Type: application/json" \
  -d '{
    "email": "testing123@email.com",
    "username": "tester1",
    "password": "mypassword123",
    "fullname": "Testing User"
  }'
```

Update user:

```bash
curl -X PUT http://127.0.0.1:3000/users/1 \
  -H "Content-Type: application/json" \
  -d '{
    "fullname": "Testing User Updated"
  }'
```

## Format Response

Success:

```json
{
  "status": "success",
  "data": {}
}
```

Error:

```json
{
  "code": "BAD_REQUEST",
  "status": "fail",
  "message": "..."
}
```

## Catatan Penting

- Data user saat ini disimpan in-memory (`HashMap`), jadi akan hilang saat server restart.
- `password_hash` disimpan di model internal, tapi tidak dikirim ke response JSON.

## Dokumentasi Tambahan di Repo

- `CARGO_COMMANDS.md`
- `RUST_AXUM_CHEATSHEET.md`
- `RUST_NAMING_CONVENTIONS.md`
