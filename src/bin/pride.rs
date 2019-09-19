use pixel_canvas::prelude::*;

fn main() {
    let canvas = Canvas::new(1280, 720).title(":3").hidpi(true);
    canvas.render(move |_, image| {
        let width = image.width();
        let height = image.height();
        for (y, row) in image.chunks_mut(width).enumerate() {
            for pixel in row {
                *pixel = [
                    Color::rgb(85, 205, 252),
                    Color::rgb(247, 168, 184),
                    Color::rgb(255, 255, 255),
                    Color::rgb(247, 168, 184),
                    Color::rgb(85, 205, 252),
                ][y * 5 / height];
            }
        }
    });
}
