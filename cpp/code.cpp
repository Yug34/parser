class example {};

class Base {
    public:
    int add(int x, int y) {
        return x + y;
    }
    int sub(int x, int y) {
        return x - y;
    }
};

int main() {
    Base base;
    base.add(1, 2);

    return 0;
}