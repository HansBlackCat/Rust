
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool
}
struct  VectorStruct (i32, i32, i32);
#[derive(Debug)]
struct  Rectangle {
    width: i32,
    height: i32,
    // fn Akit(&self) ->i32 { self...  // can't fn in struct, use impl
}

impl Rectangle {
    // Associated Functions, use :: to call AF
    fn square(size: i32) ->Rectangle {
        Rectangle { width: size, height: size }
    }
    fn area(&self) ->i32 {
        self.width*self.height
    }
    fn can_hold(&self, other: &Rectangle) ->bool {
        self.width > other.width && self.height > other.height
    }
}


fn main() {
    let user1 = User {
        email: String::from("abc@example.com"),
        username: String::from("NaNaNa"),
        active: true,
        sign_in_count: 1
    };

    let mut user2 = User {
        email: String::from("abc@example.com"),
        username: String::from("NaNaNa"),
        active: true,
        sign_in_count: 1
    };
    user2.active = false;

    let user3 = User {
        email: String::from("abc@example.com"),
        username: user1.username,
        ..user2
    };
    let user4 = user_maker_with_default(String::from("Kim"), String::from("TO@abc@example.com"));

    let some_point = VectorStruct(3,5,-3);

    let rec1 = Rectangle{ width: 30, height: 15 };
    let rec2 = Rectangle{ width: 25, height: 10 };
    let area_rec1 = area(rec1);
    println!("Area of rec1: {}", area_rec1);
    println!("rec1 is {:?}", rec2);

    let rec_for_impl = Rectangle { width:10, height:6 };
    println!("Area of rec_for_impl is: {}", rec_for_impl.height*rec_for_impl.width);
    println!("Area of rec_for_impl is: {}", rec_for_impl.area());

    let rect1 = Rectangle { width:30, height:10 };
    let rect2 = Rectangle { width:10, height:5 };
    let rect3 = Rectangle { width:70, height:5 };
    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    let rec_sq = Rectangle::square(30);
}

fn user_maker_with_default(username: String, email: String) ->User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1
    }
}
fn area(rec:Rectangle) ->i32 { rec.height*rec.width }