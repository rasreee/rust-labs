# Exercise 4: Output Redirection

**Note: Make sure you complete Exercise 3 before starting this part.**

While input redirection is useful, output redirection is perhaps more so. The program `primes.rs` is designed to print out all the prime numbers less than 1,000,000. Unfortunately, there may be a few bugs… >:)

In order to use our output later, we need to send our output to a file, instead of println-ing to the terminal. At the same time, we may also want to send some debugging information to the console. The function `format!` comes in handy for this, as it allows you to print to different datastreams. In particular, most programs have access to two different output streams: stdout (Standard Output), and stderr (Standard Error). By default, both stdout and stderr send their output to terminal, but with redirection, the two outputs can be sent to different files. (Note: `println!` has the same behavior as `format!` to stdout)

## Action Items

1. Debug and add functionality to the code in primes.rs so that the following behavior is achieved when you run the command in 1:
   - All prime numbers less than 1,000,000 are outputted to the specified file, one per line.
   - The program prints to terminal status messages of the form n% complete, where n goes from 1 to 99, after every percent complete (Note that we do not expect a “100% complete” message). The exact timing of this doesn’t matter, as long as a correct status message is sent at about the right time.
   - After the program runs, the program prints to terminal the total number of prime numbers found (Hint: There should be 78,498 prime numbers), with the line “Total primes found: “.
   - Each status message sent to terminal is on its own line, and there are no trailing whitespaces.
2. Run your code at the end to print out your results in primenumbers.txt. **Note:** this will take a few minutes to run :") so this would be a good time to get up, go hydrate, and #selfcare until it's finished running.

**NOTE:** Before actually fixing the bugs in the code, it's a good idea to first set up the **reporting**, then use that to debug your code. To compile & run your code, use

```/bin/bash
$ rustc primes.rs
$ ./primes
```

To print your output to the file

```/bin/bash
$ ./primes > primenumbers.txt
```
