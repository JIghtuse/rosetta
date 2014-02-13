use std::io::File;

fn main() {
    let input_file = Path::new("input.txt");

    let input = match File::open(&input_file).read_to_end() {
        Err(e) => fail!(e),
        Ok(input) => input
    };

    let output_file = Path::new("output.txt");
    match File::create(&output_file).write(input) {
        Ok(()) => { /* succeeded */ }
        Err(e) => println!("failed to write: {}", e),
    }
}
