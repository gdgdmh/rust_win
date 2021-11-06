use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number!.(数を当ててごらん)");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    println!("The scret number is(秘密の数字は次の通り): {}", secret_number);

    loop {
        println!("Please input your guess.(ほら、予想を入力してみてね)");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line.(行の読み込みに失敗しました)");

        let result = guess.trim().parse::<u32>();
        let guess = match result {
            Ok(num) => num,
            Err(_) => continue
        };

        println!("You guessed(次のように予想しました): {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!(小さすぎ)"),
            Ordering::Greater => println!("Too big!(大きすぎ)"),
            Ordering::Equal => {
                println!("You win!(やったね)");
                // ループから抜ける
                break;
            }
        }
    }
}
