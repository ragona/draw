# draw
*WARNING: This library is in early development! Expect changes.*

A tiny drawing library for Rust. 
Simple display list and vector shapes. 
Currently supports SVG output, designed to also support PNG and other formats.

## Example
```rust
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
        color: RGB::new(0, 0, 0),
    }),
    stroke: Some(Stroke {
        width: 2,
        color: RGB::new(255, 0, 0),
    }),
};

// add it to the canvas
canvas.display_list.add(rect);

// save the canvas as an svg
render::save(
    &canvas,
    "tests/svg/basic_end_to_end.svg",
    SvgRenderer::new(),
)
.expect("Failed to save");
```

## Todo list
- [ ] Lines
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