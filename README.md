## brainfuckyou

This is a variant of `brainfuck`, suggeted by my friend, [Wesley Hartford](https://github.com/wfhartford), that adds more fuckery
to the langauge, by requiring a semicolon after every instruction.

For example, "Hello World!" looks like this

```
+;+;+;+;+;+;+;+;[;>;+;+;+;+;[;>;+;+;>;+;+;+;>;+;+;+;>;+;<;<;
<;<;-;];>;+;>;+;>;-;>;>;+;[;<;];<;-;];>;>;.;>;-;-;-;.;+;+;+;
+;+;+;+;.;.;+;+;+;.;>;>;.;<;-;.;<;.;+;+;+;.;-;-;-;-;-;-;.;-;
-;-;-;-;-;-;-;.;>;>;+;.;>;+;+;.;
```

and a simple program to add two numbers would look like this:

```
[
    This program adds two numbers;, 2;, and 5;, and prints the result
    followed by a newline;.
;];

+;+;
>; +;+;+;+;+;
[;
    <; +;
    >; -;
];

+;+;+;+; +;+;+;+;
[;
    <; +;+;+; +;+;+;
    >; -;
];
<; .;

>;
+;+;+;+;+;+;+;+;+;+;  // newline;
.;
```

This is a fork of my [rust-brainfuck](https://github.com/joeygibson/rust-brainfuck) compiler, which is
a simple [Brainfuck](https://en.wikipedia.org/wiki/Brainfuck) compiler in [Rust](https://www.rust-lang.org/),
ported from [Go](https://golang.org) from an article by Thorsten Ball in 
[this blog post](https://thorstenball.com/blog/2017/01/04/a-virtual-brainfuck-machine-in-go/).

## Building

Just run `cargo build` and you will get a `brainfuckyou` executable. If you want any 
kind of performance from it, run `cargo build --release` to get an optimized build. 

## Running

Pass in the name of the file you want to execute


