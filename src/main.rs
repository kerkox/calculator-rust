use regex::Regex;

fn math_operation(reg: Regex, mut operation: String, operator: &str) -> String {
  loop {
    let caps = reg.captures(&operation);
    if caps.is_none() {
      break;
    }
    let caps = caps.unwrap();
    let cap_expresion = caps.get(0).unwrap().as_str();
    let left_value : i32 = caps[1].parse().unwrap();
    let right_value : i32 = caps[2].parse().unwrap();
    let result: i32 = match operator {
      "\\+" => left_value + right_value,
      "\\*" => left_value * right_value,
      "-" => left_value - right_value,
      "/" => left_value / right_value,
      _ => 0,        
    };
    operation = operation.replace(cap_expresion, &result.to_string());
    
  }
  return operation;
}

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

