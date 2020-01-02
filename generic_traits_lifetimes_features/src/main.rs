use generic_traits_lifetimes_features::{Tweet, Summary, NewArticle, notify};


struct Point<T> {
    x: T,
    y: T,
}
struct PointAdvanced<T,U> {
    x: T,
    y: U,
}

impl<T> Point<T> {
    fn fst(&self) ->&T {
        &self.x
    }
}
impl Point<f32> {
    fn distance_from_origin(&self) ->f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}
impl<T, U> PointAdvanced<T, U> {
    fn mixup<V, W>(self, other: PointAdvanced<V,W>) ->PointAdvanced<T, W> {
        PointAdvanced {
            x: self.x,
            y: other.y,
        }
    }
}


fn main() {
    /*
    let number_list = vec![34, 50, 25, 100, 65];
    let mut largest = number_list[0];

    for number in number_list {
        if number > largest {
            largest = number;
        }
    }
    println!("The largest number is {}", largest);

    let number_list = vec![102, 32, 6000, 89, 54, 2, 43, 8];
    let mut largest = number_list[0];

    for number in number_list {
        if number > largest {
            largest = number;
        }
    }
    println!("The largest number is {}", largest);
    */ // Long and tedious
    let number_list = vec![34, 50, 25, 100, 65];
    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let number_list = vec![340, 50, 25, 100, 65];
    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let integer = Point { x:5, y:7 };
    let float = Point { x: 3.4, y: 9.2};
    // let multi_ty = Point { x:3, y:8.6};
    let multi_ty = PointAdvanced { x:3, y:8.6};

    let p = Point { x:5.4, y:8.9 };
    println!("You know you? {}", p.fst());
    println!("Distance from Origin: {}", p.distance_from_origin());

    let p1 = PointAdvanced { x:3, y: 8 };
    let p2 = PointAdvanced { x:"Je", y:"Ju" };
    let p3 = p1.mixup(p2);
    println!("p3.x: {}, p3.y:{}", p3.x, p3.y);

    let integer = Some(5);
    let float = Some(3.6);

    // << Traits >>
    let tweet = Tweet {
        username: String::from("KanKan"),
        content: String::from("About Austro-Hungary"),
        reply: false,
        retweet: false,
    };
    println!("\n1 new tweet: {}", tweet.summarize());

    let article = NewArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from("The Pittsburgh Penguins once again are the best hockey team in the NHL."),
    };

    println!("New article available! {}", article.summarize());

    println!("Print author: {}", tweet.summarize_author());
    println!("Print author: {}", article.summarize_author());

    notify(tweet);


}

fn largest(list: &[i32]) ->i32 {
    let mut largest = list[0];
    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }
    largest
}
fn largest_generics<T: PartialOrd + Copy>(list: &[T]) ->T {
    let mut largest = list[0];
    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }
    largest
}


