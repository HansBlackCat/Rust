
#[derive(PartialEq,Debug)]
struct Shoe {
    size: u32,
    style: String,
}
#[derive(PartialEq, Debug)]
struct Counter {
    count: u32,
}
impl Counter {
    fn new() ->Counter {
        Counter{ count: 0}
    }
}
impl Iterator for Counter {
    type Item = u32;
    fn next(&mut self) -> Option<Self::Item> {
        self.count += 1;
        if self.count < 20 {
            Some(self.count)
        } else {
            None
        }
    }
}
#[test]
fn iterator_demonstration() {
    let v1 = vec![1,2,3];
    let mut v1_iter = v1.iter();

    assert_eq!(v1_iter.next(), Some(&1));
    assert_eq!(v1_iter.next(), Some(&2));
    assert_eq!(v1_iter.next(), Some(&3));
    assert_eq!(v1_iter.next(), None);
}
#[test]
fn iterator_sum() {
    let v1 = vec![1,2,3];
    let v1_iter = v1.iter();
    let total: i32 = v1_iter.sum();
    let totalized = [1,2,3].iter().sum();
    assert_eq!(total,totalized);
}
fn shoe_in_my_size(shoe: Vec<Shoe>, shoe_size: u32) ->Vec<Shoe> {
    shoe.into_iter()
        .filter(|s|s.size == shoe_size)
        .collect()
}
#[test]
fn filter_by_size() {
    let shoes = vec![
        Shoe { size: 10, style: String::from("sneaker") },
        Shoe { size: 13, style: String::from("sandal")},
        Shoe { size: 10, style: String::from("boot")},
    ];
    let in_my_size = shoe_in_my_size(shoes, 10);
    assert_eq!(
        in_my_size,
        vec![
            Shoe { size: 10, style: String::from("sneaker") },
            Shoe { size: 10, style: String::from("boot")},
        ]
    );
}
#[test]
fn calling_next() {
    let mut counter = Counter::new();
    assert_eq!(counter.next(), Some(1));
}
#[test]
fn sumor_sumer() {
    let sum: u32 = Counter::new().zip(Counter::new().skip(1))
        .map(|(a,b)|a*b)
        .filter(|x| x%3==0)
        .sum();
    assert_eq!(18, sum);
}