mod modules {
    pub mod jobsheet_1;
    pub mod jobsheet_2;
    pub mod jobsheet_3;
    pub mod jobsheet_4;
    pub mod jobsheet_5;
}

mod models {
    pub mod mahasiswa;
    pub mod study_group;
}

fn main() {
    modules::jobsheet_5::main_jobsheet5();
}
