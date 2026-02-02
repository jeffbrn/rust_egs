#include "test.hpp"

Test1::Test1(const std::string &msg_) : msg(msg_) {}

bool Test1::method_a() {
	return true;
}

std::string Test1::get_msg() {
	return msg;
}
