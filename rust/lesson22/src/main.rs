/*

    Annotation and decorators

    Annotatio/Decators serve to modify code behaviour in a declarative 
    way without explicity changing the core logic of the function or class.
    They often  allow for additional funcationlaity to be "applied" to a 
    class, method or variable. They're all applied at runtime.

*/




/*
            Macros

    In rust, macro are a powerfull feature that allow a metaprogramming by enabling
    the generation a code at compile-time. Macro in rust are similar to functions but differ
    in that they operate at the syntactic level - they generate or transform Rust code before 
    the program is actually compile.


    Key Concept:
        1. Code generation: Macro allow you to define a pattern for generating code.
        This means you can write code that produce other code, reduce redunacy and boilerplate.

        2. MetaProgramming: Rust macros are a form of metaprogramming because they allow  you to 
        write code that writes or manipulate other code. This can be useful for tasks like reducing
        boilerplate, creat domain specific language(DSLs), or automating repetitive patterns.




    Types of macros:
        1. Declerative Macros:
            These are the most common type of macros in Rust. They replace 
            the code written with a different code during compile time.

        2. Procedural Macros:
            These are more complex macros that allow you to define custom 
            behaviour for code generation throught rust code itself.
            Procedural macro operate on rust's abstract syntax tree (AST)
            and are commonly used for things like deriving traits automactially
            or creating custom attribute.



*/



// Defining a Declarative macro
macro_rules! say_hello{
    ()=>{
        println!("Hello");
    }
}


fn main() {
    say_hello!();
    println!("Hello, world!");
}
