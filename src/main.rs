// rectangle struct
#[derive(Debug)]
struct Rectangle {
    height: u64,
    width: u64,
}

impl Rectangle {
    // &self - an instance method
    fn area(&self) -> u64 {
        self.height * self.width
    }

    fn can_contain(&self, other_rectangle: &Rectangle) -> bool {
        self.width > other_rectangle.width && self.height > other_rectangle.height
    }
}

fn main() {
    let rectangle = Rectangle {
        width: 30,
        height: 50
    };

    // you can print a struct with {:?}
    // and pretty print with {:#?}
    println!("Rectangle: {:#?}", rectangle);

    println!("Area of rectangle is: {}", rectangle.area());
}

#[test]
fn rectangle_can_contain_rectangle2() {
    let rectangle = Rectangle {
        width: 30,
        height: 50
    };

    let rectangle2 = Rectangle {
        width: 10,
        height: 30
    };

    assert_eq!(rectangle.can_contain(&rectangle2), true);
}

#[test]
fn rectangle_cannot_contain_rectangle2() {
    let rectangle = Rectangle {
        width: 30,
        height: 50
    };

    let rectangle2 = Rectangle {
        width: 60,
        height: 90
    };

    assert_eq!(rectangle.can_contain(&rectangle2), false);
}