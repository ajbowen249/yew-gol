pub fn init_field(width: u32, height: u32) -> Vec<Vec<bool>> {
    vec![vec![false; width as usize]; height as usize]
}

pub fn iterate_field(field: &Vec<Vec<bool>>) -> Vec<Vec<bool>>{
    // Funkiness with integer types here is to make sampling the edges easier.

    let width = field[0].len() as i32;
    let height = field.len() as i32;

    let mut out_field = init_field(width as u32, height as u32);

    let safe_get = |x: i32, y: i32| {
        if x < 0 || y < 0 || x >= width || y >= height {
            0
        } else {
            if field[y as usize][x as usize] { 1 } else { 0 }
        }
    };

    for y in 0..height {
        for x in 0..width {
            let live_neighbors = safe_get(x - 1, y - 1)
                + safe_get(x, y - 1)
                + safe_get(x + 1, y - 1)

                + safe_get(x - 1, y)
                + safe_get(x + 1, y)

                + safe_get(x - 1, y + 1)
                + safe_get(x, y + 1)
                + safe_get(x + 1, y + 1);

            let cell_value = field[y as usize][x as usize];

            out_field[y as usize][x as usize] = if cell_value {
                live_neighbors == 2 || live_neighbors == 3
            } else {
                live_neighbors == 3
            };
        }
    }

    out_field
}
