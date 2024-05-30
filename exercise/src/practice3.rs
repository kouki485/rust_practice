struct Person {
	name: String,
	age: u8,
}


fn intro(person: &Person){
	println!("{} is {} years old.",person.name,person.age);
}

fn main(){
	let mut peter = Person{ name: String::from("Peter"),age: 27};
	intro(&peter);
	peter.age = 28;
	intro(&peter);

}