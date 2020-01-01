use std::collections::HashMap;
use rand::Rng;

fn main() {
    let v: Vec<i32> = Vec::new();
    let v_macro = vec![1, 2, 3];

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
        let v = vec![1, 2, 3, 4, 5];
        let unexisting_factor1 = v[3];
        // let unexisting_factor1 = &v[100];
        let unexisting_factor2 = v.get(100);
        println!("{:?}", v)
    }
    {
        let v = vec![100, 32, 57];
        for i in &v { // &v or v both OK
            println!("{}", i);
        }
        println!("{:?}", v); // if v then Error
    }
    {
        let mut v = vec![100, 30, 20];
        for i in &mut v {
            *i /= 10;
        }
        println!("{:?}", v);
    }
    // << New String >>
    {
        let s = String::new();
        let data = "initial value";
        let s = data.to_string();
        let s_alter = "initial value".to_string();
    }
    {
        let mut hello = String::from("안녕하시오");
        let world = " world!";
        hello.push_str(world);
        println!("{}", hello);

        let k = "Key".to_string();
        let s = " Super".to_string();
        let mu = k + &s;
        println!("{}", mu);
    }
    {
        let s1 = String::from("tik");
        let s2 = String::from("tak");
        let s3 = String::from("tok");
        println!("{}", format!("{}-{}-{}", s1, s2, s3));
    }
    {
        let s = String::from("안뇽");
        let s_slice = &s[0..3];
        println!("{}", &s.len());
        println!("{}", s_slice);
        for i in "नमस्ते".chars() {
            print!("{} ", i);
        }
        println!();
        for i in "안뇽".bytes() {
            print!("{} ", i);
        }
        println!();
    }
    // << Hash >>
    {
        let mut scores = HashMap::new();
        scores.insert(String::from("Blue"), 10);
        println!("{:?}", scores);
    }
    {
        let teams = vec![String::from("Blue"), String::from("Yello")];
        let initial_scores = vec![10, 50];
        let scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();
    }
    {
        let field_name = String::from("Favorite color");
        let field_value = String::from("Blue");
        let mut map = HashMap::new();
        map.insert(field_name, field_value);
    }
    {
        let mut scores = HashMap::new();
        scores.insert(String::from("Blue"), 10);
        scores.insert(String::from("Yellow"), 50);

        let team_name = String::from("Blue");
        let score = scores.get(&team_name);
        println!("{:?}", score);

        for (sc, va) in &scores {
            println!("{} - {}", sc, va);
        }
    }
    {
        let mut scores = HashMap::new();
        scores.insert(String::from("Blue"), 10);
        scores.insert(String::from("Yellow"), 50);

        scores.insert(String::from("Blue"), 30);
        scores.entry(String::from("Yellow")).or_insert(30);
        println!("{:?}", scores);
        println!("{:?}", scores.entry(String::from("Blue")));
    }
    {
        let text = "hello rust howdy haskell hello";
        let mut map = HashMap::new();

        for word in text.split_whitespace() {
            let count = map.entry(word).or_insert(0);
            *count += 1;
        }
        println!("{:?}", map);
    }
    {
        // let rng_gen = rand::thread_rng().gen_range(1, 101);
        let mut rng_vec = Vec::new();
        for i in 0..100 {
            rng_vec.push(rand::thread_rng().gen_range(1, 101));
        }

        let mut rng_mean = 0;
        for i in 0..rng_vec.len() {
            rng_mean += rng_vec[i];
        }
        let rng_mean: f64 = rng_mean as f64 / rng_vec.len() as f64;

        let mut rng_median = 0;
        let sorted_rng_vec = rng_vec.to_owned();
        rng_median = rng_vec[rng_vec.len() / 2];

        let mut rng_mode = 0;
        let copied_rng_vec = rng_vec.to_owned();
        let mut map = HashMap::new();
        for i in copied_rng_vec {
            let count = map.entry(i).or_insert(0);
            *count += 1;
        }
        let mode = map.into_iter().max_by_key(|&(_, key)| key).map(|(val, _)| val).expect("Error");


        println!("{:?}", rng_vec);
        println!("RNG_mean: {}", rng_mean);
        println!("RNG_median: {}", rng_median);
        println!("RNG_mode: {}", mode);
    }
}