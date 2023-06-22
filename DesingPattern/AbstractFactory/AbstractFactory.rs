// Abstract Factory
trait AnimalFactory {
    fn create_animal(&self) -> Box<dyn Animal>;
}

// Concrete Factory 1
struct DogFactory;

impl AnimalFactory for DogFactory {
    fn create_animal(&self) -> Box<dyn Animal> {
        Box::new(Dog)
    }
}

// Concrete Factory 2
struct CatFactory;

impl AnimalFactory for CatFactory {
    fn create_animal(&self) -> Box<dyn Animal> {
        Box::new(Cat)
    }
}

// Abstract Product
trait Animal {
    fn make_sound(&self) -> &str;
}

// Concrete Product 1
struct Dog;

impl Animal for Dog {
    fn make_sound(&self) -> &str {
        "Woof!"
    }
}

// Concrete Product 2
struct Cat;

impl Animal for Cat {
    fn make_sound(&self) -> &str {
        "Meow!"
    }
}

// Usage
fn main() {
    let dog_factory: Box<dyn AnimalFactory> = Box::new(DogFactory);
    let dog: Box<dyn Animal> = dog_factory.create_animal();
    println!("{}", dog.make_sound()); // Output: "Woof!"

    let cat_factory: Box<dyn AnimalFactory> = Box::new(CatFactory);
    let cat: Box<dyn Animal> = cat_factory.create_animal();
    println!("{}", cat.make_sound()); // Output: "Meow!"
}