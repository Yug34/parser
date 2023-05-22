struct Some_Struct {
    int x;
    double d;
    char c;
} randomStruct;

class foo {
    // TODO in the parser:
    public:
        foo() {}
        ~foo() {}
    private:
    protected:
};

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
    Base base;
    base.add(1, 2);
    base.sub(2, 1);

    randomStruct.d = 1.42;

    return 0;
}