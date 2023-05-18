class foo {
    private:
    public:
    protected:
};

class bar {
    private:
    public:
    protected:
};

class baz {
    private:
    public:
    protected:
};

class Base: public foo, private bar, protected baz {
    public:
    int add(int x, int y) {
        return x + y;
    }
    int sub(int x, int y) {
        return x - y;
    }
    private:
    int mul(int x, int y) {
        return x * y;
    }
};

int main() {
    Base base;
    base.add(1, 2);

    return 0;
}