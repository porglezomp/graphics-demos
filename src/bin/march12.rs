use pixel_canvas::prelude::*;

#[derive(Debug)]
struct Camera {
    pos: Vec3,
    dir: Vec3,
}

#[derive(Debug)]
struct Hit {
    point: Vec3,
    normal: Vec3,
}

fn render(pos: Vec3, dir: Vec3) -> Color {
    let upness = dir.dot(xyz(0.0, 0.0, 1.0));
    let upness = (upness + 0.2).restrict(0.0..=1.0);
    let sky = rgb(255, 220, 200).blend(rgb(64, 127, 255), upness);
    let light_dir = xyz(2.0, 0.2, 1.5).normal();
    match march(pos, dir, 300, 0.5) {
        Some(hit) => {
            let dist = (hit.point - pos).len();
            if dist > 150.0 {
                return sky;
            }
            let fog = dist / 150.0;
            let height = hit.point.z.remap(-6.0..3.0, 0.0..1.0).restrict(0.0..=1.0);
            let color = if hit.point.z <= -0.95 {
                rgb(16, 64, 128)
            } else if hit.point.z <= -0.5 {
                rgb(255, 240, 128)
            } else {
                let rock_fac = hit.point.z.restrict(6.0..=10.0).remap(6.0..10.0, 0.0..1.0);
                let snow_fac = hit.normal.z.restrict(0.0..=1.0).sqrt();
                let rock = rgb(40, 40, 40).blend(Color::WHITE, snow_fac);
                (rgb(128, 255, 64) * height).blend(rock, rock_fac)
            };
            let sun_fac = if march(hit.point + hit.normal * 0.04, light_dir, 300, 0.5).is_none() {
                1.0
            } else {
                0.0
            };
            let sun_light = hit.normal.dot(light_dir).restrict(0.0..=1.0);
            let sky_fac = hit.normal.z.remap(0.0..1.0, 0.4..1.0).restrict(0.0..=1.0);
            let light = rgb(255, 250, 240) * sun_light * sun_fac + rgb(16, 32, 96) * sky_fac;
            (color * light).blend(sky, fog)
        }
        None => sky,
    }
}

fn height(x: f32, y: f32) -> f32 {
    let ground = (x * 0.5 + (y * 0.1).sin()).sin() * (y * 0.5 + (x * 0.05).cos()).sin()
        + (x * 0.1).sin() * (y * 0.1).sin() * 2.0
        + (x * 0.02).sin() * (y * 0.02).sin() * 4.0;
    (ground * ground.abs()).restrict(-1.0..)
}

fn normal(x: f32, y: f32) -> Vec3 {
    let eps = 0.001;
    let root = xyz(x, y, height(x, y));
    let a = xyz(x + eps, y, height(x + eps, y));
    let b = xyz(x, y + eps, height(x, y + eps));
    (a - root).cross(b - root).normal()
}

fn march(mut pos: Vec3, dir: Vec3, steps: usize, dt: f32) -> Option<Hit> {
    let dir = dir.normal();
    for _ in 0..steps {
        let height = height(pos.x, pos.y);
        if pos.z < height {
            if dt >= 0.01 {
                return march(pos - dir * dt, dir, 32, dt / 2.0);
            }
            return Some(Hit {
                point: pos,
                normal: normal(pos.x, pos.y),
            });
        }
        pos = pos + dir * dt;
    }
    None
}

fn main() {
    let canvas = Canvas::new(300, 720)
        .hidpi(false)
        .title("Mountains")
        .state(Camera {
            pos: xyz(0.0, 0.0, 5.0),
            dir: xyz(0.0, 1.0, 0.0),
        })
        .render_on_change(true);
    canvas.render(move |camera, img| {
        let (w, h) = (img.width() as usize, img.height() as usize);
        let aspect = w as f32 / h as f32;
        for (y, row) in img.chunks_mut(w).enumerate() {
            let y = (y as f32).remap(0.0..h as f32, -1.0..1.0);
            for (x, pixel) in row.iter_mut().enumerate() {
                let x = (x as f32).remap(0.0..w as f32, -1.0..1.0);
                let x = x * aspect;
                let dir = camera.dir(x, y);
                *pixel = render(camera.pos, dir);
            }
        }
    });
}

impl Camera {
    fn dir(&self, x: f32, y: f32) -> Vec3 {
        Vec3 { x, y: 1.0, z: y }.normal()
    }
}

fn xyz(x: f32, y: f32, z: f32) -> Vec3 {
    Vec3 { x, y, z }
}

fn rgb(r: u8, g: u8, b: u8) -> Color {
    Color { r, g, b }
}
