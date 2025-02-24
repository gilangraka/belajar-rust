fn rerata(list_nilai: &Vec<i32>) -> f32 {
    let mut count = 0;
    let len = list_nilai.len();

    for i in list_nilai {
        count += i;
    }

    let res = count as f32 / len as f32;
    return res;
}

fn pola_bintang(list_angka: Vec<i32>) {
    for i in list_angka {
        for _ in 0..i {
            print!("*");
        }
        println!("");
    }
}

pub fn main_jobsheet_4(list_nilai: Vec<i32>, list_angka: Vec<i32>) {
    let res_rerata = rerata(&list_nilai);

    println!("Rerata nilai dari list nilai {:?} adalah {}", list_nilai, res_rerata);
    println!("");
    println!("Pola bintang dari list angka {:?} adalah", list_angka);
    pola_bintang(list_angka);
}
