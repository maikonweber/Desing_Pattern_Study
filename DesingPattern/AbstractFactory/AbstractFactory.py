# Abstract Factory
class AnimalFactory:
    def create_animal(self):
        raise NotImplementedError("This method should be overridden")


# Concrete Factory 1
class DogFactory(AnimalFactory):
    def create_animal(self):
        return Dog()


# Concrete Factory 2
class CatFactory(AnimalFactory):
    def create_animal(self):
        return Cat()


# Abstract Product
class Animal:
    def make_sound(self):
        raise NotImplementedError("This method should be overridden")


# Concrete Product 1
class Dog(Animal):
    def make_sound(self):
        return "Woof!"


# Concrete Product 2
class Cat(Animal):
    def make_sound(self):
        return "Meow!"


# Usage
dog_factory = DogFactory()
dog = dog_factory.create_animal()
print(dog.make_sound())  # Output: "Woof!"

cat_factory = CatFactory()
cat = cat_factory.create_animal()
print(cat.make_sound())  # Output: "Meow!"
