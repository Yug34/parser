struct aaa {
    int x;
};

union bbb {
    int random1;
    char random2;
    float random3;
};

class foo {
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

class Base: public foo, public bar {
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
    void arrayFunc(char charArray[]) {
        return;
    }
    void intArrayFunc(int intArray[]) {
        return;
    }
};

int main() {
    Base base;
    base.add(1, 2);
    base.sub(2, 1);

    return 0;
}