trait Pet {
	fn talk(&self) -> String;

	fn greet(&self) {
		println!("hello.What's your name? {}",self.talk());
	}
}

struct Dog {
	name: String,
	age: i8,
}

impl Pet for Dog {
	fn talk(&self) -> String {
		format!("my name is {}",self.name)
	}
}

fn main() {
	let pochi = Dog {name: String::from("pochi") ,age:5};
	pochi.greet();
}