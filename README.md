# Writing an Interpreter

This is a project for learning about the parts of an interpreter,
and then maybe some parts of compilers. It is inspired by, and will be
based -- at least in part -- on the excellent book *Crafting an Interpreter*
by Robert Nystrom. We will follow and mention other good sources as
we go, too.

Here are a few goals we have while starting out:

+ To understand enough to write our own parser, including lexing, etc.
+ To be able to dig into and understand the source code of CPython.
+ Same for the Zend (PHP) engine.
+ To potentially contribute to these projects, and also to Typst.

So, we want to dip our toes into languages and compilers, starting with
parsers and their antecedents. It should be a fun journey!

Granted, these goals are ambitious and time is limited -- and we have
lots of other interests. So we may never get there for all of them. But,
hey, it's nice and motivating to have goals, and we're sure to learn from
any work we do on this.

## Progress

So far we have implemented a basic version of the scanner, supporting
single-character tokens, using the `FileUtf8Reader` described below. 
We will start by implementing a Rust version of his parser from the book.
But then we will make our own language with some of its own bells and 
whistles, and for that we will modify the parser, including the scanner 
part.

## Some code ideas

__Lazy file char iterator:__

The module [`src/parser/file_utf8_reader.rs`](interpreter/src/parser/file_utf8_reader.rs) has a
struct type `FileUtf8Reader` that implements the `Iterator` trait. It is a
wrapper around `BufReader<File>`, and its iterator only reads enough bytes
from the file to read the next utf-8 char. This is useful for reading utf-8
chars from large file, potentially containing no linebreaks, without reading
the entire file into memory first.
