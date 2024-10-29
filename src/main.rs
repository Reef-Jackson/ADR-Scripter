use adr_create::ADR;
use std::io::stdin;
mod adr_create;

fn main() {
    let new_adr = ADR::new();
    println!("{:#?}", new_adr);
}
