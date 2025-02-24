use crate::models::mahasiswa::Mahasiswa;

pub struct StudyGroup {
    nama: String,
    active: bool,
    anggota: Vec<Mahasiswa>
}

impl StudyGroup {
    pub fn new(nama: &str) -> StudyGroup {
        StudyGroup {
            nama: String::from(nama),
            active: true,
            anggota: Vec::new()
        }
    }

    pub fn detail(&self){
        println!("DETAIL STUDY GROUP :");
        println!("Nama : {}", self.nama);
        println!("Status Aktif : {}", self.active);
        println!("Anggota : ");
        print!("{:?}", self.anggota);
    }

    pub fn add_anggota(&mut self, anggota: Mahasiswa){
        self.anggota.push(anggota);
    }
}