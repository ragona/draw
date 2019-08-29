use crate::canvas::Canvas;

pub enum Shape {
    Rectangle { width: u32, height: u32 },
}

//
//impl Draws for Rectangle {
//    fn draw(&self) {
//        let x = svg::node::element::Rectangle::new()
//            .set("x", 60)
//            .set("y", 10)
//            .set("width", 50)
//            .set("height", 50)
//            .set("fill", "black")
//            .set("stroke", "red")
//            .set("stroke-width", 1);
//    }
//}
