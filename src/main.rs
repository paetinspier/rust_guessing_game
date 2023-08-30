use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);
    let mut count = 0;

    loop {
        println!("Input guess: ");
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        println!("You guessed: {guess}");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        count += 1;
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }

    let algo_count = bs(secret_number);

    match count.cmp(&algo_count) {
        Ordering::Equal => {
            println!("You tied the algorithm with {count} tries!");
        },
        Ordering::Greater => {
            println!("You lost to the algorithm {algo_count} tries to {count} tries :(");
        },
        Ordering::Less => {
            println!("You beat the algorithm {count} tries to {algo_count} tries :)");
        }
    }
}


fn bs(secret_number: u32) -> u32 {
    let mut arr: [u32; 100] = [0; 100];

    for (index, element) in arr.iter_mut().enumerate() {
        *element = (index as u32) + 1;
    }

    let mut low: usize = 0;
    let mut high: usize = arr.len() - 1;
    let mut count: u32 = 0;

    while low <= high {
        let mid: usize = low + (high - low) / 2;
        count += 1;

        match arr[mid].cmp(&secret_number) {
            std::cmp::Ordering::Less => {
                low = mid + 1;
            }
            std::cmp::Ordering::Greater => {
                high = mid - 1;
            }
            std::cmp::Ordering::Equal => {
                return count;
            }
        }
    }
    count
}
