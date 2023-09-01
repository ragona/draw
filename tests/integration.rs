use draw::*;
use rand::rngs::SmallRng;
use rand::{Rng, SeedableRng};

fn lines_canvas() -> Canvas {
    let mut canvas = Canvas::new(100, 100);

    // create a line that starts in the top middle of the screen, then draw down the middle
    let line = LineBuilder::new(50.0, 10.0)
        .curve_to(50.0, 50.0, 75.0, 30.0)
        .line_to(50.0, 75.0)
        .build();

    // turn that line into a shape, give it a stroke, and add it to the canvas
    let drawing = Drawing::new()
        .with_shape(line)
        .with_style(Style::stroked(1, RGB::new(0, 0, 0)));

    canvas.display_list.add(drawing);

    canvas
}

#[test]
fn lines_svg() {
    let canvas = lines_canvas();

    // save the canvas as an svg
    render::save(&canvas, "tests/svg/lines.svg", SvgRenderer::new()).expect("Failed to save");
}

#[test]
fn lines_png() {
    let canvas = lines_canvas();

    // save the canvas as an png
    render::save(&canvas, "tests/png/lines.png", PngRenderer::new()).expect("Failed to save");
}

fn random_circles_canvas() -> Canvas {
    let mut canvas = Canvas::new(1000, 1000);
    let mut rng = SmallRng::from_seed([0; 16]);
    let mut points = vec![];

    for _ in 0..10000 {
        points.push(Point {
            x: rng.gen_range(250, 750) as f32,
            y: rng.gen_range(0, 1000) as f32,
        })
    }

    for point in points {
        let mut circle = Drawing::new().with_shape(Shape::Circle { radius: 25 });
        circle.position.x = point.x;
        circle.position.y = point.y;
        circle.style = Style::stroked(2, Color::random());
        canvas.display_list.add(circle);
    }

    canvas
}

#[test]
fn random_circles_svg() {
    let canvas = random_circles_canvas();

    draw::render::save(&canvas, "tests/svg/random_circles.svg", SvgRenderer::new())
        .expect("Failed to save");
}

#[test]
fn random_circles_png() {
    let canvas = random_circles_canvas();

    draw::render::save(&canvas, "tests/png/random_circles.png", PngRenderer::new())
        .expect("Failed to save");
}
