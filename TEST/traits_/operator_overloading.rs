use std::ops;

struct Foo;
struct Bar;

#[derive(Debug)]
struct FooBar;
#[derive(Debug)]
struct BarFoo;

#[derive(Debug)]
struct ExVec(i32, i32);

impl ops::Add<Bar> for Foo {
    type Output = FooBar;
    fn add(self, _rhs: Bar) ->FooBar{
        println!("> Foo.add(Bar) was called");
        FooBar
    }
}
impl ops::Add<Foo> for Bar {
    type Output = BarFoo;
    fn add(self, _rhs: Foo) ->BarFoo {
        println!("> Bar.add(Foo) was called");
        BarFoo
    }
}

impl ops::Add<ExVec> for ExVec {
    type Output = ExVec;
    fn add(self, _rhs: ExVec) ->ExVec {
        ExVec(self.0+_rhs.0, self.1+_rhs.1)
    }
}

fn main() {
    println!("Foo + Bar = {:?}", Foo+Bar);
    println!("Bar + Foo = {:?}", Bar+Foo);

    let temp1 = ExVec(32, 10);
    let temp2 = ExVec(0, 15);
    println!("{:?}", temp1+temp2);


}