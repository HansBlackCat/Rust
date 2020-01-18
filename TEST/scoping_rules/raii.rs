//raii.rs

struct ToDrop;
impl Drop for ToDrop {
    fn drop(&mut self) {
        println!("--ToDrop is being dropped--");
    }
}
fn destroy_box(c: Box<i32>) {
    println!("Destroying a box that contains {}", c);
}

fn create_box() {
    let _box1 = Box::new(3i32);
} // box1, destroyed

// take ownership & destroy
fn eat_box_i32(boxed_i32: Box<i32>) {
    println!("Destroying box that contains {}", boxed_i32);
}

// literal borrow
fn borrow_i32(borrowed_i32: &i32) {
    println!("This int is: {}", borrowed_i32);
}

fn main() {
    let _box2 = Box::new(5i32);

    {
        let _box3 = Box::new(4i32);
    } // box3, destroyed

    for _ in 0u32..1_000 {
        create_box();
    } // all, destroyed

    let _x = ToDrop;
    println!("Made a ToDrop!");

    // ______________________________
    let x = 5u32;
    let y = x;
    println!("x is {}, and y is {}", x, y);

    let a = Box::new(5i32);
    println!("a contains: {}", a);
    // Copying Pointer
    // a is not accessible
    let b = a; 
    destroy_box(b);
    
    // ______________________________
    let immuable_box = Box::new(5u32);
    println!("immuntable_box contains {}", immuable_box);

    // *immuable_box = 4;
    let mut mutable_box = immuable_box;
    println!("immuntable_box contains {}", mutable_box);

    *mutable_box = 3;
    println!("mutable_box now contains {}", mutable_box);

    // ______________________________
    println!("");
    let boxed_i32 = Box::new(5_i32);
    let stacked_i32 = 6_i32;

    borrow_i32(&boxed_i32);
    borrow_i32(&stacked_i32);

    {
        // take ref inside box
        let _ref_to_i32: &i32 = &boxed_i32;

        eat_box_i32(boxed_i32);

        // borrow_i32(_ref_to_i32);
    }
    // eat_box_i32(boxed_i32);
    
}