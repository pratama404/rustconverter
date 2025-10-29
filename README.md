# 🔹 UnitConv – Konverter Satuan CLI dengan Rust

**UnitConv** adalah aplikasi Command Line Interface (CLI) sederhana berbasis **Rust** yang digunakan untuk mengonversi berbagai satuan seperti suhu dan panjang.  
Aplikasi ini juga menyimpan **riwayat konversi** agar pengguna dapat melihat hasil sebelumnya.

---

## 🚀 Cara Menjalankan Proyek

### 1️⃣ Clone Repository
```bash
git clone https://github.com/pratama404/rustconverter.git
cd unitconv
```

### 2️⃣ Build Proyek
```bash
cargo build
```

### 3️⃣ Jalankan Aplikasi
Gunakan perintah `cargo run -- <subcommand>` untuk menjalankan aplikasi.

---

## ⚙️ Cara Penggunaan

### 🔸 Konversi Satuan
Untuk melakukan konversi, gunakan subcommand `convert`:

```bash
cargo run -- convert --from <satuan_awal> --to <satuan_tujuan> --value <nilai>
```

**Contoh:**
```bash
cargo run -- convert --from celsius --to fahrenheit --value 100
```
📤 Output:
```
100 celsius = 212 fahrenheit
```

Contoh lain:
```bash
cargo run -- convert --from cm --to km --value 16000
```
📤 Output:
```
16000 cm = 0.16 km
```

---

### 🔸 Melihat Daftar Satuan yang Didukung
```bash
cargo run -- list
```
📤 Output:
```
Satuan yang didukung:
1. [suhu] celsius
2. [suhu] fahrenheit
3. [suhu] kelvin
4. [panjang] cm
5. [panjang] inch
6. [panjang] km
7. [panjang] miles
```

---

### 🔸 Melihat Riwayat Konversi
```bash
cargo run -- history
```
📤 Output:
```
Riwayat Konversi:
1. 100 celsius = 212 fahrenheit
2. 16000 cm = 0.16 km
```

---

### 🔸 Menghapus Riwayat (Opsional)
Jika fitur ini ditambahkan, pengguna dapat menghapus riwayat konversi dengan perintah:
```bash
cargo run -- clear-history
```
📤 Output:
```
Riwayat berhasil dihapus.
```

---

## 📦 Struktur Proyek
```
unitconv/
│
├── src/
│   ├── main.rs          # Entry point aplikasi
│   ├── converter.rs     # Modul konversi
│   ├── history.rs       # Modul riwayat
│
├── Cargo.toml           # Dependency dan metadata proyek
└── README.md            # Dokumentasi
```

---

## 🧩 Teknologi yang Digunakan
- **Rust 1.80+**
- **Crate clap** – untuk parsing argumen CLI
- **Crate serde / serde_json** *(opsional)* – untuk menyimpan riwayat konversi
- **Crate chrono** – untuk mencatat waktu konversi (jika digunakan)

---

## 💡 Fitur yang Tersedia
✅ Konversi antar satuan suhu dan panjang  
✅ Menampilkan daftar satuan yang didukung  
✅ Menyimpan dan menampilkan riwayat konversi  
✅ Ekstensi modular untuk menambah jenis satuan baru  
✅ CLI interaktif dengan perintah sederhana

---

## 🌱 Rencana Pengembangan
- [ ] Menambah dukungan satuan berat (kg, gram, pon)
- [ ] Menyimpan riwayat ke file eksternal (JSON)
- [ ] Menambahkan fitur penghapusan riwayat otomatis
- [ ] Menambahkan dukungan konversi kecepatan dan waktu
- [ ] Membuat versi GUI berbasis Tauri atau egui

---

## 🧠 Contoh Alur Kerja
```
$ cargo run -- convert --from celsius --to fahrenheit --value 100
100 celsius = 212 fahrenheit

$ cargo run -- convert --from cm --to km --value 16000
16000 cm = 0.16 km

$ cargo run -- list
Satuan yang didukung:
1. [suhu] celsius
2. [suhu] fahrenheit
3. [suhu] kelvin
4. [panjang] cm
5. [panjang] inch
6. [panjang] km
7. [panjang] miles

$ cargo run -- history
Riwayat Konversi:
1. 100 celsius = 212 fahrenheit
2. 16000 cm = 0.16 km
```

---

## 👤 Author
**Ageng Putra Pratama**  
🌐 GitHub: [github.com/pratama404](https://github.com/pratama404)  
🗓️ Terakhir diperbarui: 29 October 2025

---

> “Rust membuat program cepat dan aman.” 🦀
Hellcome Rustaceans