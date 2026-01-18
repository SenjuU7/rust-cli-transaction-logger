# Rust CSV Transaction Logger (CLI)

A simple Rust CLI application for recording sales transactions into a CSV file.  
This project focuses on learning **file I/O**, **user input handling**, and **basic data modeling** using Rust.

---

## ✨ Features

- CLI-based user input
- Store transaction data in CSV format
- Auto-generate CSV header if file is empty
- Append data safely (no overwrite)
- Timestamp generation
- Simple struct-based data modeling

---

## 📦 Data Structure

Each transaction is stored with the following fields:

- `nama` – Item name
- `harga` – Item price
- `qty` – Quantity
- `total` – Total price
- `waktu` – Timestamp (UNIX time)
- `kasir` – Cashier name
- `catatan` – Additional notes

---

## 🛠️ Technologies Used

- **Rust**
- Standard Library (`std::io`, `std::fs`, `std::time`)
- CSV file handling (manual, no external crate)

---

## 🚀 How to Run

1. Make sure Rust is installed  
   👉 https://www.rust-lang.org/tools/install

2. Clone the repository
   ```bash
   git clone https://github.com/SenjuU7/rust-cli-transaction-logger
