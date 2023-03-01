pub trait Animal {
    fn speak(&self);
    fn name(&self) -> String;
}

pub struct Dog {
    name: String,
}

impl Dog {
    pub fn new(name: String) -> Dog {
        Dog { name }
    }
}

impl Animal for Dog {
    fn speak(&self) {
        println!("{} says woof!", self.name);
    }

    fn name(&self) -> String {
        self.name.clone()
    }
}

pub struct Cat {
    name: String,
}

impl Cat {
    pub fn new(name: String) -> Cat {
        Cat { name }
    }
}

impl Animal for Cat {
    fn speak(&self) {
        println!("{} says meow!", self.name);
    }

    fn name(&self) -> String {
        self.name.clone()
    }
}

pub enum AnimalChoice {
    Dog,
    Cat,
}

pub enum AnimalType {
    Dog(Dog),
    Cat(Cat),
}

pub fn speak<T: Animal>(t: T) {
    t.speak();
}

pub fn create_animal(animal_type: AnimalChoice, name: String) -> AnimalType {
    match animal_type {
        AnimalChoice::Dog => AnimalType::Dog(Dog::new(name)),
        AnimalChoice::Cat => AnimalType::Cat(Cat::new(name)),
    }
}

pub fn test_poly() {
    let dog = Dog::new("Fido".to_string());
    let cat = Cat::new("Felix".to_string());
    speak(dog);
    speak(cat);

    let pooch = create_animal(AnimalChoice::Dog, "Fido".to_string());
    let pussy = create_animal(AnimalChoice::Cat, "Felix".to_string());

    // no way to call speak() on dog or cat here except by using a match
    // which makes the polymorphic speak redundant
    for animal in vec![pooch, pussy] {
        match animal {
            AnimalType::Dog(dog) => dog.speak(),
            AnimalType::Cat(cat) => cat.speak(),
        }
    }
}
