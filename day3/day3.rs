use std::fs;

fn koh_lanta(mut equipe_jaune: Vec<&str>, most: bool) -> &str {
   let len = equipe_jaune[0].len();
   for i in 0..len {
       let slice_len = equipe_jaune.len();
       if slice_len == 1 { break ; }
       let mut ones = 0;
       for value in &equipe_jaune {
           if value.as_bytes()[i] == b'1' { ones += 1; }
       }
       equipe_jaune = equipe_jaune
          .into_iter()
          .filter(|value| {
              if ones >= slice_len / 2 {
                  if most {
                      value.as_bytes()[i] == b'1'
                  } else {
                      value.as_bytes()[i] == b'0'
                  }
              } else {
                  if most {
                      value.as_bytes()[i] == b'0'
                  } else {
                      value.as_bytes()[i] == b'1'
                  }
              }
           })
          .collect();
   }
   equipe_jaune[0]
}

fn main() {
    let contents = fs::read_to_string("input")
        .expect("Cannot read the file");

   let oxygen_generator_rating = contents
       .lines()
       .collect::<Vec<&str>>();
   let co2_scrubber_rating = contents
       .lines()
       .collect::<Vec<&str>>();
   println!("magic number is {}", 
       usize::from_str_radix(koh_lanta(oxygen_generator_rating, true), 2).unwrap() *
       usize::from_str_radix(koh_lanta(co2_scrubber_rating, false), 2).unwrap());
}
