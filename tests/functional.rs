use draw::*;

fn test_canvas() -> Canvas {
    // create a canvas to draw on
    let mut canvas = Canvas::new(100, 100);

    // create a new drawing
    let rect = Drawing::new()
        // give it a shape
        .with_shape(Shape::Rectangle {
            width: 50,
            height: 50,
        })
        // move it around
        .with_xy(25.0, 25.0)
        // give it a cool style
        .with_style(Style::stroked(5, Color::black()));

    // add it to the canvas
    canvas.display_list.add(rect);

    canvas
}

#[test]
fn basic_end_to_end_svg() {
    let canvas = test_canvas();

    // save the canvas as an svg
    render::save(
        &canvas,
        "tests/svg/basic_end_to_end.svg",
        SvgRenderer::new(),
    )
    .expect("Failed to save");
}

#[test]
fn basic_end_to_end_png() {
    let canvas = test_canvas();

    // save the canvas as a png
    render::save(
        &canvas,
        "tests/png/basic_end_to_end.png",
        PngRenderer::new(),
    )
    .expect("Failed to save");
}
