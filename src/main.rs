struct Solution;

fn main() {
    let result = "Raye Lattice";
    println!("{:?}", result);
}

impl Solution {
    pub fn the_maximum_achievable_x(num: i32, t: i32) -> i32 {
        num + t * 2
    }
}
