# CLI Pokédex

A simple command-line interface (CLI) Pokédex built in Rust, allowing users to explore Pokémon types and their effectiveness in battles.

## Current Features

- **Type Information Lookup**:  
  Users can get detailed information about Pokémon types, including their offensive and defensive properties.  
  - Offensive properties include which types a Pokémon type is effective against, not effective against, and has no effect on.  
  - Defensive properties include which types a Pokémon type is weak to, resists, or is immune to.

- **Listing All Types**:  
  Users can list all available types along with their effectiveness.

## Example Usage

### List all types and their information:
```bash
cargo run -- --list
```
Example output:
```
fire:
  Offensive: ["grass", "ice"]
  Defensive: ["water", "rock"]

water:
  Offensive: ["fire", "ground"]
  Defensive: ["electric", "grass"]
```

### Get information for a specific type (e.g., fire):
```bash
cargo run -- --type fire
```
Example output:
```
fire:
  Offensive: ["grass", "ice"]
  Defensive: ["water", "rock"]
```

### If the type is not found:
```bash
cargo run -- --type fighting
```
Example output:
```
No information found for type: fighting
```

## Future Plans

- **Pokémon Search**: Implement the ability to search for Pokémon by name and retrieve basic information about them, such as types, stats, and abilities.
  
- **Type Effectiveness Lookup**: Add a more detailed type effectiveness calculator that allows users to check how one type fares against another.

- **JSON File Support**: Instead of hardcoding type information in the Rust code, load it dynamically from a JSON file to make it easier to update the data.

- **Error Handling and Validation**: Improve error handling and input validation for better user experience, especially when invalid types or commands are provided.

- **Interactive CLI**: Implement an interactive CLI with prompts and auto-completion for better usability.

## Installation

To run this project, you need to have [Rust](https://www.rust-lang.org/) installed on your machine.

1. Clone the repository:
   ```bash
   git clone https://github.com/yourusername/cli-pokedex.git
   ```

2. Navigate to the project directory:
   ```bash
   cd cli-pokedex
   ```

3. Build and run the project:
   ```bash
   cargo run -- --list
   ```
