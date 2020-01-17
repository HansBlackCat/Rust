
// concrete type
struct A;

// concrete type
struct Single(A);

// gen type
struct SingleGen<T>(T);

#[allow(dead_code)]
fn main() {
    let s = Single(A);

    let chr: SingleGen<char> = SingleGen('a');

    let _t: SingleGen<A> = SingleGen(A);
    let _i32: SingleGen<i32> = SingleGen(6);
}