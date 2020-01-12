use std::mem;

fn analyze_slice(slice: &[i32]) {
    println!("first element of the slice: {}", slice[0]);
    println!("the slice has {} elements", slice.len());
}

fn main() {
    let single_arr: [i32; 1] = [0];

    let xs: [i32; 5] = [1, 2, 3, 4, 5];
    println!("array xs: {:?}", xs);

    let ys: [i32; 500] = [0; 500];
    println!("array ys: {:?}", ys[120]);

    println!("array size: {}", xs.len());
    println!("array occupies {} bytes", mem::size_of_val(&xs));
    println!("single-element array occpies {} bytes\n", mem::size_of_val(&single_arr));

    // array -> slices
    analyze_slice(&xs);
    analyze_slice(&ys[0..]);

    let zs: [i32; 5] = [1,2,3,4,5];
    println!("\narray first element: {}", zs[0]);
    println!("array lenght (you need to use to_owned() method): {}", zs.to_owned().len());

}