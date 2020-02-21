#![allow(unused_mut, unused_variables, dead_code, non_snake_case, irrefutable_let_patterns, non_snake_case)]

/* << Cargo structure >>
 * ----------------------
 * .
 * ├── Cargo.toml
 * └── src
 *     └── main.rs
 * 1 directory, 2 files
 * ---------------------
 */
//

fn main() {
    // struct & enum
    // struct < enum
    #[derive(Debug)]
    struct UnitStruct {
        kg: i32,
        ib: i32,
    }

    let a = UnitStruct {kg: 64, ib: 12};
    
    impl UnitStruct {
        fn print(&self) {
            println!("{:?}", self)
        }
    }
    a.print();

    #[derive(Debug)]
    enum A {
        AbstractTypeValue,
        ConcreteType(Vec<i32>),
        RGB(i32, i32, i32),
        MyHouse {
            living_room: u32,
            kitchen: u32,
            garage: u32,
        },
        Inherite(UnitStruct),
    }

    let a = A::AbstractTypeValue;
    let b = A::ConcreteType(vec![1,2,3,4]);
    let c = A::RGB(255, 255, 255);
    let d = A::MyHouse {
        living_room: 2,
        kitchen: 1,
        garage: 0,
    };

    impl A {
        fn print(&self) {
            match self {
                A::AbstractTypeValue => println!("It's Abstract! {:?}", self),
                A::ConcreteType(_)   => println!("Concrete! {:?}", self),
                A::RGB(a,b,c)        => println!("RGB! ({}, {}, {})", a, b, c),
                A::MyHouse { .. }    => println!("MyHouse! {:#?}", self),
                _                    => println!("??"),
            }
        }
    }

    a.print();
    c.print();    
}