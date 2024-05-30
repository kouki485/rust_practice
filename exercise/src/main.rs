fn collatz_length(mut n: i32) -> u32 {
	let mut i = 1;

	loop{
		if n == 1 {
			break;
		} else if n % 2 == 0 {
			n /= 2;
			i += 1;
		} else {
			n = n * 3 + 1;
			i += 1;
		}
	}
	return i;
}

fn main(){
	println!("length:{}",collatz_length(2));
}

