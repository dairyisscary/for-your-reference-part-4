fn reference_sum(borrowed_vec: &Vec<i32>) -> i32 {
    borrowed_vec.iter().fold(0, |sum, val| sum + val)
}

fn put_a_one_into_it_opps(borrowed_vec: &Vec<i32>) {
    borrowed_vec.push(1);
}

fn put_a_one_into_it(borrowed_vec: &mut Vec<i32>) {
    borrowed_vec.push(1);
}

fn main() {
    let mut vec = vec![1, 2, 3];

    {
        let ref_to_vec: &Vec<i32> = &vec;
        let sum = reference_sum(ref_to_vec);
        println!("The sum is {}.", sum);
        println!("We still have ownership here {:?}", vec);
    }

    {
        put_a_one_into_it(&mut vec);
        put_a_one_into_it(&mut vec);
        let sum_again = reference_sum(&vec);
        println!("The sum is {}.", sum_again);
        println!("We still have ownership here {:?}", vec);
    }
}
