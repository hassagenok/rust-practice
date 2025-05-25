fn draw_tree(triangle_count: u32) {
    let first_triangle = [
        [" ", "*", " "],
        [" ", "*", " "],
        ["*", "*", "*"],
    ];

    let mut line_index: u32 = 0;
    let mut triangle_index = 0;
    let mut triangle_height = 3;
    let mut triangle_start_line = 0;
    let mut current_triangle_width: u32 = 5;

    let tree_width = current_triangle_width + 2 * triangle_count;
    println!("width: {}", tree_width);

    loop {
        if line_index >= triangle_start_line + triangle_height {
            if triangle_index >= 1 {
                triangle_height += 1;
            }

            triangle_index += 1;
            triangle_start_line = line_index;
            current_triangle_width += 2;

            if triangle_index >= triangle_count {
                break;
            }
        }

        let stars_count: u32 = (line_index - triangle_start_line) * 2 + 1;

        let left_padding = ((tree_width - current_triangle_width) / 2) + ((current_triangle_width - stars_count) / 2);

        let mut triangle_column = 0;

        for i in 0..tree_width {
            let mut draw_star = false;

            if triangle_index == 0 {
                let center = tree_width / 2;
                let in_center_range = i >= center - 1 && i <= center + 1;
                if in_center_range && first_triangle[line_index as usize][triangle_column as usize] == "*" {
                    draw_star = true;
                }
                if in_center_range && triangle_column < 2 {
                    triangle_column += 1;
                }
            } else {
                if i >= left_padding && i < left_padding + stars_count {
                    draw_star = true;
                }
            }

            if draw_star {
                print!("*");
            } else {
                print!(" ");
            }
        }

        println!();
        line_index += 1;
    }
}

#[test]
fn test() {
    draw_tree(7);
}
