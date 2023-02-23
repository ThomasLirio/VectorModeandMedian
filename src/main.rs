use std::io;
use std::collections::HashMap;



fn main() {
    println!("Calculate the median and the mode of your list of integers separeted by semicommas");

    let mut v: String = String::new();
    io::stdin()
        .read_line(&mut v)
        .expect("failed to read line");
    if v.trim().len() == 0 {
       println!("please insert a list of integers separeted by semicommas");
    } else  {
        let mut iv: Vec<i32> = Vec::new();

        let mut vmode:HashMap<i32, usize> = HashMap::new();

        for i in v.split(";"){

            let input: i32 = i
                .trim()
                .parse()
                .expect("Wanted a number");

            iv.push(input);

            let count = vmode.entry(input).or_insert(0);
            *count += 1;
        }
        iv.sort();

        println!("your list of integers {:?} has: ", &iv);
        println!("{} as mode", vmode.iter()
                                .max_by(|a, b| a.1.cmp(&b.1))
                                .map(|(k, _v)| k).unwrap());
        if iv.len()%2 == 0{
            println!("{} as median", ((iv[iv.len()/2 -1] as f32 + iv[iv.len()/2] as f32)/2 as f32));
        } else {
            println!("{} as median", iv[(iv.len()-1)/2 ] );
        }
    }
}
