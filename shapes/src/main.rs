struct Rect {
    w: f64,
    h: f64,
}

impl Rect {
    fn new(width: f64, height: f64) -> Rect {
        Rect {
            w: width,
            h: height,
        }
    }

    fn area(&self) -> f64 {
        self.h * self.w
    }

    fn grow(&mut self, by: f64) {
        self.w += by;
        self.h += by;
    }
    fn is_square(&self) -> bool {
        self.h == self.w
    }
}

fn main() {
    let mut rect1 = Rect::new(44.0, 88.0);
    let area = rect1.area();
    println!("{area}");
    rect1.grow(10.0);
    println!("{}", rect1.is_square());
    println!("after : {} x{}", rect1.h, rect1.w);
}
