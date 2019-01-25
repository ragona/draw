#![allow(dead_code, unused_imports)]

#[cfg(test)]
mod tests {
    use draw::{self, samples, Bitmap, Drawable, Pixel, Result, Sprite};

    #[test]
    fn test_save() -> Result<()> {
        draw::save(&samples::bitmap_child(), "tests/img/bitmap_child.png")?;
        draw::save(&samples::rotate_bitmap(), "tests/img/rotate_bitmap.png")?;
        draw::save(
            &samples::four_corner_rectangles(),
            "tests/img/four_corner_rectangles.png",
        )?;
        draw::save(
            &samples::many_rectangles(10000),
            "tests/img/many_rectangles.png",
        )?;
        draw::save(
            &samples::circle_from_rotated_point(),
            "tests/img/circle_from_rotated_point.png",
        )?;

        Ok(())
    }

    #[test]
    fn red_square() -> Result<()> {
        draw::save(&samples::red_square(), "tests/img/red_square.png")
    }

    #[test]
    fn transparency() -> Result<()> {
        draw::save(&samples::transparent(), "tests/img/transparent.png")
    }

    #[test]
    fn circle() -> Result<()> {
        draw::save(&samples::circle(), "tests/img/circle.png")
    }

    #[test]
    fn circles() -> Result<()> {
        draw::save(&samples::circles(), "tests/img/circles.png")
    }

    #[test]
    fn spiral() -> Result<()> {
        draw::save(&samples::spiral(), "tests/img/spiral.png")
    }
}
