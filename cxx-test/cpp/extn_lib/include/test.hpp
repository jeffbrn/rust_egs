#pragma once

#include <string>

struct Test1 {
	Test1(const std::string &msg_);

	bool method_a();

	std::string get_msg(); // This stops the autocxx parser from treating this object as a POD.

	int n {42}; // because it is not a POD this property is inaccessible to autocxx

private:
	std::string msg {};
};