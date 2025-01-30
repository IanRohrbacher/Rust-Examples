fn print_vec(vec: &Vec<u8>) {
    for num in vec {
        println!("{}", num);
    }
}

fn main() {
    let nums: Vec<u8> = Vec::new();
    print_vec(&nums);

    let size = nums.len();
    println!("The size is {}.", size);
}