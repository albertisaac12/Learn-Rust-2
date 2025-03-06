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
```
