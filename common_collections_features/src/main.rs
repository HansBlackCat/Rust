fn main() {
    let v:Vec<i32> = Vec::new();
    let v_macro = vec![1,2,3];

    {
        let mut v_for_push = vec![];
        v_for_push.push(5);
        v_for_push.push(6);
        v_for_push.push(12);
    } // dropped

    {
        let v = vec![1, 2, 3, 4, 5];
        let third: &i32 = &v[2];
        println!("The third element is {}", third);
        match v.get(2) {
            Some(third) => println!("Maybe third : {}", third),
            None => ()
        }
    }

    {
        let v = vec![1,2,3,4,5];
        // let unexisting_factor1 = &v[100];
        let unexisting_factor2 = v.get(100);
    }




}
