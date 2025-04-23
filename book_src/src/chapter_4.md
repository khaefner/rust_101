## Chapter 4: Exploring the Cosmos of Data: Collections and Iteration
![logo](Line_Header_Star_Trek.png)

Welcome back, cadets! In the vast expanse of space, Starfleet vessels encounter countless forms of data – sensor readings, crew manifests, star charts, and more. To manage this information effectively, we need ways to store and process collections of data. In this chapter, we'll explore some of Rust's fundamental collection types and learn how to iterate over them, allowing us to navigate the cosmos of data with precision.

### Vectors (`Vec<T>`): Resizable Starship Cargo Holds

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
    println!("First rank: {}", first_rank);
}
``` 

It's safer to use the get() method, which returns an Option. This prevents your program from crashing if you try to access an index that is out of bounds:

```rust,editable
fn main() {
    let starfleet_ranks = vec!["Ensign", "Lieutenant", "Commander"];
    let maybe_rank = starfleet_ranks.get(3); // Trying to access an index that doesn't exist
    match maybe_rank {
        Some(rank) => println!("Rank at index 3: {}", rank),
        None => println!("No rank found at index 3."),
    }
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

#### Iterating Over Vectors:

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
    for name in crew_names.into_iter() {
        println!("- {}", name);
        // The vector 'crew_names' is consumed here and can no longer be used
    }
    // println!("{:?}", crew_names); // This would cause an error as crew_names has been moved
}
```
### Hash Maps (HashMap<K, V>): Storing Starship System Data

Hash maps are collections that store key-value pairs. Think of them as the databases on a starship, where you can quickly look up information (the value) using a unique identifier (the key). Hash maps are useful for associating data with specific labels.

Creating Hash Maps:

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
Inserting Elements:

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
Accessing Elements:

You can retrieve a value from a hash map using its key with the get() method, which returns an Option:
```rust,editable

use std::collections::HashMap;

fn main() {
    let starship_registry = HashMap::from([("USS Enterprise", "NCC-1701")]);
    let enterprise_registry = starship_registry.get("USS Enterprise");
    match enterprise_registry {
        Some(registry) => println!("USS Enterprise registry: {}", registry),
        None => println!("USS Enterprise not found in registry."),
    }
}
```
Mutability:

Hash maps are mutable if declared with mut. You can insert, update, or remove key-value pairs.

Iterating Over Hash Maps:

You can iterate over the key-value pairs in a hash map using a for loop:

    Iterating immutably:

```rust,editable

use std::collections::HashMap;

fn main() {
    let starship_registry = HashMap::from([("USS Enterprise", "NCC-1701"), ("USS Voyager", "NCC-74656")]);
    println!("Starship registry:");
    for (name, registry) in starship_registry.iter() {
        println!("- {}: {}", name, registry);
    }
}
```
    Iterating mutably:

```rust,editable

use std::collections::HashMap;

fn main() {
    let mut photon_torpedo_counts = HashMap::from([("Forward", 10), ("Aft", 5)]);
    println!("Initial torpedo counts: {:?}", photon_torpedo_counts);
    for (_, count) in photon_torpedo_counts.iter_mut() {
        *count += 2;
    }
    println!("Updated torpedo counts: {:?}", photon_torpedo_counts);
}
```
    Taking ownership and iterating:

```rust,editable

use std::collections::HashMap;

fn main() {
    let system_status = HashMap::from([
        (String::from("Warp Core"), String::from("Online")),
        (String::from("Shields"), String::from("Offline")),
    ]);
    println!("System status:");
    for (system, status) in system_status.into_iter() {
        println!("- {}: {}", system, status);
    }
    // The hash map 'system_status' is consumed here
}
```
Other Useful Collections

Rust's standard library provides other useful collection types, including:

String: While not strictly a collection in the same way as Vec or HashMap, it can be thought of as a collection of characters.
HashSet<T>: Stores unique values in no particular order. Useful for checking if a value exists in a set.
BTreeMap<K, V> and BTreeSet<T>: Similar to HashMap and HashSet, but they store elements in a sorted order based on their keys.

Iteration in Detail: The Core of Data Processing

Iteration is the process of going through the elements of a collection one by one. In Rust, iteration is often done using iterators.  

The Iterator Trait:

The Iterator trait in Rust defines the behavior of an iterator. The most important method of this trait is next(), which returns an Option. Each time you call next() on an iterator, it produces the next item in the sequence. When the iterator reaches the end, next() returns None.

for Loops and Iterators:

The for loop in Rust is actually done syntactically over iterators. When you write for element in collection, Rust automatically creates an iterator for the collection and repeatedly calls next() until it returns None.

Iterator Adaptors:

Iterators in Rust are often used with adaptors, which are methods that transform an iterator into another iterator. This allows you to perform operations on the elements of a collection in a concise and expressive way. Here are a few examples:  

`map()`: Applies a closure to each element of the iterator, producing a new iterator with the results.   

```rust,editable

fn main() {
    let warp_factors = vec![2.0, 5.0, 8.0];
    let doubled_factors: Vec<f64> = warp_factors.iter()
        .map(|factor| factor * 2.0)
        .collect(); // Collect the results into a new vector
    println!("Doubled warp factors: {:?}", doubled_factors);
}

filter(): Creates a new iterator that yields only the elements for which the provided closure returns true.
```rust,editable

    fn main() {
        let crew_ages = vec![25, 45, 30, 60, 35];
        let experienced_crew: Vec<u8> = crew_ages.iter()
            .filter(|&age| *age > 40)
            .copied() // Convert &u8 to u8 for the new vector
            .collect();
        println!("Experienced crew ages: {:?}", experienced_crew);
    }
```
collect(): Consumes the iterator and gathers the elements into a collection type, such as a Vec or HashMap.

These are just a few examples of the many powerful iterator adaptors available in Rust. They allow you to perform complex data transformations and manipulations in a clean and efficient manner.
Conclusion: Navigating Data with Confidence

Collections and iteration are fundamental tools for managing and processing data in Rust. Whether you're storing a list of starship components in a Vec, looking up system statuses in a HashMap, or transforming sensor readings using iterators, these concepts will be essential for building sophisticated and data-driven applications worthy of the Federation. As you continue your Rust journey, explore the various collection types and master the art of iteration to confidently navigate the vast cosmos of data