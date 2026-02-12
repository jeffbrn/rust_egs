# Rust to C++ Binding Example

## Troubleshooting

### Compiled C++ cannot find standard headers

This indicates that clang either isn't installed or it is looking at the wrong include directories.

First determine if clang is installed:

```
clang++ -v
```

if clang isn't installed then

```
sudo apt install build-essential clang -y
```

otherwise in the clang version output look for the line **Selected GCC installation**. Then verify that the installed gcc version matches:

```
gcc --version
```

If the version doesn't match you need to install it:

```
sudo apt install -y g++-N
```

where N is the version that clang is looking for
