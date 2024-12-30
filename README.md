# Writing an Interpreter

This is a project for learning more practical details about writing an interpreter,
and then maybe some details about writing compilers. To start, it will follow
along with the excellent book *Crafting an Interpreter*
by Robert Nystrom, and will re-implement code from that book in Rust.

Here are a few goals we have while starting out:

+ To understand enough details to write our own parser, including lexing, etc.
+ To help us dig into and understand the source code of CPython.
+ Same for the Zend (PHP) engine.
+ To potentially contribute to these projects, and others (e.g., Typst).

So, we want to dip our toes into the practical details of languages and compilers,
starting with parsers. Our interest in the projects mentioned comes from a long-standing
curiosity about parsers and virtual machines.

Granted, these goals are ambitious and time is limited -- and we have
lots of other interests. So we may never get there for all of them. But
hey, it's nice and motivating to have goals, and we're sure to learn from
any work we do on this. It should be a fun journey!

## Progress

As mentioned above, we are following along with the book *Crafting an Interpreter*,
and we will start by implementing a Rust version of his parser from the book.
(Which several others have also done, a Google search reveals.)
So far we've implemented a basic version of the scanner,
[here](interpreter/src/parser/scanner.rs),
supporting single-character tokens, using the `FileUtf8Reader` described below,
and we've implemented the first version of the parser from chapter 6,
[here](interpreter/src/parser/parser.rs).

We've pulled the fragment of the grammar that uses out into its own file
[here](interpreter/src/parser/grammar.rs), including the Visitor pattern pretty
printer from the book.

Later we plan to make our own language with some of its own bells and
whistles, using Bob's Lox as a starting point. For that we will use our
implementation of his parser and modify it as needed.

## Some code ideas

__Lazy file char iterator:__

The module [`file_utf8_reader.rs`](interpreter/src/parser/file_utf8_reader.rs) has a
struct type `FileUtf8Reader` that implements the `Iterator` trait. It is a
wrapper around `BufReader<File>`, and its iterator only reads enough bytes
from the file to read the next utf-8 char. This is useful for reading utf-8
chars from large file, potentially containing no linebreaks, without reading
the entire file into memory first.

## Sources

As mentioned, to start we will follow Robert Nystrom's *Crafting and Interpreter*.
We will follow and mention other good sources as we go, too. A couple more we have in
mind now are:

+ Nora Sandler's *Writing a C Compiler*
+ The famous Purple Dragon book
