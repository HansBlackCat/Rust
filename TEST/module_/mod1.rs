mod my_mod {
    fn private_fn() {
        println!("Called 'my_mod::private_fn()");
    }
    pub fn function() {
        println!("Called 'my_mod::fn()");
    }
    pub fn indirect_access() {
        println!("Called 'my_mod::indirect_access()");
        private_fn();
    }
    pub mod nested {
        pub fn function() {
            println!("Called 'my_mod::nested::fn()");
        }
        fn private_fn() {
            println!("Called `my_mod::nested::private_fn()");
        }
        // in PATH, must be parent or ancestor
        pub(in crate::my_mod) fn public_fn_in_my_mod() {
            println!("Called `my_mod::nested::public_fn_in_my_mod()");
        }
        // pub(self) == private
        pub(self) fn public_fn_in_nested() {
            println!("Called `my_mod::nested::public_fn_in_nested()");
        }
        // visible only in parent mod
        pub(super) fn public_fn_in_super_mod() {
            println!("Called `my_mod::nested::public_fn_in_super_mod()");
        }
    }
    pub fn call_public_fn_in_my_mod() {
        println!("Called `my_mod::call_public_fn_in_my_mod()");
        nested::public_fn_in_my_mod();
        println!("> ");
        nested::public_fn_in_super_mod();
    }
    // Only visible in current crate
    pub(crate) fn public_function_in_crate() {
        println!("Call `my_mod::public_function_in_crate()");
    }
    // Mod private
    mod private_nested {
        pub fn funtion() {
            println!("Call `my_mod::private_nested::fn()");
        }
        pub(crate) fn restricted_fn() {
            println!("Call `my_mod::private_nested::restricted_fn()");
        }
    }
}
fn function() {
    println!("Call `function()");
}

fn main() {

}