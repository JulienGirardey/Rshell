# Rshell

## Why choose Rust ?

Rust provides fine-grained control of memory and other resources.
Rust's ownership model makes concurrent programming more manageable and less prone to bugs.
Modern Tooling: Cargo, Rust's package manager and build system, is highly praised for its ease of use.

## Hello, world

To beginning we will create a new file with the ".rs" ending.
    Exemple: hello_world.rs
After that we will create a new main function witn our print.
With Rust we define a new function with "fn".
    Exemple: fn main () {}
Like C langage, all program need a main function to be compiled.

Here a list of possible print:
    <ul>
        <li>format!: write formatted text to String</li>
        <li>print!: same as format! but the text is printed to the console (io::stdout).</li>
        <li>println!: same as print! but a newline is appended.</li>
        <li>eprint!: same as print! but the text is printed to the standard error (io::stderr).</li>
        <li>eprintln!: same as eprint! but a newline is appended.</li>
    </ul>

For this exemple we will use "printIn!()"
