struct Some_Struct {
    int x;
    double referenced_double;
    char c;
} randomStruct;

class unusedClass {
    public:
    private:
    protected:
};

class foo {
    // TODO in the parser, constructors and destructors:
    public:
        foo() {}
        ~foo() {}
    private:
    protected:
};

// Just some dummy classes, will be inherited in the Base class
class bar {
    public:
    private:
    protected:
};

class baz {
    public:
    private:
    protected:
};

class lorem {
    public:
    private:
    protected:
};

class Base: public foo, public bar, private baz, protected lorem {
    public:
    int add(int x, int y, int z) {
        // Unused argument z won't have the "used" tag in the AST!
        return x + y;
    }
    int sub(int x, int y) {
        return x - y;
    }
    private:
    int mul(int x, int y) {
        return x * y;
    }
    int arrayFunc(char charArray[]) {
        return 0;
    }
};

int main() {
    // Using only these two (add, sub) methods from the Base class, the rest won't show up as "used" in the AST.
    Base base;
    base.add(1, 2, 3);
    base.sub(2, 1);

    // Referencing only the 'd' variable in the struct, it'll be tagged as referenced in the AST
    randomStruct.referenced_double = 1.42;

    return 0;
}