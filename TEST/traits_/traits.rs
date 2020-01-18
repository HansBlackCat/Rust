
struct Sheep { naked: bool, name: &'static str }

trait Animal {
    // 'Self' = implementor's type
    fn new(name: &'static str) ->Self;

    fn name(&self) ->&'static str;
    fn noise(&self) ->&'static str;

    // can provide default method 
    fn talk(&self) {
        println!("{} says {}", self.name(), self.noise());
    }
}
impl Sheep {
    fn is_naked(&self) ->bool {
        self.naked
    }
    fn sheer(&mut self) {
        if self.is_naked() {
            println!("{} is already naked...", self.name());
        } else {
            println!("{} gets a haircut!", self.name);
            self.naked = true;
        }
    }
}
impl Animal for Sheep {
    fn new(name: &'static str) ->Sheep {
        Sheep { name: name, naked: false }
    }
    fn name(&self) ->&'static str {
        self.name
    }
    fn noise(&self) ->&'static str {
        if self.is_naked() {
            "baaaaaah?"
        } else {
            "baaaaaaaaaaaaah!"
        }
    }
    fn talk(&self) {
        println!("{} pauses briefly... {}", self.name, self.noise());
    }
}

fn main() {
    let mut dolly: Sheep = Animal::new("Dolly");

    dolly.talk();
    dolly.sheer();
    dolly.talk();
}