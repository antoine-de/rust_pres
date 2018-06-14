struct People {
    age: u32,
}

impl People {
    pub fn new() -> Self {
        People { age: 12 }
    }

    pub fn change_age(&mut self, age: u32) {
        self.age = double(age);
    }

    pub fn say_hello(&self) {
        print!("hi, I'm {}", self.age);
    }
}

pub fn double(val: u32) -> u32 {
    val * 2 
}

fn main() {
    let mut bob = People::new();

    bob.change_age(6);

    bob.say_hello();
}
