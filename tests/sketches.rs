use draw::canvas::Canvas;
use draw::shape::StraightLine;
use draw::style::{Fill, Stroke};
use draw::{render, Drawing, Position, Shape, Style, SvgRenderer, RGB};

#[test]
fn basic_end_to_end() {
    // create a canvas to draw on
    let mut canvas = Canvas::new(100, 100);

    // create a rectangle
    let mut rect = Drawing::new(Shape::Rectangle {
        width: 50,
        height: 50,
    });

    // move it around
    rect.position.x = 25.0;
    rect.position.y = 25.0;

    // give it a cool style
    rect.style = Style {
        fill: Some(Fill {
            color: RGB::new(75, 75, 75),
        }),
        stroke: Some(Stroke {
            width: 2,
            color: RGB::new(255, 0, 0),
        }),
    };

    // put a circle in the middle of it
    let mut circle = Drawing::new(Shape::Circle { radius: 10 });
    circle.position.x = 50.0;
    circle.position.y = 50.0;

    // add them to the canvas
    canvas.display_list.add(rect);
    canvas.display_list.add(circle);

    // save the canvas as an svg
    render::save(
        &canvas,
        "tests/svg/basic_end_to_end.svg",
        SvgRenderer::new(),
    )
    .expect("Failed to save");
}

#[test]
fn lines() {
    let mut canvas = Canvas::new(100, 100);

    // create a line that starts in the top middle of the screen, then draw down the middle
    let mut line = StraightLine::new(Position::new(50.0, 10.0));
    line.line_to(Position::new(60.0, 50.0));
    line.line_to(Position::new(40.0, 100.0));

    // turn that line into a shape, give it a stroke, and add it to the canvas
    let mut drawing = Drawing::new(line.into());
    drawing.style = Style::stroked(5, RGB::new(0, 0, 0));
    canvas.display_list.add(drawing);

    // save the canvas as an svg
    render::save(&canvas, "tests/svg/lines.svg", SvgRenderer::new()).expect("Failed to save");
}
