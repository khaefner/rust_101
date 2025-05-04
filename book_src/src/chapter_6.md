<link rel="stylesheet" href="star.css">

## Chapter 6: Error Handling Protocols: Navigating the Unexpected
![logo](Line_Header_Star_Trek.png)

Welcome, cadets, to a critical aspect of starship operation and Rust programming: error handling. Just as a Starfleet crew must be prepared to deal with unexpected anomalies, system failures, or hostile encounters, our Rust programs need robust mechanisms to handle errors gracefully and prevent catastrophic failures (warp care dumps). In this chapter, we'll explore Rust's approach to error handling, ensuring our code can navigate the unexpected with the same resilience as a starship venturing into uncharted space.

Unlike most languages, which typically handle both recoverable and unrecoverable errors uniformly with mechanisms like exceptions, Rust distinguishes between them. Rust avoids exceptions, instead employing the `Result<T, E>` type for errors that can be recovered from and the `panic!` macro to halt execution upon encountering unrecoverable errors.


### ![logo](Star_Trek_icon.png) Unrecoverable Errors with `panic!`: Engaging Emergency Protocols



Sometimes, our programs encounter situations where recovery is impossible, and the best course of action is to immediately stop execution. In Rust, this is achieved using the `panic!` macro. Think of `panic!` as engaging the ship's emergency protocols – a last resort when a critical system fails beyond repair.

**Note:** Rust takes this approach so that it does not end up in an unkown state (like reading from a pointer that has been been deleted)

Here's a simple example of how `panic!` works:

```rust, editable
fn main() {
    let warp_core_status = "critical overload";

    if warp_core_status == "critical overload" {
        panic!("Warp core breach imminent! Initiate emergency ejection sequence!");
    }

    println!("Warp core stable."); // This line will not be reached if a panic occurs
}
```

In this code, we check the `warp_core_status`. If it's "critical overload," we invoke panic! with a descriptive message. When `panic!` is called, the program will print the panic message, unwind the stack (cleaning up resources), and then terminate. The message provided to `panic!` should be informative enough to understand the reason for the unrecoverable error.

Here is an example of the code automatically panicking. Running the code will show you a backtrace on where the code panicked.

```rust,editable
fn main() {
    let starfleet_ranks = ["Ensign", "Lieutenant", "Commander"]; // This array has 3 elements, with indices 0, 1, and 2.

    // We are trying to access the element at index 3, which is outside the valid range [0, 2].
    let out_of_bounds_access = starfleet_ranks[3];

    println!("Attempted access: {}", out_of_bounds_access); // This line will not be reached
}
```


**When to use `panic!`:** Generally, you should only use panic! for truly unrecoverable errors or when a fundamental assumption in your code has been violated. For most errors that your program might encounter during normal operation (like a file not being found or a network connection failing), a more graceful approach is usually preferred.

### ![logo](Star_Trek_icon.png) Recoverable Errors with Result: Reporting Mission Failures

For errors that our program can potentially recover from, Rust provides the Result enum. Think of Result as a detailed mission report – it either indicates success with a resulting value or failure with an error message. The Result enum is defined as follows:

```rust, editable
enum Result<T, E> {
    Ok(T),    // Represents a successful operation with a value of type T
    Err(E),   // Represents a failed operation with an error value of type E
}
```

`Ok(T)`: This variant indicates that the operation was successful and holds the resulting value of type `T`.
`Err(E)`: This variant indicates that the operation failed and holds an error value of type`E` The `E` type is often used to represent the specific kind of error that occurred.

Here's an example of a function that might return a Result:

```rust, editable
use std::fs::File;
use std::io::{ErrorKind, Error}; // Import Error as well

fn open_log_file(filename: &str) -> Result<File, Error> { // Use Error here
    match File::open(filename) {
        Ok(file) => Ok(file), // File opened successfully, return the File wrapped in Ok
        Err(error) => match error.kind() {
            ErrorKind::NotFound => {
                // Log file not found, perhaps create it? For now, return the error
                Err(error)
            }
            other_error => {
                // Some other error occurred
                // Convert ErrorKind to Error using .into()
                Err(other_error.into())
            }
        },
    }
}

fn main() {
    match open_log_file("starship_log.txt") {
        Ok(file) => println!("Successfully opened the log file."),
        Err(error) => println!("Failed to open the log file: {:?}", error),
    }
}
```

**In this code:**

`use std::io::{ErrorKind, Error};:` We explicitly import the `Error` type from `std::io`.
`fn open_log_file(filename: &str) -> Result<File, Error>:` While not strictly necessary for the fix, it's clearer to use the type alias `Error` for `std::io::Error`.
`Err(other_error.into()):` In the other_error arm, we now call `.into()` on `other_error` (which is of type `ErrorKind`). This converts the `ErrorKind` into a `std::io::Error`, which is the expected type for the `Err` variant of our `Result`.
In the main function, we call `open_log_file` and use another match expression to handle the returned `Result`. If it's `Ok`, we print a success message. If it's `Err`, we print an error message (using `{:?}` to display the error for debugging).

### ![logo](Star_Trek_icon.png) Handling Result: Examining the Mission Report

Rust provides several ways to handle Result values, allowing you to examine the outcome of an operation.
match Statement: Detailed Analysis

As seen in the previous example, the match statement is a powerful way to handle Result because it forces you to explicitly consider both the Ok and Err cases.
if let: Concise Handling of Success or Failure

If you're only interested in handling one of the Result variants (either Ok or Err) and want to ignore the other, you can use if let.

```rust, editable
use std::fs::File;
use std::io::{ErrorKind, Error}; // Import Error as well
fn main() {

fn open_log_file(filename: &str) -> Result<File, Error> { // Use Error here
    match File::open(filename) {
        Ok(file) => Ok(file), // File opened successfully, return the File wrapped in Ok
        Err(error) => match error.kind() {
            ErrorKind::NotFound => {
                // Log file not found, perhaps create it? For now, return the error
                Err(error)
            }
            other_error => {
                // Some other error occurred
                // Convert ErrorKind to Error using .into()
                Err(other_error.into())
            }
        },
    }
}

    let log_file_result = open_log_file("starship_log.txt");

    if let Ok(_file) = log_file_result {
        println!("Log file opened successfully (using if let).");
    } else if let Err(error) = log_file_result {
        println!("Failed to open log file (using if let): {:?}", error);
    }
}
```

Here, `if let Ok(_file)` will only execute the code block if `log_file_result` is an `Ok` variant (we use `_file` to indicate that we are not actually using the File value in this case). Similarly, `else if let Err(error)` will execute its block only if the result is an `Err` variant, and it binds the error value to the error variable.
unwrap(): 

<div class="warning-block">
  <img src="Yellow_Alert_Icon.png" alt="Yellow Alert Icon" class="warning-icon">
  <p class="warning-text">
The Result type also has a method called unwrap(). If the Result is Ok, unwrap() will return the value inside Ok. However, if the Result is Err, unwrap() will cause your program to panic! It's generally best to avoid unwrap() in production code and instead handle errors explicitly using match or if let. unwrap() can be useful for quick prototyping or in tests where you expect an operation to always succeed.
  </p>
</div>


```rust, editable
use std::fs::File;
use std::io::{ErrorKind, Error}; // Import Error as well
// Be cautious when using unwrap()!
 fn main() {

    fn open_log_file(filename: &str) -> Result<File, Error> { // Use Error here
    match File::open(filename) {
        Ok(file) => Ok(file), // File opened successfully, return the File wrapped in Ok
        Err(error) => match error.kind() {
            ErrorKind::NotFound => {
                // Log file not found, perhaps create it? For now, return the error
                Err(error)
            }
            other_error => {
                // Some other error occurred
                // Convert ErrorKind to Error using .into()
                Err(other_error.into())
            }
        }
    }
}


     let log_file = open_log_file("starship_log.txt").unwrap();
     println!("Successfully opened the log file (using unwrap).");
 }
```



### ![logo](Star_Trek_icon.png) Providing a Custom Panic Message

Similar to unwrap(), the expect() method also returns the value inside Ok if the Result is successful. However, if the Result is Err, expect() will cause a panic! with a custom error message that you provide.

```rust, editable
use std::fs::File;
use std::io::{ErrorKind, Error}; // Import Error as well

    fn open_log_file(filename: &str) -> Result<File, Error> { // Use Error here
        match File::open(filename) {
            Ok(file) => Ok(file), // File opened successfully, return the File wrapped in Ok
            Err(error) => match error.kind() {
                ErrorKind::NotFound => {
                    // Log file not found, perhaps create it? For now, return the error
                    Err(error)
                }
                other_error => {
                    // Some other error occurred
                    // Convert ErrorKind to Error using .into()
                    Err(other_error.into())
                }
            }
        }
    }

// Use expect() to provide more context if a panic occurs
 fn main() {
     let log_file = open_log_file("critical_system_log.txt")
         .expect("Failed to open the critical system log file!");
     println!("Successfully opened the critical system log file.");
 }
```
`expect()` can be slightly better than `unwrap()` as it provides more context about why the program panicked, but it still leads to program termination in case of an error.

### ![logo](Star_Trek_icon.png) Propagating Errors: Reporting Up the Chain of Command

Often, a function might encounter an error that it doesn't know how to handle directly. In such cases, it's useful to propagate the error up the call stack to the calling function, which might have more context to decide what to do. Rust provides the ? operator to make error propagation easier.

```rust, editable
use std::fs;
use std::io;

fn read_stardate_from_log(filename: &str) -> Result<String, io::Error> {
    let content = fs::read_to_string(filename)?; // The '?' operator propagates the error
    Ok(content)
}

fn process_log_file() -> Result<(), io::Error> {
    let stardate = read_stardate_from_log("stardate_log.txt")?;
    println!("Current stardate: {}", stardate);
    Ok(())
}

fn main() -> Result<(), io::Error> {
    process_log_file()?;
    println!("Log processing complete.");
    Ok(())
}
```


**Here's how the ? operator works:**

If the `Result` value on which it's used is `Ok`, the `?` operator will return the value inside `Ok`.
If the `Result` value is `Err`, the `?` operator will return the `Err` value from the current function, effectively propagating the error up the call stack.

Important: The `?` operator can only be used in functions that themselves return a `Result` or `Option` (another type we might discuss later). In the main function, you can return a `Result<(), E>` to use the `?` operator.

**In our example:**

`read_stardate_from_log` reads the content of a file. If `fs::read_to_string` returns an `Err`, the `?` operator will immediately return that `Err` from `read_stardate_from_log`. If it's `Ok`, the content is assigned to content.
Similarly, in `process_log_file`, if `read_stardate_from_log` returns an `Err`, that error is propagated up.
The main function also returns `Result<(), io::Error>`, allowing it to use the `?` operator to handle potential errors from `process_log_file`.

### ![logo](Star_Trek_icon.png) Defining Custom Error Types: Creating Specific Mission Failure Reports

For more complex applications, you might want to define your own custom error types to provide more specific information about the errors that can occur in your program. You can do this using enums or structs.


```rust, editable
#[derive(Debug)]
enum DataProcessingError {
    InvalidFormat,
    MissingField(String),
    ChecksumMismatch,
    InvalidRange,
}

fn process_sensor_data(data: &str) -> Result<String, DataProcessingError> {
    if !data.starts_with("SENSOR:") {
        return Err(DataProcessingError::InvalidFormat);
    }
        // Split the data
    let parts: Vec<&str> = data.split(':').collect();
    let status = parts[1];
    let value_part_str = parts[2]; // We'll use the third part directly now for value

    // In a real scenario, we would perform more checks on status
    if status == "ERROR" {
         // Consider if "ERROR" status might still have a value part or if it's just an error report
        return Err(DataProcessingError::ChecksumMismatch); // Or a more specific error type
    }
    // Maybe enforce that status must be "OK" if not "ERROR"?
    if status != "OK" {
         return Err(DataProcessingError::InvalidFormat); // Or InvalidStatus?
    }

    // --- Value Extraction, Parsing, and Validation ---

    // 1. Extract the value string part after '='
    //    Use the third part directly (parts[2]) assuming format SENSOR:STATUS:Reading=Value
    let (_key, value_str) = value_part_str.split_once('=')
        .ok_or(DataProcessingError::InvalidFormat)?; // Use `ok_or` and `?` for concise error handling if '=' is missing

    // 2. Parse the value string into an i32
    //    Use `parse` which returns Result, and `?` will propagate the ParseIntError
    //    (converted via the `From` trait implementation above, or use `.map_err` explicitly)
    let actual_num: i32 = value_str.parse()?; // '?' handles the Result<i32, ParseIntError>

    // 3. --- THE FIX ---
    //    Now `actual_num` is definitely an i32, so we can compare it directly.
    if actual_num < 1 || actual_num > 100 {
        return Err(DataProcessingError::InvalidRange);
    }
    Ok(format!("Processed: {}", data))
}

fn main() {
    let result = process_sensor_data("SENSOR:OK:Reading=42");
    //let result = process_sensor_data("SENSOR:OK:Reading=42");
    //let result = process_sensor_data("SENSOR:OK:Reading=42");
    match result {
        Ok(output) => println!("Data processing successful: {}", output),
        Err(error) => println!("Data processing failed: {:?}", error),
    }

    let result_fail = process_sensor_data("INVALID_DATA");
    match result_fail {
        Ok(output) => println!("Data processing successful: {}", output),
        Err(error) => println!("Data processing failed: {:?}", error),
    }
}
```

In this example, we define an enum DataProcessingError with different variants representing specific errors that can occur during data processing. Our process_sensor_data function now returns a Result with our custom error type. This allows for more precise error handling in the main function. The #[derive(Debug)] attribute allows us to easily print the error for debugging purposes.
The Error Trait: Standardizing Error Reporting

Rust's standard library provides the std::error::Error trait, which is a trait that error types should implement to provide a standard way of working with errors. Implementing this trait allows you to access more information about an error, such as its source (if it was caused by another error). For our simple examples, deriving Debug on our custom error types is often sufficient.
Best Practices for Error Handling: Starfleet Standard Procedures

**Prefer Result for recoverable errors:**
Use Result to signal that an operation might fail in a way that the calling code can handle.

**Use `panic!` sparingly for truly unrecoverable errors:**
Reserve `panic!` for situations where continuing execution would lead to unsafe or incorrect behavior.

**Provide informative error messages:**  Whether you're using `panic!` or the `Err` variant of `Result`, make sure the error message is clear and helpful for debugging.

**Handle errors explicitly:** Avoid excessive use of `unwrap()` or `expect()` in production code. Instead, use `match`, `if let`, or the `?` operator to handle errors gracefully.

**Consider defining custom error types:** For complex applications, custom error types can provide more context and make error handling more precise.

### ![logo](Star_Trek_icon.png) Conclusion: Ensuring Mission Success Through Proper Error Handling

Mastering error handling is essential for writing robust and reliable Rust programs, just as having well-defined emergency protocols is crucial for the safety of a starship and its crew. By understanding and utilizing panic! for unrecoverable errors and Result for recoverable ones, along with the various ways to handle Result values, you'll be well-equipped to navigate the unexpected challenges that arise in the vast universe of software development. Continue to practice these techniques, and your code will be as resilient as any Starfleet vessel!
