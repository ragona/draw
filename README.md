# draw
Toy rust library for drawing. Simple display list, vector and bitmap shapes. Intended to be used to produce single images or image sequences for creating animated mp4 or gif files. This is a quick learning project that I created to get my feet wet with Rust, and will likely not be developed much.

```rust
use draw::{self, Sprite, SpriteId, Rectangle, Pixel};

let mut parent = Sprite::new(100, 100);
let mut child = Rectangle::new(50, 50, 25, 25, Pixel::red());

parent.add_child(child);

draw::save(&parent, "tests/img/example.png");
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

