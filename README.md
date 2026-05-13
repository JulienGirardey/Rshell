# Rshell

## Why choose Rust ?

Rust provides fine-grained control of memory and other resources.<br>
Rust's ownership model makes concurrent programming more manageable and less prone to bugs.<br>
Modern Tooling: Cargo, Rust's package manager and build system, is highly praised for its ease of use.<br>

## Hello, world

To beginning we will create a new file with the ".rs" ending.<br>
    Exemple: hello_world.rs<rb>
After that we will create a new main function witn our print.<br>
With Rust we define a new function with "fn".<br>
    Exemple: fn main () {}<br>
Like C langage, all program need a main function to be compiled.<br>

Here a list of possible print:
    <ul>
        <li>format!: write formatted text to String</li>
        <li>print!: same as format! but the text is printed to the console (io::stdout).</li>
        <li>println!: same as print! but a newline is appended.</li>
        <li>eprint!: same as print! but the text is printed to the standard error (io::stderr).</li>
        <li>eprintln!: same as eprint! but a newline is appended.</li>
    </ul>

For this exemple we will use "println!()"<br>

## Librairy and Binary

Binary crate is an exec program like "main.rs"
    <ul>
        <li>Binary = exec (with main), run by user</li>
        <li>Library = reusable code, not runnable</li>
    </ul>

## macro better than function ?

Macro is more complex than function but,
it can accept an undefine number of argument,
and he's check at the runtime,
it can handle the code