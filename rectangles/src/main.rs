#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn width(&self) -> bool {
        self.width > 0
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

impl Rectangle {
    fn is_bigger(&self, other: &Rectangle) -> bool {
        self.area() > other.area()
    }
}

fn main() {
    let width1 = 30;
    let height1 = 50;

    println!(
        "長方形的面積為 {} 平方像素。",
        area(width1, height1)
    );

    let rect1 = (30, 50);
    println!(
        "長方形的面積為 {} 平方像素。",
        area_v2(rect1)
    );

    let rect2 = Rectangle {
        width: 30,
        height: 50,
    };
    println!(
        "長方形的面積為 {} 平方像素。",
        area_v3(&rect2)
    );

    // println!("rect1 is {}", rect1);
    // println!("rect2 is {}", rect2);
    println!("rect2 is {:?}", rect2);
    println!("rect2 is {:#?}", rect2);

    let scale = 2;
    let rect3 = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };
    dbg!(&rect3);

    let rect4 = Rectangle {
        width: 30,
        height: 50,
    };
    println!(
        "長方形的面積為 {} 平方像素。",
        rect4.area()
    );

    if rect4.width() {
        println!("長方形的寬度不為零，而是 {}", rect4.width);
    }

    let rect5 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect6 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect7 = Rectangle {
        width: 60,
        height: 45,
    };
    println!("rect5 能容納 rect6 嗎？{}", rect5.can_hold(&rect6));
    println!("rect5 能容納 rect7 嗎？{}", rect5.can_hold(&rect7));
    println!("rect5 面積大於 rect6 嗎？{}", rect5.is_bigger(&rect6));
    println!("rect5 面積大於 rect7 嗎？{}", rect5.is_bigger(&rect7));

    let sq = Rectangle::square(3);
    dbg!(&sq);
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}

fn area_v2(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

fn area_v3(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}