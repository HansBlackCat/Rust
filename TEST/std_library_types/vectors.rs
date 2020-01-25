

fn main() {
    let collected_iterator: Vec<i32> = (0..=10).collect();
    println!(".collect (1..10) > {:?}", collected_iterator);

    let mut xs = vec![6i32, 1, 3];
    println!("vec! > {:?}", xs);
    xs.push(4i32);
    println!("vec! with push > {:?}", xs);

    println!("immutable vec lenght > {:?}", collected_iterator.len());

    //enumerate
    for x in xs.iter() {
        println!("> {:?}", x);
    }
    println!("");
    for (x,y) in xs.iter().enumerate() {
        println!("> {:?}'th {:?}", x, y);
    }
    println!("");
    for x in xs.iter_mut() {
        *x *= 3;
        println!("> {:?}", x);
    }
    println!("");
    for (mut x,y) in xs.iter_mut().enumerate() {
        x = 4 - x;
        *y -= 3;
        println!("> {:?}'th {:?}", x, y); 
    }
    println!("{:?}", xs);
}