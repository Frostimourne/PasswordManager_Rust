# Overview

This is a very simple username and password manager, written in Rust.  It receives username and password inputs from the user, and stores them in a hashmap for quick retrieval.

[Software Demo Video](https://youtu.be/ZPCuWsGKByI)

# Development Environment

This program was written with the assistance of VSCode, using Rust's Cargo system for building, debugging, etc.

As mentioned above, this program is written in Rust.  Being my first foray into Rust, there was a significant learning curve compared to other projects I've done recently due to differences in syntax and code structure.

# Useful Websites

The main reference used for understanding rust was The Rust Programming Language online guide, as well as troubleshooting from Stack Overflow for key issues with syntax and understanding of Tust keywords.

- [The Rust Programming Language](https://doc.rust-lang.org/stable/book/)
- [Stack Overflow](https://stackoverflow.com/)

# Future Work

{Make a list of things that you need to fix, improve, and add in the future.}

- Big #1: There is no persistence to this program.  It does not store the data anywhere, and therefore would need to remain open at all times in order to function.  Some form of encrypted storage would need to be implemented for even basic use
- #2: Protect the passwords.  Implement a username and password requirement in order to request passwords from program, so that someone couldn't just launch the program and access all saved passwords
- #3: Multiple user support. Program could create new files when needed for additional users to store passwords, retrieved with above username and password credentials used to access password manager