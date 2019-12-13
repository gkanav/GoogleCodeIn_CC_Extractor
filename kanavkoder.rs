use std::io;

fn main() {
	let mut input_text = String::new();
    io::stdin()
        .read_line(&mut input_text)
        .expect("failed to read from stdin");

    let trimmed = input_text.trim();
    let my_int: i32 = trimmed.parse().unwrap();
    if my_int%400==0 {
		println!("the given year is a leap year");
	}
	else if my_int%4==0 && my_int%100!=0 {
		println!("the given year is a leap year");
	} else{
		println!("the given year is not a leap year");
	}

}

