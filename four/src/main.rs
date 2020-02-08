#[derive(Debug)]
struct VecAccumulator {
    items: Vec<i32>,
}

// Syntax for adding "methods"/functions to a type
impl VecAccumulator {
    // Speical "Self" type
    fn from(other: &Vec<i32>) -> Self {
        VecAccumulator {
            items: other.clone()
        }
    }

    fn new() -> Self {
        VecAccumulator {
            items: Vec::new(),
        }
    }

    // Special self param
    fn sum(&self) -> i32 {
        self.items.iter().fold(0, |sum, item| sum + item)
    }

    fn clear(&mut self) {
        self.items.clear();
    }

    fn print(&self) {
        let sum = self.sum();
        println!("The sum is {}.", sum);
    }

    fn invalidate_me(self) -> i32 {
        self.sum()
    }
}

fn main() {
    let mut accum = VecAccumulator::from(&vec![1, 2, 3]);
    accum.print();

    accum.clear();
    accum.print();

    let _final_sum = accum.invalidate_me();
    accum.print(); // opps

    let another = VecAccumulator::new();
    another.print();
}
