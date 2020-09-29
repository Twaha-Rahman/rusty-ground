mod fcc_inter_algos;

use fcc_inter_algos::filter_out;

fn main() {
    let mut vec1 = vec![1, 2, 3, 5, 1, 2, 3];
    let mut vec2 = vec![2, 3];

    let diff = filter_out(&mut vec1, &mut vec2);
    println!("The diff of the arrays are: {:?}", diff);
}
