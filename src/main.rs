use clap::Parser;
use std::collections::HashMap;

/// Simple CLI Pokédex
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Type to list information for
    #[arg(short, long)]
    _type: Option<String>,

    /// List all types and their information
    #[arg(short, long)]
    list: bool,

    /// Search a pokemon by name (not yet implemented)
    #[arg(short, long)]
    search: Option<String>,
}

#[derive(Debug)]
struct TypeInfo {
    offense: Vec<String>,
    defense: Vec<String>,
}

fn main() {
    let mut types_info: HashMap<String, TypeInfo> = HashMap::new();

    types_info.insert(
        "fire".to_string(),
        TypeInfo {
            offense: vec!["grass".to_string(), "ice".to_string()],
            defense: vec!["water".to_string(), "rock".to_string()],
        },
    );
    types_info.insert(
        "water".to_string(),
        TypeInfo {
            offense: vec!["fire".to_string(), "ground".to_string()],
            defense: vec!["electric".to_string(), "grass".to_string()],
        },
    );

    let args = Args::parse();

    if args.list {
        for (type_name, info) in &types_info {
            println!("{}:", type_name);
            println!("  Offensive: {:?}", info.offense);
            println!("  Defensive: {:?}", info.defense);
            println!();
        }
    }
    
    if let Some(type_name) = args._type {
        match types_info.get(&type_name) {
            Some(info) => {
                println!("{}:", type_name);
                println!("  Offensive: {:?}", info.offense);
                println!("  Defensive: {:?}", info.defense);
            }
            None => println!("No information found for type: {}", type_name),
        }
    }
}

