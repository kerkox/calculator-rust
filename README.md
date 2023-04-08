# Basic Calculator made with Rust

operations implemented:

## + Adition
## - subtract
## / division
## * multiplication

## Execute the program 
clone the repo and then execute the next command:

`cargo run`


### Features: 
for the development of the calculator we made use of regular expressions to extract the operations to be performed in pairs.

However, to make the code more dynamic, we made use of regular expression generation using a vector where the operations to be implemented are stored. 

```rust
  fn main() {
    println!("Please enter your operation: ");
    let mut expresion: String = String::new();
    std::io::stdin().read_line(&mut expresion).unwrap();

    // Regex
    let operations = vec!["/", "\\*", "\\+", "-"];
    let mut result: String = expresion;
    for operator in operations {
      let regex_str = format!("(\\d+)\\s*{operator}\\s*(\\d+)");
      let re = Regex::new(&regex_str).unwrap();
      result = math_operation(re, result, operator);
    }
    println!("Result: {result}");
  }
```