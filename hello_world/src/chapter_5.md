## Chapter 5: Defining Starfleet Structures and Classifications
![logo](Line_Header_Star_Trek.png)

Welcome back, cadets! In this chapter, we'll delve into the creation of custom data types in Rust using two powerful tools: structs and enums. Think of structs as blueprints for our Starfleet vessels, allowing us to group together related information. Enums, on the other hand, are like the classifications we use to categorize different types of ships or the various operational states they can be in. Mastering these concepts will enable us to model complex systems within our Rust programs with clarity and precision.

### Structs: Creating Starship Blueprints

Structs (short for "structures") are a way to group together multiple values of different types under a single name. They allow you to create your own custom types that represent real-world entities, like a starship. Think of a struct as a blueprint that defines the properties or fields of a starship.

Here's how we can define a struct to represent a starship:

```rust, editable
#[derive(Debug)]
struct Starship {
    name: String,
    class: String,
    registry: String,
    warp_capable: bool,
    crew_capacity: u32,
}
```
In this code:

    struct Starship declares a new struct named Starship.
    The curly braces {} enclose the definitions of the fields within the struct.
    Each field has a name (e.g., name, class) and a type (e.g., String, bool, u32), separated by a colon.
    #[derive(Debug)] is an attribute that automatically implements the Debug trait for our Starship struct. This allows us to easily print the struct's contents for debugging using the {:?} format specifier in println!.

Now that we have defined our Starship struct, we can create instances of it:

```rust, editable
fn main() {
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
Tuple Structs

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
Enums: Defining Starfleet States and Classifications

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

    enum StarshipClass declares a new enum named StarshipClass.
    The values within the curly braces are called variants. Here, Cruiser, Destroyer, ScienceVessel, Freighter, and Shuttle are the possible values that a variable of type StarshipClass can hold.
    We access enum variants using the double colon :: (e.g., StarshipClass::Cruiser).

Enums with Data

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

Using match with Enums

Enums are often used with the match control flow construct, which allows you to execute different code based on the specific variant of the enum.

```rust, editable
fn report_status(status: &ShipStatus) {
    match status {
        ShipStatus::Online => println!("Ship is online and operational."),
        ShipStatus::Offline => println!("Ship is currently offline."),
        ShipStatus::Warping(factor) => println!("Ship is warping at factor {}.", factor),
        ShipStatus::Docked { at_starbase } => println!("Ship is docked at {}.", at_starbase),
        ShipStatus::Alert(level) => println!("Ship is under {}!", level),
    }
}
```
fn main() {
    let ship_status = ShipStatus::Warping(7.5);
    report_status(&ship_status);

    let another_status = ShipStatus::Docked { at_starbase: String::from("Earth Spacedock") };
    report_status(&another_status);
}

The match expression in report_status checks the variant of the status enum and executes the corresponding code block. Notice how we can destructure the data associated with the Warping and Docked variants to access their values.
Methods on Structs and Enums: Giving Starships and States Functionality

We can define methods (functions associated with a specific type) on both structs and enums using the impl (implementation) keyword. This allows us to add behavior to our custom data types.
Methods on Structs

```rust, editable
impl Starship {
    fn describe(&self) {
        println!("This is the {} class vessel '{}' with registry {}.", self.class, self.name, self.registry);
    }

    fn is_ready(&self) -> bool {
        self.warp_capable && self.crew_capacity > 0
    }
}
```
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
In the impl Starship block, we define two methods: describe and is_ready. The &self parameter in the describe method is a reference to the instance of the Starship struct on which the method is being called. The is_ready method returns a boolean value based on the warp_capable and crew_capacity fields.
Methods on Enums

```rust, editable
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
Here, we define a method can_engage_warp on the ShipStatus enum. It uses a match expression to determine if the current status allows warp travel.
Conclusion: Building Blocks of the Federation Fleet

Structs and enums are fundamental building blocks in Rust, allowing us to create well-organized and meaningful data structures that accurately represent the entities and states within our programs. By using structs, we can define the properties of complex objects like starships, and with enums, we can represent a finite set of possible values or states. Combined with methods, these constructs enable us to model the behavior of our systems in a clear and concise manner. As you continue your journey in Rust, you'll find structs and enums to be invaluable tools in your programming arsenal, helping you build applications as sophisticated and reliable as the technology of the United Federation of Planets. 
