use crate::garden::vegetables::Asparagus;

pub mod garden;

fn main() {
    let plant = Asparagus {
        name: String::from("Aoufi Abderahmane"),
    };
    println!("I'm growing {:?}!", plant);
}
