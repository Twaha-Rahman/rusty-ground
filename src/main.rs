mod fcc_inter_algos;

use fcc_inter_algos::pig_latin;

fn main() {
    let str_to_process = String::from("banana");

    let processed = pig_latin(str_to_process.as_str());
    println!("The processed string is: {}", processed);
}
