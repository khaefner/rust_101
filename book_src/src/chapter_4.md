## Chapter 4: Exploring the Cosmos of Data: Collections and Iteration
![logo](Line_Header_Star_Trek.png)

Welcome back, cadets! In the vast expanse of space, Starfleet vessels encounter countless forms of data – sensor readings, crew manifests, star charts, and more. To manage this information effectively, we need ways to store and process collections of data. In this chapter, we'll explore some of Rust's fundamental collection types and learn how to iterate over them, allowing us to navigate the cosmos of data with precision.

### ![logo](Star_Trek_icon.png) Vectors (`Vec<T>`): Resizable Starship Cargo Holds

Vectors are the most common type of collection in Rust. Think of them as resizable arrays, like the cargo holds of a starship that can expand or contract as needed. Vectors can store a sequence of elements of the same type.

**Creating Vectors:**

You can create an empty vector using `Vec::new()`:

```rust,editable
fn main() {
    let mut crew_manifest: Vec<&str> = Vec::new(); // Create an empty vector that will hold string slices
    crew_manifest.push("Riker");
    println!("Initial crew manifest: {:?}", crew_manifest);
}
```

You can also create a vector with initial values using the vec! macro:


```rust,editable
fn main() {
    let starfleet_ranks = vec!["Ensign", "Lieutenant", "Commander", "Captain", "Admiral"]; // Vector of string slices
    //starfleet_ranks.push("Cadet");
    println!("Starfleet ranks: {:?}", starfleet_ranks);
}
```

Or create a vector with a specific capacity (which can improve performance if you know the approximate number of elements beforehand):

```rust,editable
fn main() {
    let mut photon_torpedo_launch_sequence: Vec<u8> = Vec::with_capacity(1); // Vector to hold up to 10 u8 values
    photon_torpedo_launch_sequence.push(1);
    println!("Initial torpedo sequence capacity: {}", photon_torpedo_launch_sequence.capacity());
}
```
**Capacity and reallocation**

The capacity of a vector is the amount of space allocated for any future elements that will be added onto the vector. This is not to be confused with the length of a vector, which specifies the number of actual elements within the vector. If a vector’s length exceeds its capacity, its capacity will automatically be increased, but its elements will have to be reallocated.

For example, a vector with capacity 10 and length 0 would be an empty vector with space for 10 more elements. Pushing 10 or fewer elements onto the vector will not change its capacity or cause reallocation to occur. However, if the vector’s length is increased to 11, it will have to reallocate, which can be slow. For this reason, it is recommended to use Vec::with_capacity whenever possible to specify how big the vector is expected to get.

**Adding Elements:**

You can add elements to the end of a vector using the push() method (note that the vector must be declared as mutable):

```rust,editable
fn main() {
    let mut crew_manifest = vec!["Picard"];
    crew_manifest.push("Riker");
    crew_manifest.push("Data");
    println!("Updated crew manifest: {:?}", crew_manifest);
}
```

**Accessing Elements:**

You can access elements of a vector using indexing (starting from 0), similar to arrays:


```rust,editable
fn main() {
    let starfleet_ranks = vec!["Ensign", "Lieutenant", "Commander"];
    let first_rank = starfleet_ranks[0]; // Access the element at index 0
    //let first_rank = starfleet_ranks[5]; // Access the element at index 0
    println!("First rank: {}", first_rank);
}
``` 

It's safer to use the get() method, which returns an Option. This prevents your program from crashing if you try to access an index that is out of bounds:

```rust,editable
fn main() {
    let starfleet_ranks = vec!["Ensign", "Lieutenant", "Commander"];
    let maybe_rank = starfleet_ranks.get(2);  //This returns an Option Type
   //let maybe_rank = starfleet_ranks.get(3); // Trying to access an index that doesn't exist
   //Let's give the user some output we'll get into match in a bit
   /*
    match maybe_rank {
        Some(rank) => println!("Rank at index 3: {}", rank),
        None => println!("No rank found at index 3."), //Rust does not have nulls, but does have the concept as part of the enum Option <T>
    }
    */
}
```

**Mutability:**

Vectors are mutable if declared with mut. You can change the elements at specific indices:
```rust,editable

fn main() {
    let mut shield_levels = vec![50, 50, 50];
    println!("Initial shield levels: {:?}", shield_levels);
    shield_levels[1] = 75; // Modify the element at index 1
    println!("Updated shield levels: {:?}", shield_levels);
}
```

#### ![logo](Star_Trek_icon.png) Iterating Over Vectors:

You can iterate over the elements of a vector using a for loop in several ways:

**Iterating immutably:**

```rust,editable

fn main() {
    let starfleet_ranks = vec!["Ensign", "Lieutenant", "Commander"];
    println!("Starfleet ranks:");
    for rank in starfleet_ranks.iter() {
        println!("- {}", rank);
    }
}
```
**Iterating mutably:**

```rust,editable

fn main() {
    let mut phaser_banks = vec![50, 50, 50];
    println!("Initial phaser bank levels: {:?}", phaser_banks);
    for level in phaser_banks.iter_mut() {
        *level += 10; // Dereference the mutable reference to modify the value
    }
    println!("Updated phaser bank levels: {:?}", phaser_banks);
}
```
**Taking ownership and iterating:**

```rust,editable

fn main() {
    let crew_names = vec![String::from("Picard"), String::from("Riker"), String::from("Data")];
    println!("Crew names:");
    for name in crew_names.into_iter() {  //converts/moves into interatorg
        println!("- {}", name);
        // The vector 'crew_names' is consumed here and can no longer be used
    }
    // println!("{:?}", crew_names); // This would cause an error as crew_names has been moved
}
```
### ![logo](Star_Trek_icon.png) Hash Maps (HashMap<K, V>): Storing Starship System Data

Hash maps are collections that store key-value pairs. Think of them as the databases on a starship, where you can quickly look up information (the value) using a unique identifier (the key). Hash maps are useful for associating data with specific labels.

**Creating Hash Maps:**

You can create an empty hash map using HashMap::new() (you need to import it from the std::collections module):
```rust,editable

use std::collections::HashMap;

fn main() {
    let mut starship_registry: HashMap<&str, &str> = HashMap::new();
    println!("Initial registry: {:?}", starship_registry);
}
```
You can also create a hash map with initial key-value pairs:
```rust,editable

use std::collections::HashMap;

fn main() {
    let mut starship_registry = HashMap::from([
        ("USS Enterprise", "NCC-1701"),
        ("USS Voyager", "NCC-74656"),
        ("USS Defiant", "NX-74205"),
    ]);
    println!("Starship registry: {:?}", starship_registry);
}
```
**Inserting Elements:**

You can insert new key-value pairs into a hash map using the insert() method:
```rust,editable

use std::collections::HashMap;

fn main() {
    let mut starship_registry = HashMap::new();
    starship_registry.insert("USS Enterprise", "NCC-1701");
    starship_registry.insert("USS Voyager", "NCC-74656");
    println!("Updated registry: {:?}", starship_registry);
}
```
**Accessing Elements:**

You can retrieve a value from a hash map using its key with the get() method, which returns an Option:
```rust,editable

use std::collections::HashMap;

fn main() {
    let starship_registry = HashMap::from([("USS Enterprise", "NCC-1701")]);
    let registration = starship_registry.get("USS Enterprise");  //This returns an Option<T> Enum
    //let registration = starship_registry.get("USS Archer");  //This will have a None option.
    match registration {
        Some(registry) => println!("USS Enterprise registry: {}", registry), //here Some is a variant of option that conatins a value
        None => println!("USS Enterprise not found in registry."),  //None is a variant of option that encodes no value contained.
    }
}
```

The Option<T> is a funcdamental way that Rust keeps track of concept of a value being present or not.

**Mutability:**

Hash maps are mutable if declared with mut. You can insert, update, or remove key-value pairs.

**Iterating Over Hash Maps:**

__Iterating immutably:__

You can iterate over the key-value pairs in a hash map using a for loop:

```rust,editable

use std::collections::HashMap;

fn main() {
    let starship_registry = HashMap::from([("USS Enterprise", "NCC-1701"), 
    ("USS Voyager", "NCC-74656")]);
    println!("Starship registry:");
    for (name, registry) in starship_registry.iter() {
        println!("- {}: {}", name, registry);
    }
}
```
**Iterating mutably:**

```rust,editable

use std::collections::HashMap;

fn main() {
    let mut photon_torpedo_counts = HashMap::from([("Forward", 10), ("Aft", 5)]);
    println!("Initial torpedo counts: {:?}", photon_torpedo_counts);
    for (_, count) in photon_torpedo_counts.iter_mut() {  //iter_mut returns a mutable reference to specific entry in the HashMap
        *count += 2;
    }
    println!("Updated torpedo counts: {:?}", photon_torpedo_counts);
}
```
**Taking ownership and iterating:**

```rust,editable

use std::collections::HashMap;

fn main() {
    let mut system_status = HashMap::from([  //remember we need to declare mutable if we want to change it later.
        (String::from("Warp Core"), String::from("Online")),
        (String::from("Shields"), String::from("Offline")),
    ]);
    println!("System status:");
        for (system, status) in system_status.iter() {
        println!("- {}: {}", system, status);
    }

        // Example of iter_mut(): Modifying the value for "Shields"
    for (system, status) in system_status.iter_mut() {
        if system == "Shields" {
            *status = String::from("Online");
        }
    }

        println!("\nSystem status after bringing shields online:");
    for (system, status) in system_status.iter() {
        println!("- {}: {}", system, status);
    }

     println!("\nNow using into_iter, and consuming system_status");

    // The hash map 'system_status' is consumed here
    for (system, status) in system_status.into_iter() {
        println!("- {}: {}", system, status);
    }

    //meaning you can't use it here:
    /*for (system, status) in system_status.iter() {
        println!("- {}: {}", system, status);
    }*/

}
```

Think of it this way:

`.iter()`: "Borrowing" the contents to look at them. The original container remains yours.

`.iter_mut()`: "Borrowing" the contents to look at and potentially change them. The original container remains yours.

`.into_iter()`: "Taking" the contents out of the container. The original container is no longer yours.


## ![logo](Star_Trek_icon.png)  Other Useful Collections

Rust's standard library provides other useful collection types, including:

String: While not strictly a collection in the same way as Vec or HashMap, it can be thought of as a collection of characters.
HashSet<T>: Stores unique values in no particular order. Useful for checking if a value exists in a set.
BTreeMap<K, V> and BTreeSet<T>: Similar to HashMap and HashSet, but they store elements in a sorted order based on their keys.

## ![logo](Star_Trek_icon.png)  Iteration in Detail: The Core of Data Processing

Iteration is the process of going through the elements of a collection one by one. In Rust, iteration is often done using iterators.  

**The Iterator Trait:**

The Iterator trait in Rust defines the behavior of an iterator. The most important method of this trait is next(), which returns an Option. Each time you call next() on an iterator, it produces the next item in the sequence. When the iterator reaches the end, next() returns None.

for Loops and Iterators:

The for loop in Rust is actually done syntactically over iterators. When you write for element in collection, Rust automatically creates an iterator for the collection and repeatedly calls next() until it returns None.

**Iterator Adaptors:**

Important to note about iterators in rust.

```rust,editable

fn main() {
    let crew_ages = vec![25u8, 45u8, 30u8, 60u8, 35u8];
    let mut experienced_crew_ages = Vec::new();

    for age in crew_ages.iter() { // age is a reference of type &i32
        if *age > 40 { // Dereference 'age' here with *age
            println!("Found an experienced crew member aged: {}", *age);
            experienced_crew_ages.push(*age); // Push the dereferenced value
        }
    }
    println!("All experienced crew ages: {:?}", experienced_crew_ages);
}
```
```rust,editable
fn main() {
    let crew_ages = vec![25u8, 45u8, 30u8, 60u8, 35u8];
    let mut experienced_crew_ages = Vec::new();

    for age in crew_ages.into_iter() { // age is a copy
        if age > 40 { // Now we can use the value directly
            println!("Found an experienced crew member aged: {}", age);
            experienced_crew_ages.push(age); // Push the dereferenced value
        }
    }
    println!("All experienced crew ages: {:?}", experienced_crew_ages);
}
```

# ![logo](Star_Trek_icon.png)  Match

### Using `match`: Analyzing Conditions and Executing Protocols

The `match` control flow operator in Rust is incredibly powerful. It allows you to compare a value against a series of patterns and then execute code based on which pattern the value matches. Think of `match` as the ship's computer analyzing various sensor readings or system states and then executing the appropriate protocols based on the situation.

**Basic `match` Syntax:**

The basic syntax for a `match` expression is:

```rust,editable
match expression {
    pattern_1 => code_to_run_if_pattern_1_matches,
    pattern_2 => code_to_run_if_pattern_2_matches,
    // ... more arms
    pattern_n => code_to_run_if_pattern_n_matches,
}
```

Rust goes through the patterns one by one. When a pattern matches the expression, the corresponding code block is executed, and the match expression finishes.

Matching on Simple Values:

You can use match to compare a value against specific literal values:

```rust,editable
fn react_to_alert_level(level: u8) {
    match level {
        0 => println!("Condition Green: All systems normal."),
        1 => println!("Condition Yellow: Minor anomaly detected. Increased vigilance."),
        5 => println!("Condition Red: Major threat detected! Battle stations!"),
        _ => println!("Unknown alert level. Proceed with caution."),
    }
}

fn main() {
    react_to_alert_level(0);
    react_to_alert_level(5);
    react_to_alert_level(3); // Matches the wildcard arm
}
```

In this example, the match expression checks the value of the level variable. It matches the appropriate arm (0, 1, or 5) and prints a specific message. The _ is a wildcard pattern that matches any value not explicitly covered by the previous patterns. match expressions in Rust are exhaustive, meaning you must cover all possible cases for the type you are matching on (the wildcard _ is often used to fulfill this requirement).


## Conclusion: Mastering the Analysis Engine

The match expression is a fundamental control flow construct in Rust, providing a powerful and safe way to analyze values based on patterns. It's particularly useful for working with enums and handling Result and Option types. By mastering match, you gain the ability to write code that is both expressive and robust, capable of handling various situations with the precision of a Starfleet computer analyzing complex data streams. It's a key tool in your Rust programming arsenal, allowing you to execute the right protocols at the right time.