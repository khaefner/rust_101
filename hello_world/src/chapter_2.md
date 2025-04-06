## Chapter 2: Variables and Mutability: Assigning Crew and Adapting Systems

Just as a starship relies on assigning crew members to specific stations and adapting its systems to various situations, our Rust programs rely on variables to store and manipulate data. In this chapter, we'll explore how to declare variables and understand Rust's approach to mutability – the ability of a variable's value to change.

### Declaring Variables with `let`: Assigning Crew to Stations

In Rust, we use the `let` keyword to declare a variable. Think of this as assigning a specific crew member to a particular station on the starship. Once assigned, that crew member (the value) generally stays at their post unless explicitly reassigned.

Here's a simple example:

```rust, editable
fn main() {
    let starship_name = "USS Enterprise";
    println!("The name of the flagship is: {}", starship_name);
}
```





In this code, `let` indicates the start of a variable declaration. `starship_name` is the identifier we've chosen for our variable, much like the designation of a starship. The `=` operator assigns the string literal "USS Enterprise" to this variable. Finally, `println!` is a macro that prints the value of `starship_name` to the console. The `{}` is a placeholder that gets replaced by the value of the variable provided after the comma.

### Type Inference: The Computer Knows Best

In many cases, Rust is smart enough to figure out the type of data you're assigning to a variable. This is called type inference. In our previous example, Rust can infer that `"USS Enterprise"` is a string.

However, you can also explicitly specify the type of a variable during declaration:
Rust

```rust, editable
fn main() {
    let warp_factor: f64 = 9.975;
    println!("Engaging at warp factor: {}", warp_factor);
}
```

Here, : `f64` after the variable name specifies that `warp_factor` will hold a 64-bit floating-point number. Specifying the type can be useful for clarity or when Rust can't infer the type on its own.

### Immutability by Default: Starfleet Directives

One of Rust's core principles is immutability by default. This means that once you assign a value to a variable using `let`, you cannot change that value later in your program, unless you explicitly allow it. Think of this as a Starfleet directive – once a system is set, it generally remains that way for stability and predictability.

Let's see what happens if we try to change the value of starship_name from our first example:
Rust

```rust, editable
fn main() {
    let starship_name = "USS Enterprise";
    // starship_name = "USS Voyager"; // This will cause a compile-time error!
    println!("The name of the flagship is: {}", starship_name);
}
```

If you uncomment the line starship_name = "USS Voyager"; and try to compile this code, the Rust compiler will issue an error message. This error tells you that you cannot assign a new value to an immutable variable. This design choice in Rust helps prevent unexpected side effects and makes your code easier to reason about.
Making Variables Mutable with mut: Reassigning Crew as Needed

Of course, there are times when you need a variable's value to change. In such cases, you can use the mut keyword when declaring the variable. This is like getting authorization to reassign a crew member to a different station when the situation demands it.

Here's how you can declare a mutable variable:
Rust

```rust, editable
fn main() {
    let mut shield_strength = 50;
    println!("Initial shield strength: {}", shield_strength);

    shield_strength = 80;
    println!("Shield strength after reinforcement: {}", shield_strength);
}
```

In this example, let mut indicates that shield_strength is a mutable variable. We first assign it the integer value 50. Later in the main function, we can reassign it to the value 80 without any compiler errors. Remember to use mutability judiciously, favoring immutability for safer and more predictable code.

### Shadowing: Temporarily Redefining a Designation

Rust allows you to declare a new variable with the same name as a previous variable. This is called shadowing. The new variable "shadows" the previous 1  one within its scope. Think of this as temporarily redefining a designation for a specific task.  


Here's an example:
Rust

```rust, editable
fn main() {
    let photon_torpedoes = 50;
    println!("Initial torpedo count: {}", photon_torpedoes);

    let photon_torpedoes = photon_torpedoes - 2; // Shadowing the previous variable
    println!("Torpedo count after firing: {}", photon_torpedoes);

    let photon_torpedoes = "Recharging"; // Shadowing with a different type!
    println!("Torpedo status: {}", photon_torpedoes);
}
```

In this code, we first declare photon_torpedoes and initialize it to 50. Then, we use let photon_torpedoes again to declare a new variable with the same name. This new photon_torpedoes takes the value of the original photon_torpedoes minus 2. The first photon_torpedoes is now shadowed. Interestingly, we can even shadow a variable with a different type, as demonstrated in the last declaration where photon_torpedoes is reassigned to the string "Recharging". Shadowing is distinct from mutability; it creates a new variable rather than modifying an existing one.

### Constants: Immutable Laws of the Universe

Besides regular variables, Rust also has constants. Constants are values that are bound to a name and are not allowed to be mutated. Unlike variables declared with let, you must annotate the type of a constant. Constants are declared using the const keyword. Think of these as the fundamental, unchanging laws of the universe within your program.
Rust

```rust, editable
const SPEED_OF_LIGHT: u32 = 299792458; // Type annotation is required
fn main() {
    println!("The speed of light is {} meters per second.", SPEED_OF_LIGHT);
}
```

Here, const SPEED_OF_LIGHT: u32 = 299_792_458; declares a constant named SPEED_OF_LIGHT of type u32 (an unsigned 32-bit integer) and assigns it the value 299,792,458. Constants are typically declared in uppercase with underscores between words for readability and are often used for values that will never change throughout the program's execution.
Starfleet Analogy Recap

 -   let: Assigning a crew member to a station.
 -   Immutability by Default: Starfleet directives ensuring system stability.
 -   mut: Getting authorization to reassign a crew member when necessary.
 -   Shadowing: Temporarily redefining a designation for a specific task.
 -   const: The fundamental, unchanging laws governing the universe (or your program).
