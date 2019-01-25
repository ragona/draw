use std::cmp;

use super::*;

pub type SpriteId = usize;

pub struct Sprite {
    children: Vec<Box<Drawable>>,
    transform: Transform,
    width: i32,
    height: i32,
    partitions: Vec<Vec<Partition>>,
    slices: i32,
}

#[derive(Debug)]
struct Partition {
    bounds: Bounds,
    children: Vec<SpriteId>,
    child_bounds: Bounds,
}

impl Partition {
    pub fn new(bounds: Bounds) -> Partition {
        Partition {
            bounds,
            children: vec![],
            /// Child bounds are set to initially be exclusive; they are 0x0 and positioned at the
            /// bottom right corner of the area they cover. As children are added the child_bounds
            /// expand to contain the added objects.
            child_bounds: Bounds::new(bounds.width, bounds.height, 0, 0),
        }
    }
}

unsafe impl Sync for Sprite {}

impl Sprite {
    pub fn new(width: i32, height: i32) -> Sprite {
        let mut sprite = Sprite {
            width,
            height,
            children: vec![],
            transform: Transform::new(),
            partitions: vec![],
            slices: 1,
        };

        sprite.reset_partitions();
        sprite
    }

    ///
    /// Sprite partitioned to more efficiently draw many children. You should use this if
    /// particular areas of the screen will have a lot of children, but those children don't
    /// cover the entire screen. Add more slices for smaller children. This speeds up the process
    /// of `get_pixel` since it only looks for children within a specific partition, but it slows
    /// the process if you have large children that are across all partitions anyway.
    ///
    pub fn new_container(width: i32, height: i32, slices: i32) -> Sprite {
        let mut container = Sprite::new(width, height);
        container.slices = slices;
        container.reset_partitions();
        container
    }

    pub fn bounds(&self) -> Bounds {
        Bounds {
            x: self.transform.x,
            y: self.transform.y,
            width: self.width,
            height: self.height,
        }
    }

    pub fn background(&mut self, color: Pixel) {
        self.add_child(Rectangle::new(self.width as u32, self.height as u32, 0, 0, color));
    }

    pub fn add_child<T: 'static>(&mut self, child: T) -> SpriteId
    where
        T: Drawable + Sized,
    {
        let id = self.children.len();

        self.children.push(child.into_box());
        self.update_child_partition(id);

        id
    }

    pub fn get_child(&self, child_id: SpriteId) -> &Box<dyn Drawable> {
        self.children.get(child_id).expect("Did not find child_id")
    }

    pub fn get_child_mut(&mut self, child_id: SpriteId) -> &mut Box<dyn Drawable> {
        self.children.get_mut(child_id).expect("Did not find child_id")
    }

    pub fn update_partitions(&mut self) {
        self.reset_partitions();
        for sprite_id in 0..self.children.len() {
            self.update_child_partition(sprite_id);
        }
    }

    pub fn reset_partitions(&mut self) {
        let mut partitions: Vec<Vec<Partition>> = Vec::new();
        let slice_width = self.width / self.slices;
        let slice_height = self.height / self.slices;
        for y in 0..self.slices {
            let mut row: Vec<Partition> = vec![];
            for x in 0..self.slices {
                row.push(Partition::new(Bounds {
                    x: x * slice_width,
                    y: y * slice_height,
                    width: slice_width,
                    height: slice_height,
                }))
            }
            partitions.push(row);
        }
        self.partitions = partitions;
    }
    ///
    /// todo: Call this after children move?
    ///
    /// # Panics
    /// Panics if child_id is >= self.children.len()
    ///
    fn update_child_partition(&mut self, child_id: SpriteId) {
        let child = &self.children[child_id];
        for row in self.partitions.iter_mut() {
            for partition in row {
                let bounds = &child.bounds();
                if partition.bounds.overlaps(bounds) {
                    partition.children.push(child_id);
                    partition.child_bounds.surround(bounds);
                }
            }
        }
    }

    // todo: Are these accesses by index slow? Test unsafe_get.
    fn xy_to_partition(&self, x: i32, y: i32) -> &Partition {
        let x = cmp::min(self.slices - 1, x / (self.width / self.slices));
        let y = cmp::min(self.slices - 1, y / (self.height / self.slices));
        let row = &self.partitions[y as usize];

        &row[x as usize]
    }
}

impl Drawable for Sprite {
    fn width(&self) -> i32 {
        self.width
    }

    fn height(&self) -> i32 {
        self.height
    }

    fn transform(&self) -> Transform {
        self.transform
    }

    fn set_transform(&mut self, transform: Transform) {
        self.transform = transform;
    }

    ///
    /// Always succeeds. Returns a new pixel if the sprite has no matching children and no Drawable.
    ///
    fn get_pixel(&self, x: i32, y: i32) -> Pixel {
        let mut pixel = Pixel::new();
        let partition = self.xy_to_partition(x, y);

        if !partition.children.is_empty() && partition.child_bounds.contains(x, y) {
            for child_id in partition.children.iter().rev() {
                let child = &self.children[*child_id];
                if child.bounds().contains(x, y) {
                    let transform = &child.transform(); // fix rotation
                    pixel += child.get_pixel(x - transform.x, y - transform.y);
                    if pixel.a == 255 {
                        break;
                    }
                }
            }
        }

        pixel
    }

    fn rotate(&mut self, _degrees: u32) {
        unimplemented!() // todo
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Point, Result};

    #[test]
    fn new_sprite() {
        let sprite = Sprite::new(4, 5);

        assert_eq!(sprite.children.len(), 0);
        assert_eq!(sprite.width, 4);
        assert_eq!(sprite.height, 5);
    }

    #[test]
    fn add_child() -> Result<()> {
        let mut parent = Sprite::new(1, 1);
        let child = Sprite::new(1, 1);
        parent.add_child(child);

        assert_eq!(parent.children.len(), 1);

        Ok(())
    }

    #[test]
    fn get_pixel() {
        let mut parent = Sprite::new_container(4, 4, 1);

        let child_id = parent.add_child(
            Rectangle::new(2, 2, 1, 1, Pixel::red())
        );

        for x in 0..4 {
            for y in 0..4 {
                let pixel = parent.get_pixel(x, y);
                if parent.children[child_id].bounds().contains(x, y) {
                    assert_eq!(Pixel::red(), pixel);
                } else {
                    assert_eq!(Pixel::new(), pixel);
                }
            }
        }
    }

    #[test]
    fn xy_to_partition() -> Result<()> {
        const SIZE: i32 = 100;

        let one = Sprite::new_container(SIZE, SIZE, 1);
        let four = Sprite::new_container(SIZE, SIZE, 2);
        let sixteen = Sprite::new_container(SIZE, SIZE, 4);
        let sixty_four = Sprite::new_container(SIZE, SIZE, 8);

        for sprite in &vec![one, four, sixteen, sixty_four] {
            sprite.xy_to_partition(SIZE - 1, SIZE - 1); // can panic
        }

        Ok(())
    }

    #[test]
    fn update_child_bounds() -> Result<()> {
        let mut parent = Sprite::new(100, 100);
        let mut child = Sprite::new(50, 50);

        child.transform.x = 25;
        child.transform.y = 25;
        parent.add_child(child);

        let partition = &parent.partitions[0][0];

        assert_eq!(partition.child_bounds.top_left(), Point { x: 25, y: 25 });
        assert_eq!(partition.child_bounds.bot_right(), Point { x: 75, y: 75 });

        Ok(())
    }

}
