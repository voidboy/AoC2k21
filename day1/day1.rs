use std::fs;

fn main() {
    let contents = fs::read_to_string("input")
        .expect("Cannot read the file");


   let mut read: usize = 0;
   let mut sums: Vec<usize> = vec![];
   loop {
       let mut current = 0;
       for line in contents.lines().skip(read).take(3) {
           current += line.parse::<usize>()
               .expect("Not a number");
       }
       sums.push(current);
       read += 1;
       if read >= contents.len() { break ; }
   }
   let mut last: usize = usize::MAX;
   let mut increased: usize = 0;
   for n in sums {
       if n > last { increased += 1; }
       last = n;
   }
   println!("increased is {}", increased);
}
