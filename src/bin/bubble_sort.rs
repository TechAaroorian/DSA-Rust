use std::io;

fn main() {
    let mut user_input = String::new();

    println!("Please enter number to sort:");

    io::stdin().read_line(&mut user_input)
        .expect("Failed to read input");

    let mut num_list: Vec<i32> = user_input.split_whitespace()
        .filter_map( |x| x.parse().ok() )
        .collect();

    let num_list_len = num_list.len();
    let mut got_swap;
    let mut i;
    let mut phase_count = 1;

    println!();
    loop {
        println!("Phase {}:", phase_count);
        phase_count += 1;
        i = 0;
        got_swap = false;
        while i < num_list_len - 1 {
            if num_list[i] > num_list[i + 1] {
                let temp = num_list[i];
                num_list[i] = num_list[i + 1];
                num_list[i + 1] = temp;
                got_swap = true;
            }
            i += 1;
            println!("{:?}", num_list);
        }
        println!();
        if !got_swap {
            break;
        }
    }

    print!("\n\nResult: ");
    println!("{:?}", num_list);
}