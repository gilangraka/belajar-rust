
fn celcius_converter(celcius_val: f32) -> (f32, f32, f32, f32) {
    let fahrenheit: f32 = celcius_val * 9.0 / 5.0 + 32.0;
    let kelvin: f32 = celcius_val + 273.15;
    let rankine: f32 = celcius_val * 9.0 / 5.0 + 491.67;
    let reaumur: f32 = celcius_val * 4.0 / 5.0;

    (fahrenheit, kelvin, rankine, reaumur)
}

fn lingkaran(jari_jari: f32) -> (f32, f32) {
    const PI: f32 = 3.14;
    let keliling: f32 = 2.0 * PI * jari_jari;
    let luas: f32 = PI * jari_jari * jari_jari;

    (keliling, luas)
}

pub fn main_jobsheet(celcius_val: f32, jari_jari: f32) {
    let (fahrenheit, kelvin, rankine, reaumur) = celcius_converter(celcius_val);
    let (keliling, luas) = lingkaran(jari_jari);

    println!("Hasil konversi dari {} celcius adalah : ", celcius_val);
    println!("{} derajat fahrenheit", fahrenheit);
    println!("{} derajat kelvin", kelvin);
    println!("{} derajat rankine", rankine);
    println!("{} derajat reaumur", reaumur);

    println!("");

    println!("Hasil keliling dan luas dari lingkaran dengan jari - jari {}cm adalah", jari_jari);
    println!("Keliling Lingkaran : {}cm2", keliling);
    println!("Luas Lingkaran : {}cm2", luas);
}