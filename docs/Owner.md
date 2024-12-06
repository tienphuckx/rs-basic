# Owner

- Each variable or memory area must have a single owner.
- At any given time, a variable or memory area can only have one owner.
- In special cases, a variable or memory area may have multiple owners (e.g., through Rc or Arc).
- When the owner of a variable or memory area goes out of scope, the variable or memory area is destroyed (deallocated).

# Scope of variables
- Variables are Valid Within Their Declared Scope
    + A variable is accessible only within the block where it is defined.
    + Scope starts from the point of declaration and ends at the closing brace of the block.

    fn main() {
    {
        let x = 10; // `x` is valid from this point
        println!("x: {}", x); // OK
    }
        // println!("x: {}", x); // Error: `x` is not accessible here
    }


- Ownership and Scope
    + Ownership of a variable determines when its memory is deallocated.
    + When a variable goes out of scope, Rust automatically drops it, releasing its resources.

    fn main() {
        let s = String::from("hello"); // `s` owns the heap memory
        // `s` is valid here
    } 
    // `s` goes out of scope and its memory is freed


