# draw
*WARNING: This library is in development! It's not usable yet.*

A tiny drawing library for Rust. 
Simple display list, vector and bitmap shapes. 
Intended to be used to produce single images, or image sequences for creating animated mp4 or gif files. 

```rust
// create a canvas to draw on
let mut canvas = Canvas::new(100, 100, None);

// create some drawings of rectangles
let a = Drawing::new(Shape::Rectangle {
    width: 50,
    height: 50,
});

let b = Drawing::new(Shape::Rectangle {
    width: 10,
    height: 10,
});

// add those drawings to the canvas
canvas.display_list.add(a);
canvas.display_list.add(b);

// save the canvas as an svg
render::save(&canvas, "tests/svg/basic_end_to_end.svg", SvgRenderer::new())
    .expect("Failed to save");
```

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

## Todo list
- [ ] Draw anything other than a rectangle
