use std::fs;


fn main() {

    let mut input: Vec<i64>  = fs::read_to_string("input.txt").unwrap().lines().map(|x| x.parse::<i64>().unwrap()).collect();

    //println!("Total increases: {}", count_increases(input));
    println!("Total increases: {}", count_triplet_sum_increases(input));


}

#[allow(dead_code)]
fn count_triplet_sum_increases(list: Vec<i64>) -> i64 {
    let mut total = 0;
    let mut prev = list[0] + list[1] + list[2];
    
    for i in 3..list.len()-1 {
        if i+1 >= list.len() || i+2 >= list.len(){
            return total;
        }
        let sum = list[i] + list[i+1] + list[i+2];
        if sum > prev{ 
            total+=1;
        }
        else {
            prev = sum;
        }

    }

    total
}

#[allow(dead_code)]
fn count_increases(list: Vec<i64>) -> i64{
    
    let mut total = 0;
    let mut prev = list[0];

    for value in list{
        if value > prev {
            total+=1;
            prev = value;
        }
        else {
            prev = value;
        }
    }

    total
}