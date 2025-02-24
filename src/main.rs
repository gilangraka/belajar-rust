mod modules {
    pub mod jobsheet_1;
    pub mod jobsheet_2;
    pub mod jobsheet_3;
    pub mod jobsheet_4;
}
fn main() {
    let list_nilai: Vec<i32> = vec![50, 60, 70, 80, 90, 100];
    let pola: Vec<i32> = vec![1, 2, 3, 4, 5];
    modules::jobsheet_4::main_jobsheet_4(list_nilai, pola);
}
