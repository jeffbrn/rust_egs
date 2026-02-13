#include "test1_helper.h"
#include <iostream>

using namespace std;

int get_n(const Test1 &t)
{
    return t.n;
}

void set_n(Test1 &t, int value)
{
    t.n = value;
}

void dump(rust::Slice<const uint32_t> slice) {
    cout << "Received slice of length " << slice.size() << ": ";
    for (size_t i = 0; i < slice.size(); ++i) {
        cout << slice[i] << ", ";
    }
    cout << endl;
}