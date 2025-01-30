fn main() {
    let val: Option<u64> = None;

    if let Some(number) = val {
        println!("{}", number)
    }

    let val = Some(10);

    if let Some(number) = val {
        println!("{}", number)
    }

    // didnt get to finish
    // let number = match val {
    //     Some(n) => n,
    //     None => 0
    // };

    // if let Some(number) = val {
    //     println!("{}", number)
    // }
}