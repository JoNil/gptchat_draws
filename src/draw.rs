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
        if idx >= 0 && idx < buffer.len() as i32 {
            write_color(buffer, idx, color);
        }

        let idx = 4 * (size.x * (pos.y + y) + pos.x - x);
        if idx >= 0 && idx < buffer.len() as i32 {
            write_color(buffer, idx, color);
        }

        let idx = 4 * (size.x * (pos.y - y) + pos.x + x);
        if idx >= 0 && idx < buffer.len() as i32 {
            write_color(buffer, idx, color);
        }

        let idx = 4 * (size.x * (pos.y - y) + pos.x - x);
        if idx >= 0 && idx < buffer.len() as i32 {
            write_color(buffer, idx, color);
        }

        let idx = 4 * (size.x * (pos.y + x) + pos.x + y);
        if idx >= 0 && idx < buffer.len() as i32 {
            write_color(buffer, idx, color);
        }

        let idx = 4 * (size.x * (pos.y + x) + pos.x - y);
        if idx >= 0 && idx < buffer.len() as i32 {
            write_color(buffer, idx, color);
        }

        y += 1;
        err += 1 + 2 * y;
        if 2 * (err - x) + 1 > 0 {
            x -= 1;
            err += 1 - 2 * x;
        }
    }
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
    draw_line(buffer, size, ivec2(0, 0), ivec2(100, 100), 0xff0000ff);
    draw_line(buffer, size, ivec2(0, 50), ivec2(100, 50), 0x00ff00ff);
    draw_line(buffer, size, ivec2(50, 0), ivec2(50, 100), 0x0000ffff);

    draw_line(buffer, size, ivec2(0, 0), ivec2(50, 100), 0xffff00ff);
    draw_line(buffer, size, ivec2(0, 0), ivec2(100, 50), 0xffff00ff);

    draw_filled_circle(buffer, size, ivec2(50, 50), 10, 0xff00ffff);

    draw_cat(buffer, size);

    draw_circle(buffer, size, ivec2(100, 100), 50, 0xffffffff)
}
