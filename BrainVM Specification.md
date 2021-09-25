# BrainVM Specification
BrainVM project is inspired by the "smallest turing-complete language" Brainf\*ck. The design of Brain Bytecode takes Brainf\*ck as the template, and may add some new features in the future.

BrainVM is free software. Anyone can use it for any purpose.


*But it's not recommended to use it in production, XD*

## 1. Brain Bytecode
---

Brain Bytecode is, to some extent, a expansion of the language Brainf\*ck. Therefore, a comparison table is offered here.

| Brain Bytecode |  Description                      | Brainf\*ck |
| -------------- | --------------------------------- | ---------  |
| `leftmove n`   | pointer move left                 | `<`        |
| `rightmove n`  | pointer move right                | `>`        |
| `add n`        | add n to pointed value            | `+`        |
| `sub n`        | sub n to pointed value            | `-`        |
| `putchar`      | output pointed value as char      | `.`        |
| `getchar`      | input char to pointed value       | `,`        |
| `while`        | loop if record value is not zero  | `[`        |
| `endwhile`     | end loop if record value is zero  | `]`        |
| `putint`       | output pointed value as int       | none       |
| `getint`       | input int to pointed value        | none       |

**ATTENTION**: Note that some instructions are followed with a number `n`, which means , unlike its predecessor, you can move the pointer (or add / sub) by a specified value at one time.

Moreover, the definitions of `while` and `endwhile` are slightly different from Brainf\*ck. BrainVM will record the position of loop condition, while Brainf\*ck always use currently pointed value as loop condition.

## 2. VM arguments
---
BrainVM offers some useful arguments. Before introducing them, I must remind you that `-version` arguments will print the VM version. Other arguments are listed below:

### **1. Initial chunk**
You can specify BrainVM's initial chunk, although it auto-expands when chunk is not enough.
> `bvm -initial 1024`

sets the initial chunk to 1024. Each chunk is actually a int. That's also a difference from Brainf\*ck.

### **2. Allocate threshold**
When the chunk is not enough, BVM allocates automatically.
> `bvm -threshold 1024`

sets the allocate threshold to 1024. Which means, if the chunksize is smaller than 1024, it simply doubles. Otherwise, it is added by specified threshold.

### **3. Translate**
Since the Brain Bytecode is a expansion of its predesessor, Brainf\*ck codes can easily translated to Brain Bytecode.
> `bvm -translate hello.bf`

translates the file `hello.bf` to Brain Bytecode `hello.brain`. Note that because of the slight difference between Brainf\*ck's command `[` / `]` and Brain Bytecode `while` / `endwhile`, translation from Brain Bytecode to Brainf\*ck is hard to implement.

### **4. Optimize**
Translated files are often full of ineffective commands. You can simply optimize them by calling:
> `bvm -optimize hello.bf`

Your own program may also be optimized this way.

### **5. Speedtest**
Speed is usually of significance in programming. When you specified the flag, the VM will output execution time after running the program.
> `bvm -speedtest hello.bf`

is all you need.

*Some flags can be used at one time.*