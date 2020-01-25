use std::collections::{HashMap, HashSet};

fn call(number: &str) ->&str {
    match number {
        "798-1333" => "Num 1",
        "823-1100" => "Num 2",
        _ => "Final",
    }
}

#[derive(PartialEq, Eq, Hash)]
struct Account<'a> {
    username: &'a str,
    password: &'a str,
}
struct AccountInfo<'a> {
    name: &'a str,
    email: &'a str,
}
type Accounts<'a> = HashMap<Account<'a>, AccountInfo<'a>>;

fn try_logon<'a>(accounts: &Accounts, username: &'a str, password: &'a str) {
    println!("Username: {}", username);
    println!("Password: {}", password);
    println!("Attempting logon...");

    let logon = Account {
        username,
        password,
    };

    match accounts.get(&logon) {
        Some(account_info) => {
            println!("Successful logon!");
            println!("Name: {}", account_info.name);
            println!("Email: {}", account_info.email);
        },
        _ => println!("Logon failed"),
    }
}

fn main() {
    let mut contacts = HashMap::new();

    contacts.insert("Kim", "798-1333");
    contacts.insert("Lee", "938-1749");
    contacts.insert("Park", "823-1100");
    contacts.insert("Yoo", "362-4739");

    match contacts.get(&"Kim") {
        Some(&number) => println!("Kim - {}", call(number)),
        _ => println!("Who are you?"),
    }
    match contacts.get(&"Yoo") {
        Some(&number) => println!("Yoo - {}", call(number)),
        _ => println!("Who are you?"),
    }
    contacts.remove(&"Park");
    
    for (contacts, num) in contacts.iter() {
        println!("{} - {}", contacts, num);
    }
    println!("");

    let mut accounts: Accounts = HashMap::new();    

    let account = Account {
        username: "A.F.Kay",
        password: "qwer1234",
    };
    let account_info = AccountInfo {
        name: "Aways.F.Kay",
        email: "AFK6655@email.com",
    };
    accounts.insert(account, account_info);
    try_logon(&accounts, "A.F.Kay", "qwer1234");
    println!("");

    // HashSet
    let mut a: HashSet<i32> = vec![1i32, 2, 3].into_iter().collect();
    let mut b: HashSet<i32> = vec![2i32, 3, 4].into_iter().collect();

    assert!(a.insert(4));
    assert!(a.contains(&4));

    // assert!(b.insert(4), "Value 4 is already in set B!");
    b.insert(5);

    println!("A: {:?}", a);
    println!("B: {:?}", b);

    println!("Union: {:?}", a.union(&b).collect::<Vec<&i32>>());
    println!("Difference: {:?}", a.difference(&b).collect::<Vec<&i32>>());
    println!("Intersection: {:?}", a.intersection(&b).collect::<Vec<&i32>>());
    println!("Symmetric Difference: {:?}", a.symmetric_difference(&b).collect::<Vec<&i32>>());
    
}
