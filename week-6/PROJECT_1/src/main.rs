// Rust program for a restuarant menu
use std::io;

fn main() {

   // Input
   let item mut = String::new();
   let quantity mut = String::new();

   // Menu
   println!("Welcome to Dave's Diner");
   println!("What would you like to have today");
   println!("Here are the options available:");
   println!("Poundo Yam/Edinkaiko Soup - N3,200 (P)");
   println!("Fried Rice & Chicken      - N3,000 (F)");
   println!("Amala & Ewedu Soup        - N2,500 (A)");
   println!("Eba & Egusi Soup          - N2,000 (E)");
   println!("White Rice & Stew         - N2,500 (W)");
   println!("To select an dish from the menu, input the character code");

   // Choices
   let item1 = "Poundo Yam/Edinkaiko Soup";
   let item2 = "Fried Rice & Chicken";
   let item3 = "Amala & Ewedu Soup";
   let item4 = "Eba & Egusi Soup";
   let item5 = "White Rice & Stew";

   
   println!("\nEnter the dish code of your choice");
   io::stdin().read_line(&mut item);
   let dish:char = item.trim().parse().expect("Not a valid dish code");

    // Confirmation
   if item = 'P' {
      println!("You have selected {}", item1);
   }
   if item = 'F' {
      println!("You have selected {}", item2);
   }
   if item = 'A' {
      println!("You have selected {}", item3);
   }
   if item = 'E' {
      println!("You have selected {}", item4);
   }
   if item = 'W' {
      println!("You have selected {}", item5);
   }

   // Order
   let order mut = {
      if item == 'P' {
         let order = "item1";
      }
      else if item == 'F' {
         let order = "item2";
      }
      else if item == 'A' {
         let order = "item3";
      }
      else if item == 'E' {
         let order = "item4";
      }
      else if item == 'W' {
         let order = "item5";
      }
   }

   // Price
   let price mut = {
      if item == 'P' {
         let price = 3200;
      }
      else if item == 'F' {
         let price = 3000;
      }
      else if item == 'A' {
         let price = 2500;
      }
      else if item == 'E' {
         let price = 2000;
      }
      else if item == 'W' {
         let price = 2500;
      }

   }
   
   // Quantity
   println!("\nPlease input your desired quantity");
   io::stdin().read_line(&mut quantity);
   let qty:u8 = quantity.trim().parse().expect("Not a valid quantity");
   
   // Total
   let total = quantity * price;

   // Bill
   println!("Your order of {} is ready, your total bill is {}. Thank you ",order,total );


}
