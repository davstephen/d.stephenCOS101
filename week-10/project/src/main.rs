struct Laptop { 
brand: String, 
price: u32, 
}

impl Laptop { 
fn cost(&self, quantity: u32) -> u32 { 
return self.price * quantity; 
} 
}

fn main() { 
let laptops = vec![ 
Laptop { brand: String::from("HP"), price: 650_000 }, 
Laptop { brand: String::from("IBM"), price: 755_000 }, 
Laptop { brand: String::from("Toshiba"), price: 550_000 }, 
Laptop { brand: String::from("Dell"), price: 850_000 }, 
];

let quantity: u32 = 3;
let mut total: u32 = 0;

for i in 0..laptops.len() {
    let cost = laptops[i].cost(quantity);
    println!("{} laptops: {} x N{} = N{}", laptops[i].brand, quantity, laptops[i].price, cost);
    total += cost;
}

println!("Total cost of {} laptops from each brand: N{}", quantity, total);
}