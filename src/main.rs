use std::fs;
use nixel::parse;
use serde::Serialize;

fn main() {
    let input = fs::read_to_string("/home/oz/.nix/flake.nix").unwrap();
    let parsed = parse(input);
    
    // Create a JSON serializer
    let mut serializer = serde_json::Serializer::new(std::io::stdout());
    
    // Serialize the parsed data
    println!("{:#?}", parsed.serialize(&mut serializer).unwrap());
}
