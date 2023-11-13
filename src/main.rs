enum Fruit {
    Apple { color: String, taste: String },
    Pear { color: String, taste: String },
    Pineapple { color: String, taste: String },
}

impl Fruit {
    fn features(fruits_vec: &Vec<Fruit>){
        for fruit in fruits_vec {
            match fruit {
                Fruit::Apple { color, taste } => {
                    println!("This apple is {} and {}", color, taste);
                }
                Fruit::Pear { color, taste } => {
                    println!("This pear is {} and {}", color, taste);
                }
                Fruit::Pineapple { color, taste } => {
                    println!("This pineapple is {} and {}", color, taste);
                }
            }
        }
    }
}

fn main() {
    let fruits_vec: Vec<Fruit> = Vec::from(
        [
            Fruit::Apple {
                color: String::from("Green"),
                taste: String::from("Sour"),
            },
            Fruit::Pear {
                color: String::from("Yellow"),
                taste: String::from("Sweet"),
            },
            Fruit::Pineapple {
                color: String::from("Yellow"),
                taste: String::from("Juicy"),
            }
        ]
    );

    Fruit::features(&fruits_vec)
}