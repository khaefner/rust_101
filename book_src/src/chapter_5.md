<link rel="stylesheet" href="star.css">

## Chapter 5: Defining Starfleet Structures and Classifications
![logo](Line_Header_Star_Trek.png)

Welcome back, cadets! In this chapter, we'll delve into the creation of custom data types in Rust using two powerful tools: structs and enums. Think of structs as blueprints for our Starfleet vessels, allowing us to group together related information. Enums, on the other hand, are like the classifications we use to categorize different types of ships or the various operational states they can be in. Mastering these concepts will enable us to model complex systems within our Rust programs with clarity and precision.

### struct (Structure)

--- 


- Purpose: To group together related pieces of data that form a meaningful unit. Each piece of data is called a "field." A struct defines a type that has a fixed set of fields.

- Analogy: Think of a blueprint for an object that always has specific properties.
    - A User struct might always have a username, an email, and an age.
    - A Point struct might always have an x coordinate and a y coordinate.
- "AND" Logic: An instance of a struct contains field A AND field B AND field C, etc.
- Data Storage: All fields defined in the struct are present in every instance of that struc

A struct is similar to a class in python.

### enum (Enumeration)

---


- Purpose: To define a type that can have one of several possible variants (or states). Each variant can optionally hold associated data.
- Analogy: Think of a set of choices.
    - A TrafficLightColor enum could be either Red, Yellow, or Green.
    - A Message enum could be either a Quit message (with no data), a Move message (with x and y coordinates), or a Write message (with a string).
- "OR" Logic: An instance of an enum is variant A OR variant B OR variant C, etc.
- Data Storage: An instance of an enum will only store the data associated with the specific variant it currently represents. The variants themselves can hold different types and amounts of data.

An enum is similar to a dictionary in python.

### ![logo](Star_Trek_icon.png) Structs: Creating Starship Blueprints

Structs (short for "structures") are a way to group together multiple values of different types under a single name. They allow you to create your own custom types that represent real-world entities, like a starship. Think of a struct as a blueprint that defines the properties or fields of a starship.

Here's how we can define a struct to represent a starship:

```rust
#[derive(Debug)]  // this give us a way to print the struct easily using the {:?}
struct Starship {
    name: String,
    class: String,
    registry: String,
    warp_capable: bool,
    crew_capacity: u32,
}
```
In this code:

struct `Starship` declares a new struct named Starship.
The curly braces `{}` enclose the definitions of the fields within the struct.
Each field has a name (e.g., name, class) and a type (e.g., String, bool, u32), separated by a colon.
`#[derive(Debug)]` is an attribute that automatically implements the Debug trait for our Starship struct. This allows us to easily print the struct's contents for debugging using the `{:?}` format specifier in `println!`.

Now that we have defined our Starship struct, we can create instances of it:

```rust, editable
fn main() {
    #[derive(Debug)]
struct Starship {
    name: String,
    class: String,
    registry: String,
    warp_capable: bool,
    crew_capacity: u32,
}
    let enterprise = Starship {
        name: String::from("USS Enterprise"),
        class: String::from("Constitution"),
        registry: String::from("NCC-1701"),
        warp_capable: true,
        crew_capacity: 203,
    };

    println!("Behold the: {:?}", enterprise);
}
```
Here, we create an instance of Starship named enterprise. We provide values for each field in the order they are defined in the struct, using the syntax field_name: value.

We can access the individual fields of a struct instance using dot notation:

```rust, editable
    #[derive(Debug)]
struct Starship {
    name: String,
    class: String,
    registry: String,
    warp_capable: bool,
    crew_capacity: u32,
}
fn main() {
    let defiant = Starship {
        name: String::from("USS Defiant"),
        class: String::from("Defiant"),
        registry: String::from("NX-74205"),
        warp_capable: true,
        crew_capacity: 50,
    };

    println!("The {} is a {} class vessel.", defiant.name, defiant.class);
    println!("Registry number: {}", defiant.registry);
    if defiant.warp_capable {
        println!("Warp drive engaged!");
    }
}
```

Mutable structs work like you think they would:

```rust,editable
 #[derive(Debug)]
struct Starship {
    name: String,
    class: String,
    registry: String,
    warp_capable: bool,
    crew_capacity: u32,
}
fn main() {
    let mut defiant = Starship { // Add 'mut' here
        name: String::from("USS Defiant"),
        class: String::from("Defiant"),
        registry: String::from("NX-74205"),
        warp_capable: true,
        crew_capacity: 50,
    };

    println!("The {} is a {} class vessel.", defiant.name, defiant.class);
    println!("Registry number: {}", defiant.registry);
    if defiant.warp_capable {
        println!("Warp drive engaged!");
    }

    // Now you can change crew_capacity
    println!("Original crew capacity: {}", defiant.crew_capacity);
    defiant.crew_capacity = 60; // Change the value
    println!("Upgraded crew capacity: {}", defiant.crew_capacity);

    // You could also change other fields if needed
    // defiant.name = String::from("Renamed Defiant");
}
```



### ![logo](Star_Trek_icon.png) Tuple Structs

Rust also provides a variation called tuple structs, which are like named tuples. They don't have named fields; instead, you access their elements by index.  


```rust, editable
#[derive(Debug)]
struct Coordinates(f64, f64, f64); // Represents X, Y, Z coordinates

fn main() {
    let earth_coordinates = Coordinates(0.0, 0.0, 0.0);
    println!("Earth's coordinates: {:?}", earth_coordinates);
    println!("X-coordinate: {}", earth_coordinates.0);
    println!("Y-coordinate: {}", earth_coordinates.1);
    println!("Z-coordinate: {}", earth_coordinates.2);
}
```
Tuple structs can be useful when you want to give a name to a tuple but don't necessarily need names for each individual element.
Unit Structs

Finally, Rust has unit structs, which don't have any fields at all. They are useful when you need to create a type that only represents a concept without holding any data.  


```rust, editable
struct PhotonTorpedo; // Represents a photon torpedo

fn main() {
    let torpedo = PhotonTorpedo;
    println!("A photon torpedo has been launched.");
    // Unit structs don't have any fields to access
}
```
### ![logo](Star_Trek_icon.png) Enums: Defining Starfleet States and Classifications

Enums (short for "enumerations") allow you to define a type by enumerating its possible values. Think of enums as the different classifications of starships (e.g., Cruiser, Destroyer, Science Vessel) or the various operational states a ship can be in (e.g., Docked, InWarp, Alert).  

Here's how we can define an enum for starship classes:

```rust, editable
#[derive(Debug)]
enum StarshipClass {
    Cruiser,
    Destroyer,
    ScienceVessel,
    Freighter,
    Shuttle,
}

fn main() {
    let enterprise_class = StarshipClass::Cruiser;
    let defiant_class = StarshipClass::Destroyer;
    let voyager_class = StarshipClass::ScienceVessel;

    println!("Enterprise is a {:?}", enterprise_class);
    println!("Defiant is a {:?}", defiant_class);
    println!("Voyager is a {:?}", voyager_class);
}
```
In this code:

`enum` StarshipClass declares a new `enum` named `StarshipClass`.
The values within the curly braces are called variants. Here, Cruiser, Destroyer, ScienceVessel, Freighter, and Shuttle are the possible values that a variable of type StarshipClass can hold.
We access enum variants using the double colon :: (e.g., `StarshipClass::Cruiser`).

### ![logo](Star_Trek_icon.png) Enums with Data

A powerful feature of Rust enums is that they can hold data within their variants. This allows you to associate different kinds of data with each possible value of the enum.

```rust, editable
#[derive(Debug)]
enum ShipStatus {
    Online,
    Offline,
    Warping(f64), // Variant holding a warp factor (f64)
    Docked { at_starbase: String }, // Variant holding a named field
    Alert(String), // Variant holding an alert level (String)
}

fn main() {
    let ship1_status = ShipStatus::Online;
    let ship2_status = ShipStatus::Warping(9.9);
    let ship3_status = ShipStatus::Docked { at_starbase: String::from("Deep Space 9") };
    let ship4_status = ShipStatus::Alert(String::from("Red Alert"));

    println!("Ship 1 status: {:?}", ship1_status);
    println!("Ship 2 status: {:?}", ship2_status);
    println!("Ship 3 status: {:?}", ship3_status);
    println!("Ship 4 status: {:?}", ship4_status);
}
```
Here, our ShipStatus enum can represent different states:

    Online and Offline are simple variants without any associated data.
    Warping is a variant that holds a f64 value representing the warp factor.
    Docked is a variant that holds a struct-like data structure with a named field at_starbase of type String.
    Alert is a variant that holds a String representing the alert level.

### ![logo](Star_Trek_icon.png) Using match with Enums

Enums are often used with the match control flow construct, which allows you to execute different code based on the specific variant of the enum.

```rust, editable
#[derive(Debug)]
enum ShipStatus {
    Online,
    Offline,
    Warping(f64), // Variant holding a warp factor (f64)
    Docked { at_starbase: String }, // Variant holding a named field
    Alert(String), // Variant holding an alert level (String)
}
fn report_status(status: &ShipStatus) {
    match status {
        ShipStatus::Online => println!("Ship is online and operational."),
        ShipStatus::Offline => println!("Ship is currently offline."),
        ShipStatus::Warping(factor) => println!("Ship is warping at factor {}.", factor),
        ShipStatus::Docked { at_starbase } => println!("Ship is docked at {}.", at_starbase),
        ShipStatus::Alert(level) => println!("Ship is under {}!", level),
    }
}
fn main() {
    let ship_status = ShipStatus::Warping(7.5);
    report_status(&ship_status);

    let another_status = ShipStatus::Docked { at_starbase: String::from("Earth Spacedock") };
    report_status(&another_status);
}
```

The match expression in report_status checks the variant of the status enum and executes the corresponding code block. Notice how we can destructure the data associated with the Warping and Docked variants to access their values.

### ![logo](Star_Trek_icon.png) Methods on Structs and Enums: Giving Starships and States Functionality

We can define methods (functions associated with a specific type) on both structs and enums using the impl (implementation) keyword. This allows us to add behavior to our custom data types.
#### Methods on Structs

```rust, editable
#[derive(Debug)]
struct Starship {
    name: String,
    class: String,
    registry: String,
    warp_capable: bool,
    crew_capacity: u32,
}
impl Starship {
    fn describe(&self) {
        println!("This is the {} class vessel '{}' with registry {}.", self.class, self.name, self.registry);
    }

    fn is_ready(&self) -> bool {
        self.warp_capable && self.crew_capacity > 0  //implict return of boolean
        //return self.warp_capable && self.crew_capacity > 0;  //explicit return
        /*Note both self.warp_capable and (self.crew_capacity > 0) are booleans*/
    }
}

fn main() {
    let defiant = Starship {
        name: String::from("USS Defiant"),
        class: String::from("Defiant"),
        registry: String::from("NX-74205"),
        warp_capable: true,
        crew_capacity: 50,
    };

    defiant.describe();
    println!("Is the ship ready? {}", defiant.is_ready());
}
```
In the `impl Starship` block, we define two methods: describe and is_ready. The `&self` parameter in the describe method is a reference to the instance of the Starship struct on which the method is being called. The `is_ready` method returns a boolean value based on the warp_capable and crew_capacity fields.

### ![logo](Star_Trek_icon.png) Methods on Enums

```rust, editable
enum ShipStatus {
    Online,
    Offline,
    Warping(f64), // Variant holding a warp factor (f64)
    Docked { at_starbase: String }, // Variant holding a named field
    Alert(String), // Variant holding an alert level (String)
}
impl ShipStatus {
    fn can_engage_warp(&self) -> bool {
        match self {
            ShipStatus::Warping(_) | ShipStatus::Online => true,
            _ => false,
        }
    }
}

fn main() {
    let status1 = ShipStatus::Online;
    let status2 = ShipStatus::Docked { at_starbase: String::from("Jupiter Station") };

    println!("Can ship 1 engage warp? {}", status1.can_engage_warp());
    println!("Can ship 2 engage warp? {}", status2.can_engage_warp());
}
```
Here, we define a method `can_engage_warp` on the `ShipStatus` enum. It uses a match expression to determine if the current status allows warp travel.

<details class="discovery-details">
  <summary class="discovery-summary">
    <img src="info.png" alt="Star Trek Cadet" class="info-closed">
    <img src="info.png" alt="" class="info-open">
    The `_` Underscore
  </summary>
  <div class="discovery-content">

The underscore _ in Rust serves multiple purposes as a special identifier. Primarily, it's used to indicate that a variable or parameter is intentionally unused, preventing compiler warnings. In match expressions, _ acts as a wildcard pattern, matching any value that hasn't been matched by previous arms. It can also be used within patterns to ignore specific parts of a structure, like fields in a struct or elements in a tuple. Additionally, _ can sometimes be used as a placeholder for type inference, allowing the compiler to deduce the type. Finally, it can be used as a visual separator in numeric literals to enhance readability, such as 1_000_000.

  </div>
  </details>

### Traits: Defining Standard Starfleet Specifications

In Starfleet, different classes of starships might perform similar functions but have their own unique designs and implementations. However, they often adhere to common standards or protocols – a standard docking procedure, a universal translator interface, or a specific type of sensor array. In Rust, **traits** serve a similar purpose: they allow you to define shared behavior that different types can implement.

Think of a trait as a contract or a blueprint for functionality. It defines a set of methods that a type *must* provide if it implements that trait. This allows you to write code that works with any type that implements a particular trait, regardless of its specific implementation details.

**Defining a Trait:**

You define a trait using the `trait` keyword, followed by the trait name and a block containing method signatures.

```rust, editable
trait StarshipSystems {
    // Define methods that any implementing type must provide
    fn report_status(&self);
    fn activate_shields(&mut self);
    fn deactivate_shields(&mut self);
}
```

Okay, let's add a section on traits to Chapter 5, building upon the concepts of structs and enums. Traits are a crucial part of Rust, allowing you to define shared behavior that different types can implement.

Here's the section you can add to Chapter 5, perhaps after the discussion on methods:
Markdown

### Traits: Defining Standard Starfleet Specifications

In Starfleet, different classes of starships might perform similar functions but have their own unique designs and implementations. However, they often adhere to common standards or protocols – a standard docking procedure, a universal translator interface, or a specific type of sensor array. In Rust, **traits** serve a similar purpose: they allow you to define shared behavior that different types can implement.

Think of a trait as a contract or a blueprint for functionality. It defines a set of methods that a type *must* provide if it implements that trait. This allows you to write code that works with any type that implements a particular trait, regardless of its specific implementation details.

**Defining a Trait:**

You define a trait using the `trait` keyword, followed by the trait name and a block containing method signatures.

```rust
trait StarshipSystems {
    // Define methods that any implementing type must provide
    fn report_status(&self);
    fn activate_shields(&mut self);
    fn deactivate_shields(&mut self);
}
```

Here, we define a trait StarshipSystems. Any type that implements this trait must provide concrete implementations for the report_status, activate_shields, and deactivate_shields methods. The method signatures within the trait definition only declare the method names, parameters, and return types (if any); they don't provide the actual code for the method bodies.

Implementing a Trait:

To make a specific type adhere to a trait's contract, you implement the trait for that type using the impl TraitName for TypeName syntax.

Let's say we have two different starship structs, GalaxyClass and DefiantClass:

```rust, editable
trait StarshipSystems {
    // Define methods that any implementing type must provide
    fn report_status(&self);
    fn activate_shields(&mut self);
    fn deactivate_shields(&mut self);
}

#[derive(Debug)]
struct GalaxyClass {
    name: String,
    shields_active: bool,
    crew_count: u32,
}

#[derive(Debug)]
struct DefiantClass {
    name: String,
    shields_active: bool,
    phaser_banks: u8,
}

// Implement the StarshipSystems trait for GalaxyClass
impl StarshipSystems for GalaxyClass {
    fn report_status(&self) {
        println!("{} ({}) status:", self.name, "Galaxy Class");
        println!("  Shields Active: {}", self.shields_active);
        println!("  Crew Count: {}", self.crew_count);
    }

    fn activate_shields(&mut self) {
        println!("{} ({}) activating shields.", self.name, "Galaxy Class");
        self.shields_active = true;
    }

    fn deactivate_shields(&mut self) {
        println!("{} ({}) deactivating shields.", self.name, "Galaxy Class");
        self.shields_active = false;
    }
}

// Implement the StarshipSystems trait for DefiantClass
impl StarshipSystems for DefiantClass {
    fn report_status(&self) {
        println!("{} ({}) status:", self.name, "Defiant Class");
        println!("  Shields Active: {}", self.shields_active);
        println!("  Phaser Banks: {}", self.phaser_banks);
    }

    fn activate_shields(&mut self) {
        println!("{} ({}) raising shields.", self.name, "Defiant Class");
        self.shields_active = true;
    }

    fn deactivate_shields(&mut self) {
        println!("{} ({}) lowering shields.", self.name, "Defiant Class");
        self.shields_active = false;
    }
}

// This function works with any type that implements the StarshipSystems trait
fn perform_system_check<T: StarshipSystems>(ship: &mut T) {
    println!("Initiating system check...");
    ship.report_status();
    ship.activate_shields();
    ship.report_status();
    ship.deactivate_shields();
    ship.report_status();
    println!("System check complete.");
}

fn main() {
    let mut enterprise = GalaxyClass {
        name: String::from("USS Enterprise-D"),
        shields_active: false,
        crew_count: 1014,
    };

    let mut defiant = DefiantClass {
        name: String::from("USS Defiant"),
        shields_active: false,
        phaser_banks: 10,
    };

    perform_system_check(&mut enterprise);
    println!("---");
    perform_system_check(&mut defiant);
}
```

### ![logo](Star_Trek_icon.png) Conclusion: Building Blocks of the Federation Fleet

Structs and enums are fundamental building blocks in Rust, allowing us to create well-organized and meaningful data structures that accurately represent the entities and states within our programs. By using structs, we can define the properties of complex objects like starships, and with enums, we can represent a finite set of possible values or states. Combined with methods, these constructs enable us to model the behavior of our systems in a clear and concise manner. As you continue your journey in Rust, you'll find structs and enums to be invaluable tools in your programming arsenal, helping you build applications as sophisticated and reliable as the technology of the United Federation of Planets. 
