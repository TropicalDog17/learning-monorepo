use std::f64::consts::PI;

pub trait Shape {
    fn area(&self) -> f64;
}

pub struct Square {
    pub edge: f64
}

impl Shape for Square {
    fn area(&self) -> f64 {
        println!("Hey, square area is calculated");
        self.edge * self.edge
    }
}

pub struct Circle {
    pub radius: f64
}

impl Shape for Circle {
    fn area(&self) -> f64 {
        println!("Hey, circle area is calculated");
        self.radius * self.radius * PI
    }
}

fn get_trait_obj(num: f64) -> Box<dyn Shape> {
    if num > 10.0 {
        return Box::new(Circle {radius: num});
    }
    Box::new(Square{edge: num})
}
fn main() {
    let a = get_trait_obj(11.0);
    let _ = a.area();
}
