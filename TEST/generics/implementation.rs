
struct S;
struct GenericVal<T>(T);

impl GenericVal<f32> {}
impl GenericVal<S> {} //Concrete type 'S'
impl<S> GenericVal<S> {} //Generic anoymous type 'S'

struct Val {
    val: f64,
}

struct GenVal<T> {
    gen_val: T,
}

impl Val {
    fn value(&self) ->&f64 {
        &self.val
    }
}

impl<K> GenVal<K> {
    fn value(&self) ->&K {
        &self.gen_val
    }
}

fn main() {
    let x = Val { val: 3.0 };
    let y = GenVal { gen_val: 3i32 };

    println!("Two Value: {}, {}", x.value(), y.value());
}