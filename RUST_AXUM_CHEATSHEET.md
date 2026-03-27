# Rust + Axum Cheatsheet

## Keyword Dasar Rust

| Keyword/Sintaks | Arti                                           | Contoh                                       |
| --------------- | ---------------------------------------------- | -------------------------------------------- |
| `use`           | Import module/type/function dari crate lain    | `use axum::{Router, Json};`                  |
| `pub`           | Membuat item bisa diakses module lain (public) | `pub struct User`                            |
| `struct`        | Membuat tipe data custom                       | `struct User { id: u64 }`                    |
| `enum`          | Tipe dengan beberapa varian                    | `enum AppError { BadRequest, NotFound }`     |
| `fn`            | Function biasa (sync)                          | `fn validate_email(email: &str) -> bool`     |
| `async fn`      | Function asynchronous                          | `async fn create_user(...) -> ...`           |
| `.await`        | Menunggu operasi async selesai                 | `axum::serve(listener, app).await`           |
| `impl`          | Menambahkan method ke `struct`/trait           | `impl AppState { fn new() -> Self { ... } }` |
| `Result<T, E>`  | Return sukses/gagal                            | `Result<User, AppError>`                     |
| `?`             | Propagate error otomatis                       | `validate_email(&payload.email)?;`           |
| `Option<T>`     | Nilai opsional (`Some` / `None`)               | `password: Option<String>`                   |
| `&str`          | Borrowed string slice (tanpa ownership)        | `fn validate(name: &str)`                    |
| `String`        | Owned string (bisa diubah/disimpan)            | `pub email: String`                          |

## Atribut Umum

| Atribut                      | Fungsi                             | Contoh                                     |
| ---------------------------- | ---------------------------------- | ------------------------------------------ |
| `#[derive(...)]`             | Auto implement trait tertentu      | `#[derive(Serialize, Deserialize, Clone)]` |
| `#[serde(skip_serializing)]` | Field tidak ikut ke JSON output    | untuk `password_hash`                      |
| `#[allow(dead_code)]`        | Matikan warning item belum dipakai | field internal sementara                   |

## Axum Dasar

| Komponen                           | Arti                         | Contoh                                           |
| ---------------------------------- | ---------------------------- | ------------------------------------------------ |
| `Router`                           | Tempat daftar endpoint       | `Router::new().route("/users", get(list_users))` |
| `.route(path, method(handler))`    | Daftarkan route              | `.route("/", get(root).post(create))`            |
| `.nest("/users", users::router())` | Gabung sub-router            | route modular per domain                         |
| `Json<T>`                          | Request/response body JSON   | `Json(payload): Json<CreateUserRequest>`         |
| `State<T>`                         | Ambil shared state           | `State(state): State<AppState>`                  |
| `Path<T>`                          | Ambil path param             | `Path(id): Path<u64>`                            |
| `IntoResponse`                     | Tipe bisa jadi HTTP response | custom error / tuple `(StatusCode, Json(...))`   |

## Pola Handler CRUD (yang kamu pakai)

| Method   | Route         | Handler       | Kegunaan          |
| -------- | ------------- | ------------- | ----------------- |
| `GET`    | `/users`      | `list_users`  | List semua user   |
| `GET`    | `/users/{id}` | `get_user`    | Detail user by id |
| `POST`   | `/users`      | `create_user` | Buat user baru    |
| `PUT`    | `/users/{id}` | `update_user` | Update data user  |
| `DELETE` | `/users/{id}` | `delete_user` | Hapus user        |

## Pola Response API di Project Ini

| Kondisi | Bentuk                                                   |
| ------- | -------------------------------------------------------- |
| Sukses  | `{"status":"success","data":...}`                        |
| Gagal   | `{"code":"BAD_REQUEST","status":"fail","message":"..."}` |

## Catatan Praktis

| Kasus                          | Saran                                                                |
| ------------------------------ | -------------------------------------------------------------------- |
| Field sensitif (password hash) | Simpan di model internal, jangan tampilkan di response               |
| Validasi input                 | Buat function kecil terpisah (`validate_email`, `validate_username`) |
| Konsistensi API                | Pakai helper response supaya format selalu sama                      |
| Update parsial                 | Gunakan `Option<T>` di request update                                |
