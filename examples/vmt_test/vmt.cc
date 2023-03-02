    #include <iostream>
     
     
    class Animal
    {
    private: 
        int age = 22;
    public:
        virtual int get_age()
        {
            return age;
        }
        virtual int get_height() = 0;
    };
     
    class Dog: public Animal 
    {
    public:
        auto get_height() -> int 
        {
            return 123;
        }
    };
     
     
    auto main() -> int 
    {
        Dog my_dog;
        std::cout << my_dog.get_age() << std::endl;
        std::cout << my_dog.get_height() << std::endl;
        while (true) {
            
        }
    }