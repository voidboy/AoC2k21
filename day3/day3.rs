use std::fs;

fn main() {
    let contents = fs::read_to_string("input")
        .expect("Cannot read the file");

   let mut ones: usize;
   let bits = contents.lines().collect::<Vec<&str>>();
   let mut gamma_rate = String::new();
   let mut epsilon_rate = String::new();
   for bit in 0..bits[0].len() {
       ones = 0;
       for value in &bits {
           if value.as_bytes()[bit] == b'1' { ones += 1; }
       }
       if ones >= bits.len() / 2
       {
           gamma_rate.push('1');
           epsilon_rate.push('0');
       } else {
           gamma_rate.push('0');
           epsilon_rate.push('1');
       }
   }
   println!("diagnostic: {}",
       usize::from_str_radix(&gamma_rate, 2).unwrap() *
       usize::from_str_radix(&epsilon_rate, 2).unwrap());
}
