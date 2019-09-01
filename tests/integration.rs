use draw::*;

#[test]
fn lines() {
    let mut canvas = Canvas::new(100, 100);

    // create a line that starts in the top middle of the screen, then draw down the middle
    let mut line = LineBuilder::new(Position::new(50.0, 10.0));
    line.curve_to(Position::new(50.0, 50.0), Position::new(20.0, 30.0));
    line.line_to(Position::new(50.0, 75.0));

    // turn that line into a shape, give it a stroke, and add it to the canvas
    let mut drawing = Drawing::new(line.into());
    drawing.style = Style::stroked(1, RGB::new(0, 0, 0));
    canvas.display_list.add(drawing);

    // save the canvas as an svg
    render::save(&canvas, "tests/svg/lines.svg", SvgRenderer::new()).expect("Failed to save");
}
