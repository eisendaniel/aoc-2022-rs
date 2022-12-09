fn main() {
    let input = include_str!("example");
    input.lines().for_each(|line| {
        println!("{}", line);
    });
}
