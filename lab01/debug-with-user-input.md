# Exercise 3: Debugging User Input

Let’s see what happens if your program requires user input and you try to run GDB/LLDB on it. First, run the program defined by interactive-hello.rs to talk to an overly friendly program.

```/bin/bash
$ rustc interactive_hello.rs
$ ./interactive_hello
```

Now, we’re going to try to debug it (even though there really are no bugs).

```/bin/bash
gdb target/debug/examples/interactive_hello
```

OR

```/bin/bash
lldb target/debug/examples/interactive_hello
```

What happens when you try to run the program to completion? 

We’ll be learning about a tool to help us avoid this situation. The purpose of this exercise is to make you unafraid of running the debugger even when your program needs user input. 

As it turns out, you can send text to stdin, the file stream read by the `stdin` in this program, with some special characters right from the command line.

See if you can figure out how to send some input to the program without explicitly providing it while it’s running (which, you might have realized, gets you stuck in GDB/LLDB).

**Hint 1:** If you’re creating a text file containing your input, you’re on the right track 🥺.

**Hint 2:** You can run things with command line args (including the redirection symbols) from GDB/LLDB, as well.

Hopefully you’ll appreciate how redirection helps you avoid that annoying situation with GDB/LLDB. Don’t be afraid to use the debugger! It probably feels crusty to use right now, but it's only here to help.
