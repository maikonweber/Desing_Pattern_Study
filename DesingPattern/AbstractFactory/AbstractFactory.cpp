#include <iostream>
using namespace std;

// Abstract Product
class Animal {
public:
    virtual string makeSound() = 0;
};

// Concrete Product 1
class Dog : public Animal {
public:
    string makeSound() override {
        return "Woof!";
    }
};

// Concrete Product 2
class Cat : public Animal {
public:
    string makeSound() override {
        return "Meow!";
    }
};

// Abstract Factory
class AnimalFactory {
public:
    virtual Animal* createAnimal() = 0;
};

// Concrete Factory 1
class DogFactory : public AnimalFactory {
public:
    Animal* createAnimal() override {
        return new Dog();
    }
};

// Concrete Factory 2
class CatFactory : public AnimalFactory {
public:
    Animal* createAnimal() override {
        return new Cat();
    }
};

// Usage
int main() {
    AnimalFactory* dogFactory = new DogFactory();
    Animal* dog = dogFactory->createAnimal();
    cout << dog->makeSound() << endl; // Output: "Woof!"

    AnimalFactory* catFactory = new CatFactory();
    Animal* cat = catFactory->createAnimal();
    cout << cat->makeSound() << endl; // Output: "Meow!"

    delete dog;
    delete dogFactory;
    delete cat;
    delete catFactory;

    return 0;
}
