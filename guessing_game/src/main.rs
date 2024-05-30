use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
	println!("数を当ててね");
	let secret_num = rand::thread_rng().gen_range(1..=10);


	loop{
		println!("1~10の数字を入力して");
		
		let mut guess = String::new();
		
		io::stdin()
		.read_line(&mut guess)
		.expect("Fail to read line.");
	
	let guess: u32 = match guess.trim().parse() {
		Ok(num) => num,
		Err(_) => continue,
	};
	match guess.cmp(&secret_num) {
		Ordering::Less => println!("小さいよ"),
			Ordering::Greater => println!("大きいよ"),
			Ordering::Equal => {
				println!("正解");
				break;
			}
		}
	}
}
