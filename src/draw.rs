use glam::{ivec2, IVec2};

fn draw_pixel(buffer: &mut [u8], size: IVec2, pos: IVec2, color: u32) {
    let offset = 4 * (size.x * pos.y + pos.x);

    if let Some(pixel_ref) = buffer.get_mut((offset as usize)..(offset as usize + 4)) {
        pixel_ref[0] = (color >> 24) as u8;
        pixel_ref[1] = (color >> 16) as u8;
        pixel_ref[2] = (color >> 8) as u8;
        pixel_ref[3] = color as u8;
    }
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
            draw_pixel(buffer, size, ivec2(x, y), color);
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
            ivec2(pos.x + x, pos.y + y),
            ivec2(pos.x - x, pos.y + y),
            color,
        );
        draw_line(
            buffer,
            size,
            ivec2(pos.x + y, pos.y + x),
            ivec2(pos.x - y, pos.y + x),
            color,
        );
        draw_line(
            buffer,
            size,
            ivec2(pos.x - x, pos.y - y),
            ivec2(pos.x + x, pos.y - y),
            color,
        );
        draw_line(
            buffer,
            size,
            ivec2(pos.x - y, pos.y - x),
            ivec2(pos.x + y, pos.y - x),
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
        draw_pixel(buffer, size, pos + ivec2(x, y), color);
        draw_pixel(buffer, size, pos + ivec2(-x, y), color);
        draw_pixel(buffer, size, pos + ivec2(x, -y), color);
        draw_pixel(buffer, size, pos + ivec2(-x, -y), color);
        draw_pixel(buffer, size, pos + ivec2(y, x), color);
        draw_pixel(buffer, size, pos + ivec2(-y, x), color);
        draw_pixel(buffer, size, pos + ivec2(y, -x), color);
        draw_pixel(buffer, size, pos + ivec2(-y, -x), color);

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
            draw_pixel(buffer, size, ivec2(x, y), color);
        }
    }
}

fn draw_filled_triangle(buffer: &mut [u8], size: IVec2, a: IVec2, b: IVec2, c: IVec2, color: u32) {
    // Sort the points so that a.y <= b.y <= c.y
    let (a, b, c) = sort_points(a, b, c);

    // Compute the slopes of the edges of the triangle
    let slope_ab = (b.x - a.x) as f32 / (b.y - a.y) as f32;
    let slope_bc = (c.x - b.x) as f32 / (c.y - b.y) as f32;
    let slope_ca = (a.x - c.x) as f32 / (a.y - c.y) as f32;

    // Iterate over the scanlines of the triangle
    for y in a.y..c.y {
        let x1: f32;
        let x2: f32;

        if y < b.y {
            // Compute the x coordinates of the left and right edges of the scanline
            x1 = a.x as f32 + (y - a.y) as f32 * slope_ab;
            x2 = a.x as f32 + (y - a.y) as f32 * slope_ca;
        } else {
            // Compute the x coordinates of the left and right edges of the scanline
            x1 = b.x as f32 + (y - b.y) as f32 * slope_bc;
            x2 = a.x as f32 + (y - a.y) as f32 * slope_ca;
        }

        // Compute the starting and ending x coordinates of the scanline
        let start_x = x1.min(x2) as i32;
        let end_x = x1.max(x2) as i32;

        // Iterate over the pixels of the scanline and fill in the ones that are inside the triangle
        for x in start_x..end_x {
            draw_pixel(buffer, size, ivec2(x, y), color);
        }
    }
}

fn sort_points(a: IVec2, b: IVec2, c: IVec2) -> (IVec2, IVec2, IVec2) {
    let mut points = [a, b, c];
    if points[0].y > points[1].y {
        points.swap(0, 1);
    }
    if points[1].y > points[2].y {
        points.swap(1, 2);
    }
    if points[0].y > points[1].y {
        points.swap(0, 1);
    }
    (points[0], points[1], points[2])
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

fn draw_uniform_cubic_b_spline(buffer: &mut [u8], size: IVec2, points: &[IVec2], color: u32) {
    // Check if there are enough points to draw a uniform cubic B-spline
    if points.len() < 4 {
        return;
    }

    // Compute the B-spline coefficients
    const C: f32 = 1.0 / 6.0;

    // Iterate over the control points of the uniform cubic B-spline
    let mut prev_pos = IVec2::new(0, 0);
    for i in 0..points.len() - 3 {
        // Compute the coordinates of the current uniform cubic B-spline segment
        let p0 = points[i].as_vec2();
        let p1 = points[i + 1].as_vec2();
        let p2 = points[i + 2].as_vec2();
        let p3 = points[i + 3].as_vec2();

        // Iterate over the steps of the uniform cubic B-spline segment
        for t in 0..32 {
            // Compute the interpolated x and y coordinates
            let t = t as f32 / 32.0;
            let ct = C * t;
            let ctt = ct * t;
            let cttt = ctt * t;

            let x = C * (p0.x + 4.0 * p1.x + p2.x)
                + ct * (-3.0 * p0.x + 3.0 * p2.x)
                + ctt * (3.0 * p0.x - 6.0 * p1.x + 3.0 * p2.x)
                + cttt * (-1.0 * p0.x + 3.0 * p1.x - 3.0 * p2.x + p3.x);

            let y = C * (p0.y + 4.0 * p1.y + p2.y)
                + ct * (-3.0 * p0.y + 3.0 * p2.y)
                + ctt * (3.0 * p0.y - 6.0 * p1.y + 3.0 * p2.y)
                + cttt * (-1.0 * p0.y + 3.0 * p1.y - 3.0 * p2.y + p3.y);

            // Draw the current uniform cubic B-spline point
            let pos = IVec2::new(x as i32, y as i32);
            if t > 0.0 {
                draw_line(buffer, size, prev_pos, pos, color);
            }
            prev_pos = pos;
        }
    }
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

    draw_uniform_cubic_b_spline(
        buffer,
        size,
        &[
            ivec2(50, 50),
            ivec2(size.x - 50, 50),
            ivec2(size.x - 50, size.y - 50),
            ivec2(50, size.y - 50),
            ivec2(50, 50),
        ],
        0xff0000ff,
    )
}
