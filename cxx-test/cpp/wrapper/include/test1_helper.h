#pragma once

/*
 * Helper functions to access Test1 properties that are not
 * directly accessible via autocxx.
 */

#include "test.hpp"
#include "cxx.h"

int get_n(const Test1 &t);
void set_n(Test1 &t, int value);

void dump(rust::Slice<const uint32_t> slice);
