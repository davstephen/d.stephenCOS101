use std::io::Write;

fn main() {
    let commissioner = ["Aigbogun Alamba Daudu","Murtala Afeez Bendu","Okorocha Calistus Ogbona","Adewale Jimoh Akanbi","Osazuwa Faith Etiye"];
    let ministry = ["Internal Affairs","Justice","Defense","Power & Steel","Petroleum"];
    let zone = ["South West","North East","South South","South West","South East"];

    let mut file = std::fs::File::create("EFCC-Merge.txt").expect("Create failed");

     file.write_all("Comissioners:\n".as_bytes());
    for i in 0..commissioner.len() {
    file.write_all(commissioner[i].as_bytes()).expect("Write failed");
    file.write_all("\n".as_bytes()).expect("Write failed");
}

    file.write_all("Ministries:\n".as_bytes());
    for i in 0..ministry.len() {
    file.write_all(ministry[i].as_bytes()).expect("Write failed");
    file.write_all("\n".as_bytes()).expect("Write failed");
}
    file.write_all("Zones:\n".as_bytes());
    for i in 0..zone.len() {
    file.write_all(zone[i].as_bytes()).expect("Write failed");
    file.write_all("\n".as_bytes()).expect("Write failed");
}
    println!("Data merged into file.");
}