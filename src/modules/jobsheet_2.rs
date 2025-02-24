fn get_genap(nilai_awal: i32, nilai_akhir: i32) -> Vec<i32> {
    let mut genap: Vec<i32> = Vec::new();

    for i in nilai_awal ..= nilai_akhir {
        if (i % 2 == 0) {
            genap.push(i);
        }
    }

    genap
}

fn faktorial(nilai_n: i32) -> i32 {
    if nilai_n < 0 {
        panic!("Faktorial tidak terdefinisi untuk bilangan negatif!");
    }

    let mut faktorial: i32 = 1;
    for i in  1 ..= nilai_n{
        faktorial *= i;
    }

    faktorial
}

pub fn main_jobsheet2(nilai_awal: i32, nilai_akhir: i32, nilai_n: i32) {
    let res_genap = get_genap(nilai_awal, nilai_akhir);
    let res_faktorial = faktorial(nilai_n);

    println!("Bilangan genap dari {} sampai {} adalah", nilai_awal, nilai_akhir);
    println!("{:?}", res_genap);

    println!("");

    println!("Hasil dari {}! adalah", nilai_n);
    println!("{}", res_faktorial);
}