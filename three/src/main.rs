// 1) References must always point to valid (live) data
// 2) At any given time you may only either:
//  - exactly one mutable reference
//  - as many immutable references as you like

fn ops_dangle() -> &Vec<i32> {
    let dead_vec = vec![5, 4, 3, 2, 1];
    &dead_vec
}

fn sum(vec: &Vec<i32>) -> i32 {
    vec.iter().fold(0, |sum, val| sum + val)
}

fn add_one(vec: &mut Vec<i32>) {
    vec.push(1);
}

fn main() {
    let mut vec = vec![1, 2, 3];

    let _ref1 = &vec;
    let ref2 = &vec;
    let ref3 = &vec;
    let my_sum = sum(ref2);
    println!("my sum {}", my_sum);

    let ref4 = &mut vec;
    add_one(ref4);

    let my_sum = sum(ref3); // opps alive
    println!("my sum again {}", my_sum);
}


/*
With these rules, Rust ensures we are never holding a pointer that we can't safely deference
and that we are free from data races (but this has more than just multi threaded implications).
*/
