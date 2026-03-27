# SQLx CLI Cheat Sheet (v0.7.4)

| Area              | Command                      | Kegunaan                             | Contoh                                   |
| ----------------- | ---------------------------- | ------------------------------------ | ---------------------------------------- |
| Global            | `sqlx --version`             | Cek versi SQLx CLI                   | `sqlx --version`                         |
| Global            | `sqlx --help`                | Lihat semua command                  | `sqlx --help`                            |
| Database          | `sqlx database create`       | Buat database dari `DATABASE_URL`    | `sqlx database create`                   |
| Database          | `sqlx database setup`        | Buat DB + jalankan migration pending | `sqlx database setup`                    |
| Database          | `sqlx database reset -y`     | Fresh: drop + create + migrate       | `sqlx database reset -y`                 |
| Database          | `sqlx database drop -y`      | Hapus database                       | `sqlx database drop -y`                  |
| Migrate migration | `sqlx migrate add -r <name>` | Buat file migration `up/down`        | `sqlx migrate add -r create_users_table` |
| Migrate seeder    | `sqlx migrate add -r <name>` | Buat file seeder `up/down`           | `sqlx migrate add -r seed_users`         |

| Area    | Command                | Kegunaan                             |
| ------- | ---------------------- | ------------------------------------ |
| Migrate | `sqlx migrate run`     | Jalankan migration pending           |
| Migrate | `sqlx migrate info`    | Lihat status migration               |
| Migrate | `sqlx migrate revert`  | Rollback migration terakhir          |
| Prepare | `sqlx prepare`         | Generate metadata query offline      |
| Prepare | `sqlx prepare --check` | Validasi metadata `.sqlx` up-to-date |

| Seeder Dengan SQLx Default | Penjelasan                                        |
| -------------------------- | ------------------------------------------------- |
| Command `seed` bawaan      | Tidak ada di SQLx CLI                             |
| Pola default               | Seeder ditulis sebagai file migration SQL biasa   |
| Lokasi default             | `./migrations`                                    |
| Jalankan seeder            | `sqlx migrate run` atau `sqlx database reset -y`  |
| Rollback seeder terakhir   | `sqlx migrate revert`                             |
| Urutan eksekusi            | Berdasarkan version/timestamp nama file migration |

| Struktur Saat Ini | Lokasi                                                  |
| ----------------- | ------------------------------------------------------- |
| Migration schema  | `./migrations/20260327061644_create_users_table.*.sql`  |
| Migration seeder  | `./migrations/20260327073245_seed_users_dummy_24.*.sql` |

| Cara Buat Migration Baru | Sintaks                                  |
| ------------------------ | ---------------------------------------- |
| Buat migration `up/down` | `sqlx migrate add -r create_users_table` |
| Jalankan migration       | `sqlx migrate run`                       |
| Fresh migration          | `sqlx database reset -y`                 |

| Contoh Isi File `.up.sql`   | SQL                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                 |
| --------------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Template create table users | `CREATE TABLE IF NOT EXISTS users ( id BIGINT UNSIGNED NOT NULL AUTO_INCREMENT, email VARCHAR(255) NOT NULL, username VARCHAR(100) NOT NULL, password VARCHAR(255) NULL, fullname VARCHAR(255) NULL, create_at TIMESTAMP NULL DEFAULT CURRENT_TIMESTAMP, updated_at TIMESTAMP NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP, deleted_At TIMESTAMP NULL DEFAULT NULL, PRIMARY KEY (id), UNIQUE KEY uq_users_email (email), UNIQUE KEY uq_users_username (username) ) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;` |

| Contoh Isi File `.down.sql` | SQL                           |
| --------------------------- | ----------------------------- |
| Template drop table users   | `DROP TABLE IF EXISTS users;` |

| Opsi Penting               | Fungsi                                        | Contoh                                                        |
| -------------------------- | --------------------------------------------- | ------------------------------------------------------------- |
| `--source <DIR>`           | Ganti folder migration (default `migrations`) | `sqlx migrate run --source migrations`                        |
| `-D, --database-url <URL>` | Override `DATABASE_URL` dari `.env`           | `sqlx migrate info -D mysql://root@127.0.0.1:3306/hello_rust` |
| `--dry-run`                | Simulasi run/revert migration                 | `sqlx migrate run --dry-run`                                  |
| `--target-version <N>`     | Jalankan/revert sampai versi tertentu         | `sqlx migrate run --target-version 20260327061644`            |
| `-s, --sequential`         | Buat migration dengan nomor urut              | `sqlx migrate add -r -s add_profiles_table`                   |
| `-t, --timestamp`          | Buat migration dengan timestamp               | `sqlx migrate add -r -t add_profiles_table`                   |

| Contoh `.env`                   | Nilai                                                  |
| ------------------------------- | ------------------------------------------------------ |
| `DATABASE_URL` (tanpa password) | `mysql://root@127.0.0.1:3306/hello_rust`               |
| `DATABASE_URL` (pakai password) | `mysql://root:your_password@127.0.0.1:3306/hello_rust` |

| Workflow Harian (Recommended)  | Command                                                 |
| ------------------------------ | ------------------------------------------------------- |
| Tambah migration schema        | `sqlx migrate add -r create_posts_table`                |
| Isi file SQL migration         | Edit file `*.up.sql` dan `*.down.sql` di `./migrations` |
| Jalankan migration             | `sqlx migrate run`                                      |
| Tambah seeder migration        | `sqlx migrate add -r seed_posts_dummy`                  |
| Apply schema + seeder dari nol | `sqlx database reset -y`                                |

| Troubleshooting                         | Penyebab Umum                                    | Solusi Cepat                                              |
| --------------------------------------- | ------------------------------------------------ | --------------------------------------------------------- |
| `os error 2 while resolving migrations` | Folder migration tidak ketemu                    | Pastikan folder `./migrations` ada dan berisi file `.sql` |
| `Access denied for user`                | URL DB salah / auth MySQL tidak cocok            | Cek `DATABASE_URL` di `.env`                              |
| `checksum mismatch`                     | File migration lama diubah setelah pernah di-run | Development: `sqlx database reset -y`                     |
| `unrecognized subcommand 'seed'`        | SQLx tidak punya command seed bawaan             | Pakai migration seeder + `sqlx migrate run`               |

| Good Practice  | Rekomendasi                                                 |
| -------------- | ----------------------------------------------------------- |
| Nama migration | Gunakan prefix aksi: `create_`, `alter_`, `seed_`, `drop_`  |
| Isi `down.sql` | Selalu tulis rollback yang aman                             |
| Urutan seeder  | Buat seeder setelah schema migration                        |
| Commit Git     | Pisahkan commit schema migration dan docs bila memungkinkan |
