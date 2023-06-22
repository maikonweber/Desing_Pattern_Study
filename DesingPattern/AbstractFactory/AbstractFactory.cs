// Abstract Factory
public abstract class AnimalFactory
{
    public abstract Animal CreateAnimal();
}

// Concrete Factory 1
public class DogFactory : AnimalFactory
{
    public override Animal CreateAnimal()
    {
        return new Dog();
    }
}

// Concrete Factory 2
public class CatFactory : AnimalFactory
{
    public override Animal CreateAnimal()
    {
        return new Cat();
    }
}

// Abstract Product
public abstract class Animal
{
    public abstract string MakeSound();
}

// Concrete Product 1
public class Dog : Animal
{
    public override string MakeSound()
    {
        return "Woof!";
    }
}

// Concrete Product 2
public class Cat : Animal
{
    public override string MakeSound()
    {
        return "Meow!";
    }
}

// Usage
class Program
{
    static void Main(string[] args)
    {
        AnimalFactory dogFactory = new DogFactory();
        Animal dog = dogFactory.CreateAnimal();
        Console.WriteLine(dog.MakeSound()); // Output: "Woof!"

        AnimalFactory catFactory = new CatFactory();
        Animal cat = catFactory.CreateAnimal();
        Console.WriteLine(cat.MakeSound()); // Output: "Meow!"
    }
}
