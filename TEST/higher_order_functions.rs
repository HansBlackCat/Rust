#![allow(dead_code)]

fn is_odd(n: u32) ->bool {
    n%2 == 1
}

fn foo() ->! {
    panic!("This call never return!");
}

fn some_fn() ->() {
    ()
}

fn main() {
    println!("Find the sum of all squared odd numbers under 1000");
    let upper = 1000;

    let sum_of_square_odd: u32 =
    (0..).map(|x| x^2)
         .take_while(|&n_square| n_square < upper)
         .filter(|&n_square| is_odd(n_square))
         .fold(0, |acc, n_square| acc+n_square);
    println!("Functional Calculate: {}", sum_of_square_odd);

    // let x = panic!("panic!");
    fn some_odd(up_to: u32) ->u32 {
        let mut acc = 0;
        for i in 0..up_to {
            let addition: u32 = match i%2==1 {
                true => i,
                false => continue,
            };
            acc += addition;
        }
        acc
    }
    println!("Sum of odd numbers up to 9: {}", some_odd(9));
}