use std::io::Write;

fn main() {

    let name = ["Oluchi Mordi", "Adams Aliyu", "Shania Bolade", "Adekunle Gold", "Blanca Edemoh"];
    let matric = ["ACC10211111", "ECO10110101", "CSC10328828", "EEE11020202", "MEE10202001"];
    let dept = ["Accounting", "Economics", "Computer", "Electrical", "Mechanical"];
    let lvl = ["300", "100", "200", "200,", "100"];



    let mut file = std::fs::File::create("PAU-SIMStxt").expect("create failed");
    file.write_all("Student Names:\n".as_bytes());
    for i in 0..name.len() {
    file.write_all(name[i].as_bytes()).expect("Write failed");
    file.write_all("\n".as_bytes()).expect("Write failed");
}

    file.write_all("Martic. No's:\n".as_bytes());
    for i in 0..matric.len() {
    file.write_all(matric[i].as_bytes()).expect("Write failed");
    file.write_all("\n".as_bytes()).expect("Write failed");
}
    file.write_all("Departments:\n".as_bytes());
    for i in 0..dept.len() {
    file.write_all(dept[i].as_bytes()).expect("Write failed");
    file.write_all("\n".as_bytes()).expect("Write failed");
}
    file.write_all("Level:\n".as_bytes());
    for i in 0..lvl.len() {
    file.write_all(lvl[i].as_bytes()).expect("Write failed");
    file.write_all("\n".as_bytes()).expect("Write failed");
}
    println!("\nData written to file.");

}