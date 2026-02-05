#pragma once

/*
 * Helper functions to access Test1 properties that are not
 * directly accessible via autocxx.
 */

#include "test.hpp"

int get_n(const Test1 &t);
void set_n(Test1 &t, int value);
