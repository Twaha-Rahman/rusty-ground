mod fcc_inter_algos;

use fcc_inter_algos::array_diff;

fn main() {
    let diff = array_diff(
        &["andesite", "grass", "dirt", "pink wool", "dead shrub"],
        &["diorite", "andesite", "grass", "dirt", "dead shrub"],
    );
    println!("The diff of the arrays are: {:?}", diff);
}
