# draw
A tiny drawing library for Rust. Simple display list, vector and bitmap shapes. Intended to be used to produce single images, or image sequences for creating animated mp4 or gif files. Very early in development.

```rust
use draw::{self, Sprite, SpriteId, Rectangle, Pixel};

let mut parent = Sprite::new(100, 100);
let mut child = Rectangle::new(50, 50, 25, 25, Pixel::red());

parent.add_child(child);

draw::save(&parent, "tests/img/example.png");
```
## Design
`draw` is intended to be a friendly interface for creating images.

### Modules

#### `canvas`

A `canvas` is a surface that can be drawn to. This will usually be the first thing that you create, and you'll add elements to it.

#### `shapes`

A collection of pre-defined shapes, such as `circle`, `rectangle`, `triangle`, etc. 

#### `path`

A custom shape defined by a series of points and curves. 

#### `style`

A struct that defines the visual appearance of an object, such as the fill and stroke.

#### `container`

A trait describing any element that can contain other elements. 

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
- [ ] Split Drawable into multiple traits? (e.g. "Rotateable")
- [ ] Anti-aliasing, float positioning?
- [ ] Fix child transformations updating partition child bounds
- [ ] Track corners like verticies instead of using bounding boxes
- [ ] Add bitmap features (shear, brightness, multiply, dissolve)
- [ ] Implement rotate for rectangles / sprites
- [ ] Try 3 shear rotation for bitmaps
- [ ] Visibility
- [ ] Scaling
- [ ] Scaled saves
- [ ] Images
- [ ] OpenGL rendering

