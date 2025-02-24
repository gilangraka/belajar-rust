#[derive(Debug)]
pub struct Mahasiswa {
    nama: String,
    nilai: f32,
    active: bool,
    sign_in_count: i32,
}

impl Mahasiswa {
    pub fn new(nama: &str, nilai: f32) -> Self {
        Self {
            nama: String::from(nama),
            nilai,
            active: true,
            sign_in_count: 0,
        }
    }

    pub fn login(&mut self) {
        self.sign_in_count += 1;
        println!("User {} login sebanyak: {}", self.nama, self.sign_in_count);
    }

    pub fn detail(&mut self) {
        println!("DETAIL MAHASISWA :");
        println!("Nama : {}", self.nama);
        println!("Nilai : {}", self.nilai);
        println!("Status Aktif : {}", self.active);
        println!("Jumlah Login : {}", self.sign_in_count);
    }
}