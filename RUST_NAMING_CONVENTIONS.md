# Rust + Axum Naming Conventions

Dokumen ini jadi pedoman penamaan code di project Rust (terutama service Axum), supaya konsisten, mudah dibaca, dan idiomatik sesuai ekosistem Rust.

## Ringkasan Cepat

| Jenis                 | Case yang disarankan   | Contoh                               |
| --------------------- | ---------------------- | ------------------------------------ |
| Function / method     | `snake_case`           | `create_user`, `validate_email`      |
| Variable lokal        | `snake_case`           | `user_id`, `password_hash`           |
| Module / file         | `snake_case`           | `routes/users.rs`, `app_state.rs`    |
| Struct / Enum / Trait | `PascalCase`           | `User`, `AppError`, `UserRepository` |
| Type alias            | `PascalCase`           | `AppResult<T>`                       |
| Constant / static     | `SCREAMING_SNAKE_CASE` | `DEFAULT_COST`, `MAX_PAGE_SIZE`      |
| Crate/package name    | `snake_case`           | `hello_rust`                         |

Catatan: `camelCase` bukan gaya idiomatik Rust untuk identifier code.

## 1. Function dan Method

Gunakan `snake_case`.

Contoh:

```rust
pub async fn create_user() {}
fn validate_username(username: &str) -> bool { !username.trim().is_empty() }
```

Tidak disarankan:

- `createUser`
- `CreateUser`

## 2. Struct, Enum, Trait

Gunakan `PascalCase`.

Contoh:

```rust
pub struct CreateUserRequest {}
pub enum AppError {}
pub trait UserRepository {}
```

## 3. Field Struct dan Variabel

Gunakan `snake_case`.

Contoh:

```rust
pub struct User {
    pub id: u64,
    pub password_hash: Option<String>,
}
```

## 4. Constants

Gunakan `SCREAMING_SNAKE_CASE`.

Contoh:

```rust
const MIN_PASSWORD_LEN: usize = 8;
const DEFAULT_PAGE_SIZE: usize = 20;
```

## 5. File dan Module

Gunakan nama file/module `snake_case`.

Contoh struktur:

```text
src/
  app.rs
  config.rs
  routes/
    mod.rs
    users.rs
```

## 6. Khusus Axum: Handler dan Route

Nama handler tetap `snake_case`.

Contoh:

```rust
Router::new()
    .route("/users", get(list_users).post(create_user))
    .route("/users/{id}", get(get_user).put(update_user).delete(delete_user));
```

Handler:

- `list_users`
- `get_user`
- `create_user`
- `update_user`
- `delete_user`

## 7. JSON Naming untuk API

Secara default, serde akan memakai nama field Rust apa adanya (`snake_case`).

Contoh output default:

```json
{
  "password_hash": "..."
}
```

Kalau API contract butuh `camelCase`, tetap simpan field Rust dalam `snake_case`, lalu map pakai serde:

```rust
#[derive(serde::Serialize)]
#[serde(rename_all = "camelCase")]
struct UserResponse {
    user_id: u64,
    created_at: String,
}
```

Output:

```json
{
  "userId": 1,
  "createdAt": "2026-03-26T10:00:00Z"
}
```

Rekomendasi:

- Internal code Rust: tetap idiomatik (`snake_case`/`PascalCase`).
- Bentuk JSON eksternal: ikuti kebutuhan API contract via serde attributes.

## 8. Kapan Pakai Apa (Praktis)

| Kondisi                            | Gunakan                                           |
| ---------------------------------- | ------------------------------------------------- |
| Menulis handler baru               | `snake_case`                                      |
| Menambah request/response struct   | `PascalCase` untuk type, `snake_case` untuk field |
| Menambah konstanta global          | `SCREAMING_SNAKE_CASE`                            |
| Menamai file route/service         | `snake_case`                                      |
| Menyesuaikan format JSON eksternal | serde `rename` / `rename_all`                     |

## 9. Contoh Buruk vs Bagus

Buruk:

```rust
pub struct createuserrequest {
    pub UserName: String,
}

fn CreateUser() {}
```

Bagus:

```rust
pub struct CreateUserRequest {
    pub username: String,
}

fn create_user() {}
```

## 10. Rule of Thumb untuk Project Ini

1. Semua function/handler/module/file pakai `snake_case`.
2. Semua type (`struct`, `enum`, `trait`) pakai `PascalCase`.
3. Semua constant pakai `SCREAMING_SNAKE_CASE`.
4. Jangan pakai `camelCase` untuk identifier Rust internal.
5. Kalau API minta format lain, ubah via serde, bukan ubah style Rust internal.
