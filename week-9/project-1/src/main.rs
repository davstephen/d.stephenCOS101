use std::io::Write;

fn main() {

    let lager = "33 Export, Desperados, Goldberg, Guider, Heineken,Star\n";
    let stout = "Legend, Turbo King, Williams\n";
    let non_alcoholic = "Maltina, Amstel Malta, Fayrouz\n";


    let mut file = std::fs::File::create("brewries.txt").expect("create failed");
    file.write_all("Our categories are shown below:\n"
        .as_bytes()).expect("Write failed");
    file.write_all("These are our Lagers:\n".as_bytes());
    file.write_all(lager.as_bytes()).expect("Write failed");
    file.write_all("These are out Stouts:\n".as_bytes());
    file.write_all(stout.as_bytes()).expect("Write failed");
    file.write_all("These are our Non-alcoholic drinks\n".as_bytes());
    file.write_all(non_alcoholic.as_bytes()).expect("Write failed");
    println!("\nData written to file.");

}