use std::fs;
use std::collections::BTreeMap;

fn main() {
    let raw = fs::read_to_string("input").expect("File not found?");
    let input = raw.lines();
    let mut bit_map = BTreeMap::new();

    for line in input {
        let bits: Vec<char> = line.chars().collect();
        for (i, bit) in bits.into_iter().enumerate()  {
            match bit {
                '0' => {
                    let value = bit_map.entry(i).or_insert(0);
                    *value -= 1;
                },
                '1' => {
                    let value = bit_map.entry(i).or_insert(0);
                    *value += 1;
                }
                _ => println!("Not a valid bit!"),
            }
        }
    }

    

    println!("{:?}", bit_map);


}
