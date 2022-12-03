use glam::{ivec2, IVec2};

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
            buffer[offset as usize] = (color >> 24) as u8;
            buffer[offset as usize + 1] = (color >> 16) as u8;
            buffer[offset as usize + 2] = (color >> 8) as u8;
            buffer[offset as usize + 3] = color as u8;
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

pub fn draw(buffer: &mut [u8], size: IVec2) {
    draw_line(buffer, size, ivec2(0, 0), ivec2(100, 100), 0xff0000ff);
    draw_line(buffer, size, ivec2(0, 50), ivec2(100, 50), 0x00ff00ff);
    draw_line(buffer, size, ivec2(50, 0), ivec2(50, 100), 0x0000ffff);

    draw_line(buffer, size, ivec2(0, 0), ivec2(50, 100), 0xffff00ff);
    draw_line(buffer, size, ivec2(0, 0), ivec2(100, 50), 0xffff00ff);

    draw_filled_circle(buffer, size, ivec2(50, 50), 10, 0xff00ffff);
}
