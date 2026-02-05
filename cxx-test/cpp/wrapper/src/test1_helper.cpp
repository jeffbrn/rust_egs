#include "test1_helper.h"

int get_n(const Test1 &t)
{
    return t.n;
}

void set_n(Test1 &t, int value)
{
    t.n = value;
}
