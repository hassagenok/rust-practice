const HEIGHT: usize = 11; // Має бути непарним числом
const WIDTH: usize = HEIGHT * 2 - 1;

fn main() {
    let mut result = String::new();

    for y in 0..HEIGHT {
        let stars = if y <= HEIGHT / 2 {
            1 + 2 * y
        } else {
            1 + 2 * (HEIGHT - 1 - y)
        };
        let spaces = (WIDTH - stars) / 2;

        for _ in 0..spaces {
            result.push(' ');
        }
        for _ in 0..stars {
            result.push('*');
        }
        result.push('\n');
    }

    print!("{}", result);
}
