fn take_ownership_sum(owned_vec: Vec<i32>) -> i32 {
    owned_vec.iter().fold(0, |sum, val| sum + val)
}

fn take_and_give_ownership_sum(owned_vec: Vec<i32>) -> (i32, Vec<i32>) {
    let sum = owned_vec.iter().fold(0, |sum, val| sum + val);
    (sum, owned_vec)
}

fn main() {
    let one = vec![1, 2, 3];
    let sum_one = take_ownership_sum(one);
    println!("The sum is {}.", sum_one);
    // let sum_one_again = take_ownership_sum(one); // ooops

    let two = vec![1, 3];
    let (sum_two, two_again) = take_and_give_ownership_sum(two);
    println!("The sum is {}.", sum_two);
    println!("We got ownership back of {:?}", two_again);
}
