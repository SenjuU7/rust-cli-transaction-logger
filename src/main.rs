use std::fs::OpenOptions;
use std::io::{self, Write};
use std::time::{SystemTime, UNIX_EPOCH};

fn main() {
    println!("Menginput transaksi penjualan...");
    println!("----------------------------------");

    let mut nama = String::new();
    let mut harga = String::new();
    let mut qty = String::new();
    let mut kasir = String::new();
    let mut catatan = String::new();

    print!("Nama : ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut nama).unwrap();

    print!("Harga : ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut harga).unwrap();

    print!("Qty : ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut qty).unwrap();

    print!("Kasir : ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut kasir).unwrap();

    print!("Catatan : ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut catatan).unwrap();

let harga: u32 = harga.trim().parse().unwrap();
let qty: u32 = qty.trim().parse().unwrap();
let total = harga * qty;

    let trx = Transaksi{
        nama: nama.trim().to_string(),
        harga: harga as u32,
        qty: qty as u32,
        total: total as u32,
        waktu: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs().to_string(),
        kasir: kasir.trim().to_string(),
        catatan: catatan.trim().to_string(),
    };

    let filepath = "transaksi.csv";
    write_header(filepath);
    append_to_file(filepath, &trx.to_csv_row());

}

fn append_to_file(file_path: &str, content: &str) {
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(file_path)
        .expect("Gagal membuka file");

    writeln!(file, "{}", content).expect("Gagal menulis file");
}

fn write_header(filepath: &str){
    let ndheader = match std::fs::metadata(filepath){
        Ok(meta) => meta.len() == 0,
        Err(_) => true,
    };

    if ndheader{
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(filepath)
            .unwrap();

        writeln!(file, "nama,harga,qty,total,waktu,kasir,catatan").unwrap();
    }
}

struct Transaksi {
    nama: String,
    harga: u32,
    qty: u32,
    total: u32,
    waktu: String,
    kasir: String,
    catatan: String,
}

impl Transaksi {
    fn to_csv_row(&self) -> String {
        format!(
            "{},{},{},{},{},{},{}",
            self.nama,
            self.harga,
            self.qty,
            self.total,
            self.waktu,
            self.kasir,
            self.catatan
        )
    }
}

// fn input(prompt: &str) -> String {
//     let mut buf = String::new();
//     print!("{}", prompt);
//     io::stdout().flush().unwrap();
//     io::stdin().read_line(&mut buf).unwrap();
//     buf.trim().to_string()
// }
