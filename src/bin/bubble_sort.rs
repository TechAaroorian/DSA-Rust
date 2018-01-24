/* Bubble Sort Algorithm in Rust
* 
* Bubble Sort
* ------ ----
*
* -------------------------------------------
*   let num_list_len = num_list.len();
*   let mut got_swap;
*   let mut i;
*  
*   loop {
*       i = 0;
*       got_swap = false;
*       while i < num_list_len - 1 {
*          if num_list[i] > num_list[i + 1] {
*               let temp = num_list[i];
*               num_list[i] = num_list[i + 1];
*               num_list[i + 1] = temp;
*               got_swap = true;
*          }
*           i += 1;
*       }
*       if !got_swap {
*           break;
*       }
*   }
* --------------------------------------------
*
*/

fn main() {

    let mut num_list = vec![2, 43, 3, 56, 7, 8, 9, 65, 10, 11, 12, 21];

    let num_list_len = num_list.len();
    let mut got_swap;
    let mut i;

    println!();

    // Loop until sorting is finish
    loop {
        println!();
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

        // Checking for further sort; if fails break the loop 
        if !got_swap {
            break;
        }
    }

    print!("\n\nResult: ");
    println!("{:?}", num_list);
}