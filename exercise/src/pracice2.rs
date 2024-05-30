//Nested Arrays

fn trasnspose(matrix:[[i32; 3]; 3]) -> [[i32; 3];3]{
	let copy = matrix;
	let mut res:[[i32; 3];3] = [[0; 3];3];

	for i in 0..3{
		for j in 0..3{
			res[i][j] = copy[j][i];
		}
	}	
	return res;
}

fn print_map(matrix:[[i32; 3];3]){
	for i in 0..3{
		for j in 0..3{
			print!("{} ",matrix[i][j]);
		}
		println!("");
	}
}

fn main(){
	let matrix:[[i32; 3];3] = [
		[1,2,3],
		[4,5,6],
		[7,8,9],
	];

	println!("before");
	print_map(matrix);
	//println!("matrix: {:#?}", matrix);

	println!("after");
	print_map(trasnspose(matrix));
}