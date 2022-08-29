// Calculate the Median
// Based on the input array get the median.
use std::io;

fn median(){
  println!("Calculate the Median.\nKey in the numbers seperated by space.");
  let mut buffer = String::new();
  let mut input: Vec<i64> = match io::stdin().read_line(&mut buffer){
    Ok(_) => {
      if buffer.trim().is_empty() {
        println!("No Median.");
        return;
      }
      else{
        buffer.trim().split_whitespace().map(|a| a.parse::<i64>().unwrap()).collect()
      }      
    },
    Err(_) => {
      println!("No Input");
      return;
    }
  };
  input.sort();
  if input.len() % 2==0{
    println!("Median: {}", (input[input.len()/2-11] as f64 +input[input.len()/2] as f64)/2.0)
  }
  else{
    println!("Median: {}",input[input.len()/2]);    
  }
    
}