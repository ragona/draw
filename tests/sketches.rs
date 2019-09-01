use draw::canvas::Canvas;
use draw::style::{Fill, Stroke};
use draw::{render, shape, Drawing, Shape, Style, SvgRenderer, RGB};
use svg::node::element::path::Data;

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
}
