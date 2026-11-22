mod square;
mod render;

fn main() {
    let window_size = (7_u16, 4_u16);
    let mut square = square::Square::new((1, 0), (2, 1), (-1, 1));

    println!("{:?}", square);

    square.update_position(window_size);
    println!("{:?}", square);

    square.update_position(window_size);
    println!("{:?}", square);
}
