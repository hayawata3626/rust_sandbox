trait Animal {
    fn get_name(&self) -> &String;

    fn talk(&self) {
        println!("{} can talk", self.get_name());
    }
}

struct Dog {
    name: String,
    age: usize,
}

impl Animal for Dog {
    fn get_name(&self) -> &String {
        return &self.name;
    }
}

fn main() {
    let dog = Dog {
        name: String::from("Pochi"),
        age: 20,
    };
    println!("{} {}", dog.get_name(), dog.age); 
}
