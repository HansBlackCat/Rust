
struct A;
struct S(A);
struct SGen<T>(T);

// Not generics
fn reg_fn(_s: S) {}
fn gen_spec_t(_s: SGen<A>) {}
fn gen_spec_i32(_s: SGen<i32>) {}

// Generics
fn generic<T>(_s: SGen<T>) {}

fn main() {
    reg_fn(S(A));
    gen_spec_t(SGen(A));
    gen_spec_i32(SGen(6));

    // Explicitly specified 
    generic::<char>(SGen('a'));
    // Implicitly specified
    generic(SGen('c'));
    // Can't
    // generic<u32>(SGen('d'));
}