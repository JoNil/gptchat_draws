use glam::{ivec2, IVec2};

fn write_color(buffer: &mut [u8], offset: i32, color: u32) {
    let Some(pixel_ref) = buffer.get_mut((offset as usize)..(offset as usize + 4)) else {
        return;
    };

    pixel_ref[0] = (color >> 24) as u8;
    pixel_ref[1] = (color >> 16) as u8;
    pixel_ref[2] = (color >> 8) as u8;
    pixel_ref[3] = color as u8;
}

fn draw_line(buffer: &mut [u8], size: IVec2, start: IVec2, end: IVec2, color: u32) {
    let mut x = start.x;
    let mut y = start.y;

    let dx = (end.x - start.x).abs();
    let dy = (end.y - start.y).abs();

    let sx = if start.x < end.x { 1 } else { -1 };
    let sy = if start.y < end.y { 1 } else { -1 };

    let mut err = if dx > dy { dx } else { -dy } / 2;
    let mut err2;

    loop {
        if x >= 0 && x < size.x && y >= 0 && y < size.y {
            let offset = (y * size.x + x) * 4;
            write_color(buffer, offset, color);
        }

        if x == end.x && y == end.y {
            break;
        }

        err2 = err;

        if err2 > -dx {
            err -= dy;
            x += sx;
        }
        if err2 < dy {
            err += dx;
            y += sy;
        }
    }
}

fn draw_filled_circle(buffer: &mut [u8], size: IVec2, pos: IVec2, radius: i32, color: u32) {
    let mut x = radius;
    let mut y = 0;
    let mut err = 0;

    while x >= y {
        draw_line(
            buffer,
            size,
            IVec2::new(pos.x + x, pos.y + y),
            IVec2::new(pos.x - x, pos.y + y),
            color,
        );
        draw_line(
            buffer,
            size,
            IVec2::new(pos.x + y, pos.y + x),
            IVec2::new(pos.x - y, pos.y + x),
            color,
        );
        draw_line(
            buffer,
            size,
            IVec2::new(pos.x - x, pos.y - y),
            IVec2::new(pos.x + x, pos.y - y),
            color,
        );
        draw_line(
            buffer,
            size,
            IVec2::new(pos.x - y, pos.y - x),
            IVec2::new(pos.x + y, pos.y - x),
            color,
        );

        y += 1;
        err += 1 + 2 * y;
        if 2 * (err - x) + 1 > 0 {
            x -= 1;
            err += 1 - 2 * x;
        }
    }
}

fn draw_circle(buffer: &mut [u8], size: IVec2, pos: IVec2, radius: i32, color: u32) {
    let mut x = radius;
    let mut y = 0;
    let mut err = 0;

    while x >= y {
        let idx = 4 * (size.x * (pos.y + y) + pos.x + x);
        write_color(buffer, idx, color);

        let idx = 4 * (size.x * (pos.y + y) + pos.x - x);
        write_color(buffer, idx, color);

        let idx = 4 * (size.x * (pos.y - y) + pos.x + x);
        write_color(buffer, idx, color);

        let idx = 4 * (size.x * (pos.y - y) + pos.x - x);
        write_color(buffer, idx, color);

        let idx = 4 * (size.x * (pos.y + x) + pos.x + y);
        write_color(buffer, idx, color);

        let idx = 4 * (size.x * (pos.y + x) + pos.x - y);
        write_color(buffer, idx, color);

        let idx = 4 * (size.x * (pos.y - x) + pos.x + y);
        write_color(buffer, idx, color);

        let idx = 4 * (size.x * (pos.y - x) + pos.x - y);
        write_color(buffer, idx, color);

        y += 1;
        err += 1 + 2 * y;
        if 2 * (err - x) + 1 > 0 {
            x -= 1;
            err += 1 - 2 * x;
        }
    }
}

fn draw_filled_rectangle(
    buffer: &mut [u8],
    size: IVec2,
    pos: IVec2,
    rectangle_size: IVec2,
    color: u32,
) {
    for x in pos.x..pos.x + rectangle_size.x {
        for y in pos.y..pos.y + rectangle_size.y {
            let idx = 4 * (size.x * y + x);
            write_color(buffer, idx, color);
        }
    }
}

fn draw_filled_triangle(buffer: &mut [u8], size: IVec2, a: IVec2, b: IVec2, c: IVec2, color: u32) {
    // Calculate the bounding box of the triangle
    let min_x = a.x.min(b.x).min(c.x);
    let min_y = a.y.min(b.y).min(c.y);
    let max_x = a.x.max(b.x).max(c.x);
    let max_y = a.y.max(b.y).max(c.y);

    // Iterate over the bounding box and check if each point is inside the triangle
    for x in min_x..=max_x {
        for y in min_y..=max_y {
            let point = (x, y);
            if is_point_in_triangle(point, a, b, c) {
                let idx = 4 * (size.x * y + x);
                write_color(buffer, idx, color);
            }
        }
    }
}

// Check if a given point is inside a triangle
fn is_point_in_triangle(point: (i32, i32), a: IVec2, b: IVec2, c: IVec2) -> bool {
    let (x, y) = point;
    let a = (a.x as f64, a.y as f64);
    let b = (b.x as f64, b.y as f64);
    let c = (c.x as f64, c.y as f64);

    // Calculate the barycentric coordinates of the point with respect to the triangle
    let denominator = (b.1 - c.1) * (a.0 - c.0) + (c.0 - b.0) * (a.1 - c.1);
    let lambda_1 = ((b.1 - c.1) * (x as f64 - c.0) + (c.0 - b.0) * (y as f64 - c.1)) / denominator;
    let lambda_2 = ((c.1 - a.1) * (x as f64 - c.0) + (a.0 - c.0) * (y as f64 - c.1)) / denominator;

    // The point is inside the triangle if the barycentric coordinates are positive and sum to 1
    lambda_1 >= 0.0 && lambda_2 >= 0.0 && lambda_1 + lambda_2 <= 1.0
}

fn draw_cat(buffer: &mut [u8], size: IVec2) {
    let body_color = 0xFFFFFFFF; // white
    let ear_color = 0xFF888888; // gray
    let eye_color = 0xFF000000; // black
    let nose_color = 0xFFFF00FF; // pink
    let whisker_color = 0xFF0000FF; // blue
    let mouth_color = 0xFF000000; // black

    // draw body
    draw_filled_circle(buffer, size, IVec2::new(32, 32), 30, body_color);
    draw_filled_circle(buffer, size, IVec2::new(32, 38), 26, body_color);
    draw_filled_circle(buffer, size, IVec2::new(32, 44), 22, body_color);

    // draw ears
    draw_filled_circle(buffer, size, IVec2::new(15, 15), 7, ear_color);
    draw_filled_circle(buffer, size, IVec2::new(49, 15), 7, ear_color);

    // draw eyes
    draw_filled_circle(buffer, size, IVec2::new(22, 22), 4, eye_color);
    draw_filled_circle(buffer, size, IVec2::new(42, 22), 4, eye_color);

    // draw nose
    draw_line(
        buffer,
        size,
        IVec2::new(32, 32),
        IVec2::new(32, 37),
        nose_color,
    );
    draw_filled_circle(buffer, size, IVec2::new(32, 37), 2, nose_color);

    // draw whiskers
    draw_line(
        buffer,
        size,
        IVec2::new(27, 35),
        IVec2::new(32, 45),
        whisker_color,
    );
    draw_line(
        buffer,
        size,
        IVec2::new(37, 35),
        IVec2::new(32, 45),
        whisker_color,
    );

    // draw mouth
    draw_line(
        buffer,
        size,
        IVec2::new(32, 40),
        IVec2::new(32, 44),
        mouth_color,
    );
}

pub fn draw(buffer: &mut [u8], size: IVec2) {
    draw_filled_rectangle(buffer, size, IVec2::ZERO, size, 0x005511ff);

    draw_filled_triangle(
        buffer,
        size,
        ivec2(size.x, 0),
        ivec2(0, size.y),
        size - ivec2(20, 20),
        0x552211ff,
    );

    draw_line(buffer, size, ivec2(0, 0), ivec2(100, 100), 0xff0000ff);
    draw_line(buffer, size, ivec2(0, 50), ivec2(100, 50), 0x00ff00ff);
    draw_line(buffer, size, ivec2(50, 0), ivec2(50, 100), 0x0000ffff);

    draw_line(buffer, size, ivec2(0, 0), ivec2(50, 100), 0xffff00ff);
    draw_line(buffer, size, ivec2(0, 0), ivec2(100, 50), 0xffff00ff);

    draw_filled_circle(buffer, size, ivec2(50, 50), 10, 0xff00ffff);

    draw_cat(buffer, size);

    draw_circle(buffer, size, ivec2(100, 100), 50, 0xffffffff);

    draw_filled_rectangle(buffer, size, ivec2(200, 100), ivec2(50, 30), 0x0055ffff);
}
