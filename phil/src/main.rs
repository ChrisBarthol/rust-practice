struct Philosopher {
    name:  String,
}

impl Philosopher {
    fn new(name: &str) -> Philosopher {
        Philosopher {
            name: name.to_string(),
        }
    }
}

struct Teacher {
    name: String,
    age: f32,
}

impl Teacher {
    fn new(name: &str, age: &str) -> Teacher {
        Teacher {
            name: name.to_string(),
            age: age.parse::<f32>().unwrap(),
        }
    }
}

fn main() {
    let p1 = Philosopher::new("Judith Butler");
    let p2 = Philosopher::new("Gilles Deleuze");
    let p3 = Philosopher::new("Karl Marx");
    let p4 = Philosopher::new("Emma Goldman");
    let p5 = Philosopher::new("Michel Foucault");

    let t1 = Teacher::new("Adam Smith", "32");
    println!("{}, {}, {}, {}, {}", p1.name, p2.name, p3.name, p4.name, p5.name);
    println!("{} is {}", t1.name, t1.age);
}
