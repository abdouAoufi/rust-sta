use crate::auto::auto::Auto;
use crate::garden::vegetables::Asparagus;

pub mod auto;
pub mod garden;

fn main() {
    let plant = Asparagus {
        name: String::from("Aoufi Abderahmane"),
    };
    let mini_cooper = Auto {
        model : String::from("A8"),
        year : 2004
    };
    println!("I'm growing {:?}!", plant);
}
