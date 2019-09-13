# draw
*WARNING: This library is in early development! Expect changes.*

A small Rust library for 2D drawing. 
Simple display list and vector shapes. 
Currently supports SVG output, designed to also support PNG and other formats.

## Example
```rust
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

// save the canvas as an svg
render::save(
    &canvas,
    "tests/svg/basic_end_to_end.svg",
    SvgRenderer::new(),
)
.expect("Failed to save")
```

## Todo list
- [ ] Bezier curves
- [x] Lines
- [ ] Testing
- [ ] Add a bunch more shapes
- [ ] Clean up the SVG renderer shapes
- [x] Draw anything other than a rectangle
- [x] Positions
- [x] Styles 
- [ ] Bitmap image output

## Useful Commands

### Convert PNG sequence to mp4
```
ffmpeg -framerate 60 -pattern_type glob -i '*.png' -c:v libx264 -pix_fmt yuv420p out. mp4
```
### Convert PNG sequence to gif
```
convert -delay 1 *.png output.gif
```
### Profiling
```
export RUSTFLAGS='-g'
perf record --call-graph=lbr cargo run --release
perf report
```