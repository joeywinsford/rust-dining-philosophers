struct Philosopher {
    name: String,
}

impl Philosopher {
    fn new(name: &str) -> Philosopher {
        Philosopher {
            name: name.to_string(),
        }
    }
    fn eat(&self) {
        println!("{} is done eating", self.name);
    }
}


fn main() {
    let philosophers = vec![
        Philosopher::new("Soren Kierkegaard"),
        Philosopher::new("Gary Grosvenor"),
        Philosopher::new("Jennifer Hotfire"),
        Philosopher::new("Hywel Price"),
        Philosopher::new("Helen Pringle"),
    ];

    for philosopher in &philosophers {
        philosopher.eat();
    }
}
