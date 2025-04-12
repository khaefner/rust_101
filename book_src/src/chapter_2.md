
<link rel="stylesheet" href="star.css">


# Chapter 2: Variables and Mutability: 
## Assigning Crew and Adapting Systems
![logo](Line_Header_Star_Trek.png) 

Just as a starship relies on assigning crew members to specific stations and adapting its systems to various situations, our Rust programs rely on variables to store and manipulate data. In this chapter, we'll explore how to declare variables and understand Rust's approach to mutability – the ability of a variable's value to change.

### ![logo](Star_Trek_icon.png) Declaring Variables with `let`: Assigning Crew to Stations

In Rust, we use the `let` keyword to declare a variable. Think of this as assigning a specific crew member to a particular station on the starship. Once assigned, that crew member (the value) generally stays at their post unless explicitly reassigned.

Here's a simple example:

```rust, editable
fn main() {
    let starship_name = "USS Enterprise";
    println!("The name of the flagship is: {}", starship_name);
}
```
This is also valid:

```rust, editable
fn main() {
    let starship_name = "USS Enterprise";
    println!("The name of the flagship is: {starship_name}");
}
```



In this code, `let` indicates the start of a variable declaration. `starship_name` is the identifier we've chosen for our variable, much like the designation of a starship. The `=` operator assigns the string literal "USS Enterprise" to this variable. Finally, `println!` is a macro that prints the value of `starship_name` to the console. The `{}` is a placeholder that gets replaced by the value of the variable provided after the comma.

### ![logo](Star_Trek_icon.png) Type Inference: The Computer Knows Best

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

### ![logo](Star_Trek_icon.png) Immutability by Default: Starfleet Directives

One of Rust's core principles is immutability by default. This means that once you assign a value to a variable using `let`, you cannot change that value later in your program, unless you explicitly allow it. Think of this as a Starfleet directive – once a system is set, it generally remains that way for stability and predictability.

Let's see what happens if we try to change the value of starship_name from our first example:


```rust, editable
fn main() {
    let starship_name = "USS Enterprise";
    // starship_name = "USS Voyager"; // This will cause a compile-time error!
    println!("The name of the flagship is: {}", starship_name);
}
```

If you uncomment the line `starship_name = "USS Voyager"`; and try to compile this code, the Rust compiler will issue an error message. This error tells you that you cannot assign a new value to an immutable variable. This design choice in Rust helps prevent unexpected side effects and makes your code easier to reason about.


### ![logo](Star_Trek_icon.png) Making Variables Mutable with mut:  Reassigning Crew as Needed

Of course, there are times when you need a variable's value to change. In such cases, you can use the mut keyword when declaring the variable. This is like getting authorization to reassign a crew member to a different station when the situation demands it.

Here's how you can declare a mutable variable:


```rust, editable
fn main() {
    let mut shield_strength = 50;
    println!("Initial shield strength: {}", shield_strength);

    shield_strength = 80;
    println!("Shield strength after reinforcement: {}", shield_strength);
}
```

In this example, let mut indicates that `shield_strength` is a mutable variable. We first assign it the integer value 50. Later in the main function, we can reassign it to the value 80 without any compiler errors. Remember to use mutability judiciously, favoring immutability for safer and more predictable code.

### ![logo](Star_Trek_icon.png) Shadowing: Temporarily Redefining a Designation

Rust allows you to declare a new variable with the same name as a previous variable. This is called shadowing. The new variable "shadows" the previous 1  one within its scope. Think of this as temporarily redefining a designation for a specific task.  


Here's an example:


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

In this code, we first declare `photon_torpedoes` and initialize it to 50. Then, we use `let photon_torpedoes` again to declare a new variable with the same name. This new `photon_torpedoes` takes the value of the original `photon_torpedoes` minus 2. The first `photon_torpedoes` is now shadowed. Interestingly, we can even shadow a variable with a different type, as demonstrated in the last declaration where `photon_torpedoes` is reassigned to the string "Recharging". Shadowing is distinct from mutability; it creates a new variable rather than modifying an existing one.

<div class="warning-block">
  <img src="Yellow_Alert_Icon.png" alt="Yellow Alert Icon" class="warning-icon">
  <p class="warning-text">
    Be careful with shadowing, reassigning variables to different types may make the code harder to read.  Also it obsfucates the intention of mutability.
  </p>
</div>

<details class="discovery-details">
  <summary class="discovery-summary">
    <img src="info.png" alt="Star Trek Cadet" class="info-closed">
    <img src="info.png" alt="" class="info-open">
    Data Types
  </summary>
  <div class="discovery-content">

#### Integers
---
| Length  | Signed  | Unsigned |
| ------- | ------- | -------- |
| 8-bit   | `i8`    | `u8`     |
| 16-bit  | `i16`   | `u16`    |
| 32-bit  | `i32`   | `u32`    |
| 64-bit  | `i64`   | `u64`    |
| 128-bit | `i128`  | `u128`   |
| arch    | `isize` | `usize`  |

#### Integer Literals
---
| Number literals  | Example       |
| ---------------- | ------------- |
| Decimal          | `98_222`      |
| Hex              | `0xff`        |
| Octal            | `0o77`        |
| Binary           | `0b1111_0000` |
| Byte (`u8` only) | `b'A'`        |


####  Floating Point 
---
Rust has both 32 bit and 64 bit floating point types.

**The default is 64 bit**

`let warp_factor = 8.9 //this is 64 bit`

`let f32 warp_factor = 3.2 //32 bit`

#### The Character Type
---
`char` literals are denoted bt single quotes `'char'`



#### Tuples
---

A _tuple_ is a general way of grouping together a number of values with a
variety of types into one compound type.

```rust,editable
fn main(){
let tup = (100,2.1,'c');

println!("{}, {}, {}",tup.0,tup.1,tup.2);

println!("{:?}",tup);  //print whole tuple
}
```

#### Arrays: Fixed-Size Data Structures
---
Arrays in Rust are fixed-size collections of elements of the same data type. Think of them as a row of identical crew quarters on a starship, where each quarter holds one crew member. Once declared, the size of an array cannot be changed.

**Declaration and Initialization:**

You declare an array by specifying the element type and the number of elements within square brackets `[]`. You can initialize an array with values directly:

```rust, editable
fn main() {
    let photon_torpedo_count: [u32; 5] = [0, 0, 0, 0, 0]; // An array of 5 unsigned 32-bit integers, all initialized to 0
    let crew_roster = ["Picard", "Riker", "Data", "Troi", "LaForge"]; // An array of 5 string slices; Rust infers the type

    println!("Initial torpedo count: {:?}", photon_torpedo_count);
    println!("Bridge crew: {:?}", crew_roster);
}
```

You can also initialize an array with the same value for all elements using a repetition syntax:

```rust, editable
fn main() {
    let shield_levels: [u8; 8] = [100; 8]; // An array of 8 unsigned 8-bit integers, all initialized to 100
    println!("Shield levels: {:?}", shield_levels);
}
```

**Accessing Elements:**

You access individual elements of an array using their index, starting from 0 for the first element:

```rust, editable
fn main() {
    let crew_roster = ["Picard", "Riker", "Data", "Troi", "LaForge"];
    let first_officer = crew_roster[1]; // Accessing the element at index 1 (Riker)

    println!("The first officer is: {}", first_officer);
}
```
  </div>
  </details>


### ![logo](Star_Trek_icon.png) Constants: Immutable Laws of the Universe

Besides regular variables, Rust also has constants. Constants are values that are bound to a name and are not allowed to be mutated. Unlike variables declared with let, you must annotate the type of a constant. Constants are declared using the `const` keyword. Think of these as the fundamental, unchanging laws of the universe within your program.
Rust

```rust, editable
const SPEED_OF_LIGHT: u32 = 299792458; // Type annotation is required
//const SPEED_OF_LIGHT:  i32 = 2;  //This will fail as constants can only be decalred once withing their scope, no shadowing.
fn main() {
    println!("The speed of light is {} meters per second.", SPEED_OF_LIGHT);
}
```

Here, const SPEED_OF_LIGHT: u32 = 299,792,458; declares a constant named SPEED_OF_LIGHT of type u32 (an unsigned 32-bit integer) and assigns it the value 299,792,458. Constants are typically declared in uppercase with underscores between words for readability and are often used for values that will never change throughout the program's execution.





<details class="discovery-details">
  <summary class="discovery-summary">
    <img src="info.png" alt="Star Trek Cadet" class="info-closed">
    <img src="info.png" alt="" class="info-open">
    Constant Versus Static
  </summary>
  <div class="discovery-content">
The difference between static and const variables in Rust. While both are used to declare global-like variables with a fixed value (at least in the case of const), there are key distinctions:

const Variables:

    Always Immutable: const variables are inherently immutable and cannot be declared as mutable using mut. Their value is fixed at compile time.
    Compile-Time Evaluation: The value assigned to a const variable must be a constant expression that can be evaluated at compile time. This means you can't assign the result of a function call (unless it's a const fn) or any value that isn't known during compilation.
    No Fixed Memory Location (Often Inlined): The compiler might choose to inline the value of a const variable directly at each place it's used in the code. This means it doesn't necessarily have a single, fixed memory location in the final executable.
    Lifetime: const variables don't have a single, specific memory location, so the concept of a fixed lifetime is less relevant. Their "lifetime" is essentially the scope where they are used.
    Use Cases: Primarily used for defining symbolic constants whose values are known at compile time and will never change throughout the program's execution. Examples include mathematical constants (like PI), configuration values, or fixed string literals.

static Variables:

    Potentially Mutable (but Unsafe): static variables can be declared as mutable using the mut keyword. However, accessing or modifying a mutable static variable is inherently unsafe in Rust and requires an unsafe block. This is because Rust's memory safety guarantees cannot be enforced across global mutable state without explicit opt-in.
    Runtime Initialization (Generally): While static variables can be initialized with constant expressions, they can also be initialized with values computed at runtime (though often they are still initialized at compile time).
    Fixed Memory Location: static variables have a fixed memory location in the program's static memory. This means there is only one instance of the static variable throughout the entire program's lifetime.
    'static Lifetime: static variables have a 'static lifetime, meaning they live for the entire duration of the program.
    Use Cases: Typically used for global variables that need a fixed memory location and a 'static lifetime. This might include things like global counters, shared mutable state (with careful synchronization and unsafe blocks), or pointers to data that lives for the entire program.

Here's a table summarizing the key differences:
| Feature          | `const`                                  | `static`                                      |
| ---------------- | ---------------------------------------- | --------------------------------------------- |
| Mutability       | Always immutable                         | Potentially mutable (requires `unsafe`)       |
| Evaluation       | Compile-time only                        | Generally compile-time, but can be runtime    |
| Memory Location  | Often inlined, no fixed location        | Fixed location in static memory             |
| Lifetime         | Scope-based                             | `'static` (program lifetime)                 |
| `mut` Keyword    | Not allowed                             | Allowed (but requires `unsafe` for access)   |
| Use Cases        | Symbolic constants, compile-time values | Global variables, shared mutable state (unsafe) |

In essence:

    Use const for values that are truly constant and known at compile time. The compiler has more freedom to optimize const values.
    Use static for global variables that need a fixed memory address and a 'static lifetime. 

<img src='Yellow_Alert_Icon.png'>Be very cautious when using mutable static variables due to the inherent unsafety.


```rust, editable
const STANDARD_WARP_FACTOR_LIMIT: u8 = 9; // A constant representing the standard warp factor limit

static CURRENT_STARBASE: &str = "Earth Spacedock"; // A static variable for the current starbase

static mut ACTIVE_SHIELDS: bool = false; // A mutable static variable indicating if shields are active (requires unsafe)

fn main() {
    println!("The standard warp factor limit according to Starfleet regulations is: Warp {}", STANDARD_WARP_FACTOR_LIMIT);
    println!("The fleet is currently operating near: {}", CURRENT_STARBASE);

    println!("Are shields active? {}", is_shields_active());

    unsafe {
        ACTIVE_SHIELDS = true; // Engaging shields globally (requires unsafe block)
    }

    println!("Are shields active now? {}", is_shields_active());
}

fn is_shields_active() -> bool {
    unsafe {
        ACTIVE_SHIELDS // Accessing a mutable static variable requires an unsafe block
    }
}
```

**Explanation:**

`const STANDARD_WARP_FACTOR_LIMIT: u8 = 9;:` Here, we define a const variable named `STANDARD_WARP_FACTOR_LIMIT`. Its value, 9, is known at compile time and will never change during the program's execution. This is like a fixed regulation in Starfleet.

`static CURRENT_STARBASE: &str = "Earth Spacedock";`: This declares a static variable named `CURRENT_STARBAS`E and initializes it with the string "Earth Spacedock". This variable has a fixed memory location and a 'static lifetime, meaning it exists for the entire duration of the program. It represents a global piece of information that might be accessed from different parts of the codebase.

`static mut ACTIVE_SHIELDS: bool = false;`: This declares a mutable static variable named `ACTIVE_SHIELDS`, initialized to false. Because it's mutable, accessing and modifying it requires the use of unsafe blocks. This is because the Rust compiler cannot guarantee memory safety across global mutable state without explicit opt-in. It represents a global state that can change during the program's execution (like the activation of a starship's shields).

main function: We demonstrate accessing both the const and static variables. Notice that accessing the `mutable static ACTIVE_SHIELDS` is done within an unsafe block in both the main function and the `is_shields_active` function.
  </div>
</details>

### ![logo](Star_Trek_icon.png) Functions: Defining Starfleet Procedures

Functions are the fundamental building blocks of executable code in Rust. They allow you to encapsulate a specific task or a series of operations into a reusable block of code. Think of functions as the standard operating procedures (SOPs) that Starfleet officers follow to perform various tasks on a starship.

**Defining Functions:**

You define a function in Rust using the `fn` keyword, followed by the function name, a set of parentheses for parameters (if any), an optional arrow `->` followed by the return type, and finally, a block of code enclosed in curly braces `{}` which forms the function body.

Here's a simple example of a function that greets a Starfleet officer:

```rust, editable
fn greet_officer(rank: &str, name: &str) {
    println!("Greetings, {} {}! Welcome aboard.", rank, name);
}

fn main() {
    greet_officer("Commander", "Riker");
}
```

`fn greet_officer(rank: &str, name: &str)`: This line defines a function named `greet_officer`.
`rank: &str:` This declares a parameter named `rank` of type string slice (`&str`). It represents the officer's rank.
`name: &str:` This declares another parameter named `name` of type string slice (`&str`). It represents the officer's name.
The parentheses `()` enclose the parameters.

The code within the curly braces {} is the function body, which in this case, prints a greeting to the console.
`greet_officer("Commander", "Riker");`: This line in the main function calls the `greet_officer` function, passing the string literals "Commander" and "Riker" as arguments.

<details class="discovery-details">
  <summary class="discovery-summary">
    <img src="info.png" alt="Star Trek Cadet" class="info-closed">
    <img src="info.png" alt="" class="info-open">
    Passing References
  </summary>
  <div class="discovery-content">

The & symbol in the function signature fn greet_officer(rank: &str, name: &str) indicates that rank and name are references to string slices (str). Let's break down what this means:

- Reference: In Rust, a reference is a way to access a value without taking ownership of it. Think of it like pointing to a location in memory where the actual data resides.

- Immutable Reference (&): The single & symbol specifically denotes an immutable reference. This means that the greet_officer function is allowed to read the values of rank and name, but it cannot modify the original data that these references point to.

- String Slice (&str): The type &str represents a "string slice". It's an immutable view into a string. String literals (like "Commander" and "Riker") are string slices. Also, you can create a slice of a String.

Why use references here?

1. No Ownership Transfer: When you pass a value to a function in Rust, ownership of that value is typically transferred to the function. However, in the case of String (which is a growable, heap-allocated string), transferring ownership can sometimes be inefficient and might prevent the calling code from using the original string afterwards. By using references, we are just borrowing the data; ownership remains with the caller (main function in this case), and the original string literals are still valid after the greet_officer function has finished executing.

2. Efficiency: Passing by reference is often more efficient than passing by value, especially for larger data types. When you pass by value, the data might need to be copied. With references, we are just passing a pointer to the data, which is a much smaller and faster operation.

3. String Literals: String literals like "Commander" and "Riker" are inherently string slices (&str). To pass them directly to the function without causing type mismatches, the function parameters are defined as &str.

  </div>
  </details>


#### Parameters:

Functions can take zero or more parameters. Each parameter must have a name and a specified type. Multiple parameters are separated by commas.

```rust, editable
fn calculate_warp_jump_time(distance_ly: f64, warp_factor: f64) -> f64 {
    // Simplified calculation (not scientifically accurate!)
    distance_ly / warp_factor.powf(3.0)
}

fn main() {
    let distance = 1000.0;
    let warp = 8.0;
    let time = calculate_warp_jump_time(distance, warp);
    println!("Estimated warp jump time: {:.2} stardays", time);
}
```
`fn calculate_warp_jump_time(distance_ly: f64, warp_factor: f64) -> f64`: This function takes two parameters, `distance_ly` and warp_factor, both of type f64 (64-bit floating-point numbers).
`-> f64`: This indicates that the function returns a value of type f64.

#### Return Values:

Functions can return a value to the caller. You specify the return type after the parameter list using the -> syntax. There are two main ways to return a value from a function:

Implicit Return (Last Expression): The value of the last expression in the function body will be automatically returned if there is no explicit return statement.
Explicit Return with return: You can use the return keyword followed by the value you want to return to exit the function immediately.

In the `calculate_warp_jump_time` example above, the last expression distance_ly / warp_factor.powf(3.0) is implicitly returned.

Here's an example using an explicit return statement:

```rust, editable
fn is_borg_cube(ship_class: &str) -> bool {
    if ship_class == "Borg Cube" {
        return true;
    } else {
        return false;
    }
}

fn main() {
    let ship = "USS Enterprise";
    if is_borg_cube(ship) {
        println!("Warning! Borg cube detected!");
    } else {
        println!("No Borg cube in this sector.");
    }
}
```
`fn is_borg_cube(ship_class: &str) -> bool`: This function takes a string slice `ship_class` and returns a boolean value (`bool`).
The `if` statement checks if the `ship_class` is "Borg Cube". If it is, the function explicitly returns `true`. Otherwise, it returns `false`.

#### Function Body:

The function body is the block of code enclosed in curly braces `{}` that contains the instructions to be executed when the function is called. It can contain any valid Rust code, including variable declarations, control flow statements, and calls to other functions.

#### Calling Functions:

You call a function by writing its name followed by parentheses `()`. If the function takes parameters, you provide the corresponding arguments inside the parentheses, separated by commas.

We've seen examples of calling functions in the main functions of the previous examples.

Functions with No Return Value:

If a function doesn't return any specific value, you can either omit the -> and return type or explicitly specify the return type as the unit type () (pronounced "unit").

```rust, editable
fn play_red_alert_sound() {
    println!("Red Alert! Red Alert!");
}

fn initiate_self_destruct_sequence() -> () {   //empty tuple
    println!("Initiating self-destruct sequence. Authorization required.");
    // In a real scenario, more complex logic would go here
}

fn main() {
    play_red_alert_sound();
    initiate_self_destruct_sequence();
}
```

Both `play_red_alert_sound` and `initiate_self_destruct_sequence` perform an action (printing to the console) but don't return a specific value. The `()` type represents an empty tuple and signifies that the function has no meaningful return value.

Functions are essential for organizing your code into logical units, making it more readable, reusable, and maintainable. By defining functions for specific tasks, you can build complex programs in a structured and manageable way, much like the intricate systems and procedures that keep a Starfleet vessel operational.

```rust, editable
 fn get_first_officer(crew: &[&str]) -> &str {
    if crew.is_empty() {
        "No officers aboard!"
    } else {
        crew[0] // Returns an immutable reference to the first element
    }
}

fn main() {
    let bridge_crew = ["Picard", "Riker", "Data", "Troi"];
    let first = get_first_officer(&bridge_crew); // Pass a slice of the array
    println!("The first officer is: {}", first);
}
```
- `fn get_first_officer(crew: &[&str]) -> &str`: This function takes a slice crew of string slices (`&[&str]`) as input and returns an immutable reference to a string slice (`&str`). The & before the return type indicates that we are returning a reference.

- `crew[0]`: Inside the function, if the slice is not empty, we return a reference to the element at index `0`. The & is implicitly added here because we are indexing into a borrowed slice.

- In the main function, we create an array `bridge_crew` and pass a slice of it (`&bridge_crew`) to `get_first_officer`. The returned reference is stored in the first variable, allowing us to access the first officer's name.

### ![logo](Star_Trek_icon.png) Starfleet Analogy Recap

 -   `let`: Assigning a crew member to a station.
 -   Immutability by Default: Starfleet directives ensuring system stability.
 -   `mut`: Getting authorization to reassign a crew member when necessary.
 -   Shadowing: Temporarily redefining a designation for a specific task.
 -   `const`: The fundamental, unchanging laws governing the universe (or your program).
