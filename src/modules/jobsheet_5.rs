use crate::models::mahasiswa::Mahasiswa;
use crate::models::study_group::StudyGroup;

pub fn main_jobsheet5() {
    let mut mhs1 = Mahasiswa::new("Gilang", 85.5);
    let mut mhs2 = Mahasiswa::new("Galang", 80.9);
    let mut mhs3 = Mahasiswa::new("Elang", 80.5);

    let mut study1 = StudyGroup::new("Tugas Desain Grafis");
    study1.detail();

    study1.add_anggota(mhs1);
    study1.add_anggota(mhs2);
    study1.add_anggota(mhs3);

    study1.detail();
}