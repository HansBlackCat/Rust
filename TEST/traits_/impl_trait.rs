use std::iter;
use std::vec::IntoIter;

fn combine_vecs_explicit_return_type<'a>(
    v: Vec<i32>,
    u: Vec<i32>,
) ->iter::Cycle<iter::Chain<IntoIter<i32>, IntoIter<i32>>> {
    v.into_iter().chain(u.into_iter()).cycle()
}

fn combine_vecs<'a>(
    v: Vec<i32>,
    u: Vec<i32>,
) ->impl Iterator<Item=i32> {
    v.into_iter().chain(u.into_iter()).cycle()
}

fn make_addr_function(y: i32) ->impl Fn(i32) ->i32 {
    let closure = move |x: i32| { x + y };
    closure
}

fn double_positives<'a>(numbers: &'a Vec<i32>
) ->impl Iterator<Item=i32> + 'a {
    numbers.iter()
        .filter(|&&x| x>0)
        .map(|x| x*2)
}

fn main() {
    let plus_one = make_addr_function(1);
    assert_eq!(plus_one(2), 3);

    let temp = vec![1,4,9];
    let key = double_positives(&temp);

    
}