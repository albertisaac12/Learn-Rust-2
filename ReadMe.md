# RUST

Cargo is the rust Package manager
Cargo is also the Rust's build system

Git files won’t be generated if you run `cargo new` within an existing Git repository; you can override this behavior by using `cargo new --vcs=git.`

This file is in the `TOML (Tom’s Obvious, Minimal Language)` format, which is Cargo’s configuration format.

In Rust, packages of code are referred to as crates.

**IN rust it is important to understand that each package is a crate**

Cargo expects your source files to live inside the src directory.

The top-level project directory is just for README files, license information, configuration files, and anything else not related to your code.

One easy way to get that `Cargo.toml` file is to run `cargo init`, which will create it for you automatically.

when `cargo build` is ran it creates a executable under the `target/debug/PackageName`

the default build is a debug build, Cargo puts the binary in a directory named debug. You can run the executable with this command:
`./target/debug/PackageName #`

Running `cargo build` for the `first time` also causes Cargo to create a new file at the top level: `Cargo.lock`. This file keeps track of the exact versions of dependencies in your project.

Cargo also provides a command called `cargo check`. This command quickly checks your code to make sure it compiles but doesn’t produce an executable.

When your project is finally ready for release, you can use `cargo build --release` to compile it with optimizations. This command will create an executable in `target/release` instead of target/debug. The optimizations make your Rust code run faster, but turning them on lengthens the time it takes for your program to compile. This is why there are two different profiles: one for development, when you want to rebuild quickly and often, and another for building the final program you’ll give to a user that won’t be rebuilt repeatedly and that will run as fast as possible. If you’re benchmarking your code’s running time, be sure to run cargo build --release and benchmark with the executable in target/release.

## Note: Integer overflow and underflow will result in a panic only in the debug mode where as with the --release flag it will not be the case.

## 📌 Comparison Table

### **Trait**

Defines shared behavior for types. Acts like an interface. Types must implement it.

```rust
trait Speak {
    fn speak(&self);
}

struct Dog;

impl Speak for Dog {
    fn speak(&self) {
        println!("Woof!");
    }
}
```

### **Library (Crate)**

A Rust package (crate) that contains reusable code. Can be binary or library.

```rust
pub fn greet(name: &str) {
    println!("Hello, {}!", name);
}
```

**Usage:**

```rust
use my_library::greet;

fn main() {
    greet("Alice");
}
```

### **Module**

A way to organize code within a crate. Can be public (`pub`). Helps with reusability.

```rust
pub mod math {
    pub fn add(a: i32, b: i32) -> i32 {
        a + b
    }
}
```

**Usage:**

```rust
use my_library::math;

fn main() {
    println!("{}", math::add(2, 3));
}
```

### **Type (Struct)**

Defines data structures that hold values and methods.

```rust
struct Person {
    name: String,
    age: u32,
}
```

### **Type (Enum)**

Defines a set of possible values (variants). Useful for state representation.

```rust
enum Color {
    Red,
    Green,
    Blue,
}

fn print_color(c: Color) {
    match c {
        Color::Red => println!("Red"),
        Color::Green => println!("Green"),
        Color::Blue => println!("Blue"),
    }
}
```

```plaintext
 Why do we use rand::Rng;?
1. thread_rng() returns a random number generator.
2. That generator implements the Rng trait`.
3. The .gen_range() method is defined in the Rng trait`.
4. In Rust, traits must be in scope to use their methods.
   Without use rand::Rng;, the compiler doesn't know about .gen_range(), even though thread_rng() exists.

5. rand::thread_rng() returns a random number generator specific to the current thread.
6.It creates an instance of a secure, fast, and thread-local random number generator.
7.This function does not need to be seeded manually, as it automatically handles seeding.

8. .gen_range(1..=100)
9. .gen_range(start..=end) generates a random number within the given range (inclusive of both ends).
10. 1..=100 means generate a number from 1 to 100, including 100.
11. This method comes from the Rng trait, which is why we need use rand::Rng;.
```

The tuple without any values has a special name, `unit`. This value and its corresponding type are both written `()` and represent an empty value or an empty return type. Expressions implicitly return the unit value if they don’t return any other value.

## When your code calls a function, the values passed into the function (including, potentially, pointers to data on the heap) and the function’s local variables get pushed onto the stack. When the function is over, those values get popped off the stack.

```rust
//string literlas => &str;
//string Object => String;
```

```plaintext
With the String type, in order to support a mutable, growable piece of text, we need to allocate an amount of memory on the heap, unknown at compile time, to hold the contents. This means:
1. The memory must be requested from the memory allocator at runtime.
2. We need a way of returning this memory to the allocator when we’re done with our String.

That first part is done by us: when we call String::from, its implementation requests the memory it needs. This is pretty much universal in programming languages.

However, the second part is different. In languages with a garbage collector (GC), the GC keeps track of and cleans up memory that isn’t being used anymore, and we don’t need to think about it. In most languages without a GC, it’s our responsibility to identify when memory is no longer being used and to call code to explicitly free it, just as we did to request it. Doing this correctly has historically been a difficult programming problem. If we forget, we’ll waste memory. If we do it too early, we’ll have an invalid variable. If we do it twice, that’s a bug too. We need to pair exactly one allocate with exactly one free.
```

Rust calls the `drop` function as soon the variable goes out of the scope
now the drop function basically returns the space allocated for the variable

## Stack only copy and deep copy

Rust has a special annotation called the `Copy` trait that we can place on types that are stored on the stack, as integers are
If a type implements the `Copy` trait, variables that use it do not move, but rather are trivially copied, making them still valid after assignment to another variable.

Rust won’t let us annotate a type with `Copy` if the type, or any of its parts, has implemented the `Drop` trait.
If the type needs something special to happen when the value goes out of scope and we add the Copy annotation to that type, we’ll get a compile-time error.

Returning values can also transfer ownership.

```plaintext
The ownership of a variable follows the same pattern every time: assigning a value to another variable moves it.
When a variable that includes data on the heap goes out of scope, the value will be cleaned up by drop unless ownership of the data has been moved to another variable.
```

```rust
// two mutable borrow of the same variable cannot happen twice in the same scope
// 2 mutable references can be made if the scope changes
let mut s = String::from("hello");
{
    let r1 = &mut s;
}

let r2 = &mut s;



let mut s = String::from("hello");
let r1 = &s; // no problem
let r2 = &s; // no problem
let r3 = &mut s; // BIG PROBLEM
println!("{}, {}, and {}", r1, r2, r3); // will throw error as s was initially borrowed as an immutable reference

// Note that a reference’s scope starts from where it is introduced and continues through the last time that reference is used. For instance, this code will compile because the last usage of theimmutable references is in the println!, before the mutable reference is introduced:

let mut s = String::from("hello");

let r1 = &s; // no problem
let r2 = &s; // no problem
println!("{r1} and {r2}");

// variables r1 and r2 will not be used after this point
let r3 = &mut s; // no problem
println!("{r3}");

```

## Dangling Pointer

```rust
fn dangle() -> &String { // dangle returns a reference to a String

    let s = String::from("hello"); // s is a new String

    &s // we return a reference to the String, s
} // Here, s goes out of scope, and is dropped. Its memory goes away.
  // Danger!

```

## The Rules of References

1. At any given time, you can have either one mutable reference or any number of immutable references.
2. References must always be valid.

```rust

if let (1) = (2) {

} else {

}

// 1 should be the type you are matching 2 with example if 2 is of type Option<String> then 1 will be either Some(value) or None

```

## 🚀 Summary of `if let` and `let else` in Rust

| Feature         | Works With?               |
| --------------- | ------------------------- |
| Enums           | ✅ Yes                    |
| Option<T>       | ✅ Yes                    |
| Result<T, E>    | ✅ Yes                    |
| Structs         | ✅ Yes (Field Matching)   |
| Tuples          | ✅ Yes (Pattern Matching) |
| Arrays & Slices | ✅ Yes (Pattern Matching) |

These Rust features support `if let` and `let else`, making pattern matching concise and readable.

The reason `d.status == _blah` **did not work** without dereferencing `_blah` is because of **Rust’s pattern matching behavior and ownership rules**.

---

## **Understanding the Problem**

Your code:

```rust
if let Device {status: _blah, ..} = d {
    if d.status == *_blah {
        println!("Device is active");
    }
}
```

### **Key Issue:**

1. In the `if let` statement:

   ```rust
   if let Device {status: _blah, ..} = d
   ```

   - `_blah` **becomes a new owned `String`** instead of borrowing `d.status`.
   - This means `_blah` is a **completely new String**, separate from `d.status`.

2. In the comparison:
   ```rust
   if d.status == *_blah
   ```
   - `d.status` is a **`String`** (`String` is an owned type).
   - `_blah` is also a `String`, **but owned by the `if let` scope**.
   - `*_blah` **dereferences** `_blah` to get a `&str`, which allows comparison with `d.status`.

---

### **Why is Dereferencing Needed?**

Rust implements `PartialEq<&str>` for `String`, meaning:

```rust
String == &str   // This is allowed
```

But `String == String` **requires borrowing or ownership transfer**, which Rust doesn't do automatically in this case.

So:

- `d.status == _blah` → ❌ Error (both are `String`s but ownership issues arise)
- `d.status == *_blah` → ✅ Works (dereferences `_blah` to `&str` for comparison)

---

### **Correcting the Code**

A **better approach** is to match `status` **by reference**:

```rust
fn isOnline(d: &Device) {
    if let Device { status: ref s, .. } = d {
        if s == "online" {
            println!("Device is active");
        }
    }
}
```

✅ Here, `ref s` makes `s` a `&String`, allowing direct comparison.

## Understanding `str`, `&str`, and `String` in Rust

| Feature                   | `str` ❌ (Not Usable Directly) | `&str` ✅ (String Slice)                                           | `String` ✅ (Owned String)                |
| ------------------------- | ------------------------------ | ------------------------------------------------------------------ | ----------------------------------------- |
| **Memory Location**       | Unknown (Cannot Exist Alone)   | **Stack** (or inside `String`)                                     | **Heap** (Dynamically Allocated)          |
| **Size at Compile Time?** | ❌ No (Unsized Type)           | ✅ Yes (Reference has fixed size)                                  | ❌ No (Can grow/shrink)                   |
| **Growable?**             | ❌ No                          | ❌ No (Immutable Slice)                                            | ✅ Yes (Owned, Can Grow)                  |
| **Modifiable?**           | ❌ No                          | ❌ No (Immutable Slice)                                            | ✅ Yes (Mutable if `mut`)                 |
| **Ownership?**            | ❌ No (Cannot Exist Alone)     | ❌ No (Borrowed)                                                   | ✅ Yes (Owns Memory)                      |
| **Use Case**              | **Not Usable Directly**        | Immutable borrowed string (e.g., string literals, function params) | Mutable, growable string for dynamic text |

---

## 🚀 Analogy: `str` vs `&str` vs `String`

Think of **a book** 📖:

- `str` → **The concept of a book** (you can’t touch or own it).
- `&str` → **A borrowed copy of the book** (you can read but not edit).
- `String` → **Your personal book** (you own it, can modify or throw away).

---

### 🔄 **Common Conversions**

```rust
let s1: &str = "Hello, Rust!";  // String slice (borrowed)
let s2: String = String::from("Hello, Rust!"); // Owned String

// Convert &str → String
let s3 = s1.to_string();
let s4 = "Another way".to_owned();

// Convert String → &str
let s5: &str = &s2;  // Borrowing
let s6: &str = &s2[..];  // Slicing

// Convert String → str (only temporarily)
fn takes_str(s: &str) { println!("{}", s); }
takes_str(&s2); // Works fine!
```

## TL;DR:

1. Use if let when handling one case while ignoring others.
2. Use let else when you must handle the failure case before continuing. 🚀

## https://chatgpt.com/share/67ce2fa4-586c-8005-a5b3-555fe0ce40d5

## Packages, Crates and Modules

A related concept is scope: the nested context in which code is written has a set of names that are defined as “in scope.” When reading, writing, and compiling code, programmers and compilers need to know whether a particular name at a particular spot refers to a variable, function, struct, enum, module, constant, or other item and what that item means. You can create scopes and change which names are in or out of scope. You can’t have two items with the same name in the same scope

1. **Packages**: A Cargo feature that lets you build, test, and share crates
2. **Crates**: A tree of modules that produces a library or executable
3. **Modules and use**: Let you control the organization, scope, and privacy of paths
4. **Paths**: A way of naming an item, such as a struct, function, or modules

## Note:

1. **Cargo follows a convention that src/main.rs is the crate root of a binary crate with the same name as the package.**
2. **Likewise, Cargo knows that if the package directory contains src/lib.rs, the package contains a library crate with the same name as the package, and src/lib.rs is its crate root.**
3. **Cargo passes the crate root files to rustc to build the library or binary.**

## Module Cheat Sheet

**Case 1** : If module inside the crate root :

```rust
// main.rs

pub mod garden;

fn main() {

}
```

Go to `src/garden.rs` or `src/garden/mod.rs` for the garden definition.

**case 2** : If the module has sub modules

```rust
// main.rs
use crate::garden::vegetables:asparagus; // Note asparagus is a type and not a module in this example
pub mod garden;

fn main() {

}
```

Go to `src/garden/vegetables.rs` or `src/garden/vegetables/mod.rs` for the asparagus definition
