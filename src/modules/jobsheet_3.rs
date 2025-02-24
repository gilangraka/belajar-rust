fn is_prima(nilai: i32) -> bool {
    if nilai < 2 {
        return false;
    }

    let mut count = 0;
    for i in 1 ..= nilai {
        if nilai % i == 0 {
            count += 1;
        }
    }

    if count == 2 {
        return true;
    }
    return false;
}

fn get_prima(nilai_awal: i32, nilai_akhir: i32) -> Vec<i32> {
    let mut prima: Vec<i32> = Vec::new();

    for i in nilai_awal ..= nilai_akhir {
        if is_prima(i) {
            prima.push(i);
        }
    }

    prima
}

fn sisi_miring_segitiga(a:f32, b:f32) -> f32 {
    let c = (a.powf(2.0) + b.powf(2.0)).sqrt();
    return  c;
}

pub fn main_jobsheet3(nilai_awal:i32, nilai_akhir:i32, alpha:f32, beta:f32) {
    let res_prima = get_prima(nilai_awal, nilai_akhir);
    let res_sisi_miring_segitiga = sisi_miring_segitiga(alpha, beta);

    println!("Bilangan prima dari {} sampai {} adalah", nilai_awal, nilai_akhir);
    println!("{:?}", res_prima);

    println!("");

    println!("Sisi miring segitiga dengan sisi A {} dan sisi B {} adalah {}", alpha, beta, res_sisi_miring_segitiga);
}