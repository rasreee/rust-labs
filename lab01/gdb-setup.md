# GDB Installation and Setup

## MacOS

Use Homebrew to install dependencies

```/bin/bash
brew install gdb
```

**NOTE:** If you’re on macOS, it may be difficult to get gdb working, but you can use lldb instead. The commands differ slightly, but there are great guides out there ([like this one!](https://lldb.llvm.org/use/map.html)) to help you get started.

## Ubuntu/Debian (WSL Ubuntu included)

This section is for Ubuntu/Debian WSL/Linux, not MacOS.

For Ubuntu/Debian WSL/Linux users, install the build-essential package by running

```/bin/bash
sudo apt install build-essential gdb
```
