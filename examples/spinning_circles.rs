use draw::{self, Sprite, SpriteId, Circle, Pixel, Result, Point, Transform};

/// This one doesn't really work! What I wanted was four circles spinning around a single
/// circle in the middle of the screen. I'm doing that by rotating a list of points around
/// the center, but what happens is that it sometimes spins smoothly, and sometimes jumps
/// around. The rotation code apparently has some rough edges.
fn main() -> Result<()> {
    let starting_points = vec![
        Point{x: 450, y: 450},
        Point{x: 300, y: 300},
        Point{x: 600, y: 300},
        Point{x: 600, y: 600},
        Point{x: 300, y: 600}];

    let center = Point{x: 450, y: 450};
    let mut parent = Sprite::new(900, 900);
    let mut circle_ids: Vec<SpriteId> = vec![];

    parent.background(Pixel::rgb(50, 50, 50));

    for point in starting_points {
        let circle_id: SpriteId = parent.add_child(
            Circle::new(250, point.x, point.y, Pixel::rgba(255, 0, 0, 50))
        );

        circle_ids.push(circle_id);
    }

    for i in 0..360 {
        for id in &circle_ids {
            let circle = parent.get_child_mut(*id);
            let current_transform = circle.transform();
            let cur_point = Point{x: current_transform.x, y: current_transform.y};
            let next_point = draw::rotate_point(cur_point, center, (i) as u32);

            circle.set_transform(Transform::xy(next_point.x, next_point.y))
        }

        let filename = &format!("tmp/{:03}.png", i);
        println!("Saving {}", filename);
        draw::save(&parent, filename)?;
    }

    Ok(())
}