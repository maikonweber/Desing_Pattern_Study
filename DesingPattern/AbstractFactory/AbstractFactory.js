class AnimalFactory {
    createAnimal() {
        throw new Error('This method should be overrriden');
    }
}

class DogFactory extends AnimalFactory {
    createAnimal() {
        return new Dog();
    }
}

class CatFactory extends AnimalFactory {
    createAnimal() {
        return new Cat();
    }
}

class Animal {
   makeSound() {
    throw new Error('This method should be overridden');
    }
}


class Dog extends Animal {
  makeSound() {
    return "Woof!";
  }
}

// Concrete Product 2
class Cat extends Animal {
  makeSound() {
    return "Meow!";
  }
}

// Usage 
const dogFactory = new DogFactory();
const dog = dogFactory.createAnimal();
console.log(dog.makeSound()); // Output: "Woof!"

const catFactory = new CatFactory();
const cat = catFactory.createAnimal();
console.log(cat.makeSound()); // Output: "