const WIDTH: usize = 20;
const HEIGHT: usize = 7;

fn main() {
    let mut canvas = vec![vec![' '; WIDTH]; HEIGHT];

    for x in 0..WIDTH {
        canvas[0][x] = '*';
        canvas[HEIGHT - 1][x] = '*';
    }
    for y in 0..HEIGHT {
        canvas[y][0] = '*';
        canvas[y][WIDTH - 1] = '*';
    }

    let mid_y = HEIGHT / 2;

    for i in 1..mid_y {
        let x = i * (WIDTH / 2) / mid_y;
        canvas[i][x] = '*';
        canvas[i][WIDTH - 1 - x] = '*';
    }

    canvas[mid_y][WIDTH / 2 - 1] = '*';
    canvas[mid_y][WIDTH / 2] = '*';

    for i in (mid_y + 1)..(HEIGHT - 1) {
        let j = HEIGHT - 1 - i;
        let x = j * (WIDTH / 2) / mid_y;
        canvas[i][x] = '*';
        canvas[i][WIDTH - 1 - x] = '*';
    }

    let result: String = canvas
        .iter()
        .map(|row| {
            let mut r = row.iter().collect::<String>();
            r.push('\n');
            r
        })
        .collect();

    print!("{}", result);
}
