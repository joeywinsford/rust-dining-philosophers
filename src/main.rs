struct Philosopher {
    name: String,
}

impl Philosopher {
    fn new(name: &str) -> Philosopher {
        Philosopher {
            name: name.to_string(),
        }
    }
}


fn main() {
    let soren = Philosopher::new("Soren Kierkegaard");
    let gary = Philosopher::new("Gary Grosvenor");
    let jennifer = Philosopher::new("Jennifer Hotfire");
    let hywel = Philosopher::new("Hywel Price");
    let helen = Philosopher::new("Helen Pringle");

}
