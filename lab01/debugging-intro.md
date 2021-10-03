# Exercise 2: Debugging in Rust

To finish this exercise, you'll need to install and setup [GDB Project Debugger](https://www.gnu.org/software/gdb/). Refer to [./gdb-setup.md] for help.

For this exercise, you will find the [GDB reference card](./gdb5-refcard.pdf) useful. Compile hello.rs:

```/bin/bash
cargo build --example hello
```

If you can’t run gdb, use lldb - it’s the same thing but with a different interface. Here's a list of command mappings from GDB to LLDB ([GDB to LDB command map](https://lldb.llvm.org/use/map.html)). I have a M1 Macbook Pro, and I couldn't get gdb to work on my machine for Rust. LLDB works perfectly fine especially using VSCode's debugger.

If you have VSCode, install the [CodeLLDB extension](https://marketplace.visualstudio.com/items?itemName=vadimcn.vscode-lldb), which makes debugging with LLDB super easy.

## Action Item

Learning these commands will prove useful for the rest of this lab, and your Rust programming career in general. In the text file named answers.txt, answer the following questions.

While you’re in a gdb/lldb session, how do you set the arguments that will be passed to the program when it’s run?

1. How do you create a breakpoint?
2. How do you execute the next line of Rust code in the program after stopping at a breakpoint?
3. If the next line of code is a function call, you’ll execute the whole function call at once if you use your answer to #3. (If not, consider a different command for #3!) 
4. How do you tell GDB/LLDB that you want to debug the code inside the function (i.e. step into the function) instead? (If you changed your answer to #3, then that answer is most likely now applicable here.)
5. How do you continue the program after stopping at a breakpoint?
6. How can you print the value of a variable (or even an expression like 1+2) in gdb/lldb
7. How do you configure gdb/lldb so it displays the value of a variable after every step?
8. How do you show a list of all local variables and their values in the current function?
9. How do you quit out of gdb/lldb?
