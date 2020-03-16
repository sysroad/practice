#include <iostream>
#include <random>
#include <vector>
#include <algorithm>

int main(void) {
    std::cout << "hello world!" << std::endl;

    std::vector<long> v;

    for (int i = 0; i < 10; i++)
    {
        v.push_back(random() % 101);
    }

    std::sort(v.begin(), v.end(), [](int a, int b) -> bool {
        return a < b;
    });

    auto iter = v.begin();

    while (iter != v.end())
    {
        std::cout << *iter << " ";

        iter ++;
    }
    std::cout << std::endl;

    return 0;
}