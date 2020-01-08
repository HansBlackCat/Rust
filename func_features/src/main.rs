use std::thread;
use std::time::Duration;
use std::ptr::eq;

struct Cacher<T>
    where T: Fn(u32) ->u32
{
    calculation: T,
    value: Option<u32>,
}
impl<T> Cacher<T> where T: Fn(u32) ->u32 {
    fn new(calculation: T) ->Cacher<T> {
        Cacher {
            calculation,
            value: None,
        }
    }
    fn value(&mut self, arg: u32) ->u32 {
        match self.value {
            Some(v) => v,
            None => {
                let v = (self.calculation)(arg);
                self.value = Some(v);
                v
            }
        }
    }
}
fn generate_workout(intensity: u32, random_number: u32) {
    let mut expensive_result = Cacher::new(|num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    });
    if intensity < 25 {
        println!(
            "Today, do {} pushups!",
            expensive_result.value(intensity)
        );
        println!(
            "Next, do {} situps",
            expensive_result.value(intensity)
        );
    }
}

fn main() {
    {
        let v1 = vec![1, 2, 3];
        let v1_iter = v1.iter();
        for val in v1_iter {
            println!("Got: {}", val)
        }
    }
    {
        let v1 = vec![1,2,3];
        let v1_2 =  v1.iter().map(|x|x+1);
        let v2: Vec<_> = v1.iter().map(|x|x+1).collect();
        println!("{:?}", v1.iter());
        println!("{:?}", v1_2);
        assert_eq!(v2, vec![2,3,4]);
    }
    {

    }


}



