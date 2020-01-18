


// trait Syntax Sugar
trait Contains {
    type A;
    type B;

    fn contains(&self, &self::A, &self::B) ->bool;
}

// Not using associated types
fn difference<A, B, C>(container: &C) ->i32 where 
    C: Contains<A, B> { 9 }

// Using associated types
fn difference_with_asty<C: Contains>(container: &C) ->i32 { 9 }

struct Container(i32, i32);

trait Contain for Container {
    type A;
    type B;

    fn contain(&self, _: &self::A, _:&self::B) ->bool;
    fn first(&self) ->i32;
    fn last(&self) ->i32;
}
impl Contains for Container {
    type A = i32;
    type B = i32;

    fn contains(&self, number_1: &i32, number_2: &self::B) ->bool {
        (&self.0 == number_1) && (&self.1 == number_2)
    }
    fn first(&self) ->i32 { self.0 }
    fn last(&self) ->i32 { self.1 }
}
fn difference_as<C: Contains>(container: &C) ->i32 {
    container.last() - container.first()
}
fn main() {
    let number_1 = 3;
    let number_2 = 10;

    let container = Container(number_1, number_2);
    println!("Does container contain {} and {}: {}",
        &number_1, &number_2,
        container.contain(&number_1, &number_2));
    println!("First number: {}", container.first());
    println!("Last number: {}", container.last());

    println!("The difference is: {}", difference(&container));
}