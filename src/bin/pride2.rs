use pixel_canvas::prelude::*;

fn main() {
    let canvas = Canvas::new(1280, 720).title(":3").hidpi(true);
    canvas.render(move |_, image| {
        let (w, h) = (image.width() as usize, image.height() as usize);
        for (y, row) in image.chunks_mut(w).enumerate() {
            for (x, pixel) in row.iter_mut().enumerate() {
                *pixel = [
                    [
                        Color::rgb(85, 205, 252),
                        Color::rgb(247, 168, 184),
                        Color::rgb(255, 255, 255),
                        Color::rgb(247, 168, 184),
                        Color::rgb(85, 205, 252),
                    ][y * 5 / h],
                    [
                        Color::rgb(33, 33, 33),
                        Color::rgb(155, 89, 208),
                        Color::rgb(255, 255, 255),
                        Color::rgb(255, 244, 51),
                    ][y * 4 / h],
                    [
                        Color::rgb(163, 2, 98),
                        Color::rgb(211, 98, 164),
                        Color::rgb(255, 255, 255),
                        Color::rgb(255, 154, 86),
                        Color::rgb(213, 45, 0),
                    ][y * 5 / h],
                ][x * 3 / w];
            }
        }
    });
}
