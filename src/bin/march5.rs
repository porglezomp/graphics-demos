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
    match march(pos, dir, 300, 0.5) {
        Some(hit) => {
            let dist = (hit.point - pos).len();
            if dist > 150.0 {
                return sky;
            }
            let fog = dist / 150.0;
            let height = hit.point.z.remap(-6.0..3.0, 0.0..1.0).restrict(0.0..=1.0);
            let color = rgb(128, 255, 64) * height;
            color.blend(sky, fog)
        }
        None => sky,
    }
}

fn height(x: f32, y: f32) -> f32 {
    let wavy = (x * 0.2 + (y * 0.05).sin()).cos() * (y * 0.2 + (y * 0.03).sin()).cos();
    let base = (x * 0.1).cos() * (y * 0.1).cos();
    wavy * 3.0 + base * 2.0 - 0.5
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
