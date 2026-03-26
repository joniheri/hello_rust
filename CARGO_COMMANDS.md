# Cargo Command Reference

| Tujuan | Perintah | Keterangan Singkat |
|---|---|---|
| Cek versi Cargo | `cargo --version` | Menampilkan versi Cargo yang terpasang. |
| Init project di folder saat ini | `cargo init` | Membuat project Rust baru di direktori aktif. |
| Buat project baru | `cargo new nama_project` | Membuat project Rust baru (binary). |
| Build debug | `cargo build` | Compile project mode debug. |
| Build release | `cargo build --release` | Compile optimasi untuk production. |
| Jalankan app | `cargo run` | Build (jika perlu) lalu menjalankan binary. |
| Auto rebuild + restart saat file berubah | `cargo watch -x run` | Dev mode mirip hot reload (perlu `cargo-watch`). |
| Jalankan release | `cargo run --release` | Menjalankan binary release. |
| Cek compile tanpa build final | `cargo check` | Validasi kode lebih cepat daripada build penuh. |
| Jalankan test | `cargo test` | Menjalankan seluruh unit/integration test. |
| Jalankan test tertentu | `cargo test nama_test` | Menjalankan test yang cocok dengan nama/filter. |
| Format kode | `cargo fmt` | Menjalankan formatter Rust (`rustfmt`). |
| Cek format (CI) | `cargo fmt -- --check` | Verifikasi format tanpa mengubah file. |
| Lint dengan clippy | `cargo clippy` | Menampilkan saran/peringatan kualitas kode. |
| Clippy ketat (CI) | `cargo clippy -- -D warnings` | Gagal jika ada warning lint. |
| Generate dokumentasi | `cargo doc --no-deps` | Membuat docs project tanpa dependency docs. |
| Buka dokumentasi lokal | `cargo doc --open` | Generate lalu buka docs di browser. |
| Bersihkan artifact build | `cargo clean` | Menghapus folder `target/`. |
| Tambah dependency | `cargo add axum` | Menambah crate ke `Cargo.toml`. |
| Tambah dependency dev | `cargo add --dev tokio-test` | Menambah crate ke `[dev-dependencies]`. |
| Hapus dependency | `cargo rm nama_crate` | Menghapus crate dari `Cargo.toml`. |
| Update dependency lockfile | `cargo update` | Update versi crate di `Cargo.lock`. |
| Lihat tree dependency | `cargo tree` | Menampilkan pohon dependency. |
| Jalankan binary spesifik | `cargo run --bin nama_bin` | Untuk workspace/project multi binary. |
| Jalankan example | `cargo run --example nama_example` | Menjalankan file example. |
| Test dengan output print | `cargo test -- --nocapture` | Menampilkan output `println!` saat test. |
| Benchmark (nightly/criterion) | `cargo bench` | Menjalankan benchmark jika tersedia. |
| Publish crate | `cargo publish` | Publish crate ke crates.io (jika siap). |

## Catatan Praktis

| Kebutuhan Harian | Perintah Rekomendasi |
|---|---|
| Loop dev cepat | `cargo check` |
| Auto restart saat coding | `cargo watch -x run` |
| Validasi sebelum commit | `cargo fmt -- --check && cargo clippy -- -D warnings && cargo test` |
| Build production | `cargo build --release` |

## Setup Cargo Watch

| Tujuan | Perintah |
|---|---|
| Install cargo-watch | `cargo install cargo-watch` |
| Jalankan auto restart server | `cargo watch -x run` |
