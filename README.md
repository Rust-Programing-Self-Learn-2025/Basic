# Basic Rust

# Introduction

### **What is Rust?**
Rust is a **systems programming language** designed for **performance, safety, and concurrency**. It was created by Mozilla and is now widely used in industries like **web development, game engines, operating systems, and blockchain**.  

### **Why Use Rust? Key Advantages**  

#### **1. Memory Safety Without Garbage Collection**  
- Rust **prevents memory leaks, buffer overflows, and null pointer exceptions** at **compile time** (unlike C/C++).  
- Uses **ownership, borrowing, and lifetimes** to manage memory without a garbage collector (like Go/Java).  

#### **2. Blazing Fast Performance**  
- **Zero-cost abstractions**: High-level code compiles to efficient machine code (similar to C/C++).  
- **No runtime overhead**: Ideal for embedded systems, game engines, and high-performance applications.  

#### **3. Fearless Concurrency**  
- Rust’s compiler **detects data races before runtime**, making multithreaded code safer.  
- Enables **parallel programming** without crashes or undefined behavior.  

#### **4. Modern Tooling (Cargo & Crates.io)**  
- **Cargo**: Built-in package manager (`cargo build`, `cargo run`, `cargo test`).  
- **Crates.io**: Huge ecosystem of libraries (e.g., `serde` for JSON, `tokio` for async).  

#### **5. Cross-Platform Support**  
- Compiles to **Windows, Linux, macOS, WebAssembly (WASM), and embedded systems**.  
- Used in **WebAssembly** for browser-based high-speed apps.  

#### **6. Used by Big Tech**  
- **Microsoft**: Rewriting Windows components in Rust for security.  
- **Google**: Android now supports Rust for OS-level code.  
- **Meta (Facebook)**: Uses Rust in blockchain (Diem) and backend services.  

---

### **Example: Memory Safety in Rust**  
```rust
fn main() {
    let mut data = vec![1, 2, 3];
    let first = &data[0]; // Immutable borrow
    data.push(4); // Error! Cannot mutate while borrowed.
    println!("{}", first);
}
```
**Compiler Error**: Prevents **use-after-free** bugs at compile time.  

---

### **When to Use Rust?**  
✅ **System programming** (OS, drivers).  
✅ **High-performance web servers** (e.g., Actix, Rocket).  
✅ **Game engines** (e.g., Bevy).  
✅ **Blockchain/Smart Contracts** (Solana, Polkadot).  
✅ **Embedded/IoT devices**.  

---

### **Conclusion**  
Rust combines **C/C++-level speed** with **Python/Java-like safety**, making it perfect for **critical software where crashes are unacceptable**.  

🚀 **Get Started**:  
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh  
cargo new my_project  
```  

🦀 **"Rust is for those who crave speed without the crashes."**

## Rust installer and version management tool 

### Windows Subsystem for Linux
```
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### Is Rust up to date?

```sh
    rustup update
```

### Rust version check

```sh
    rustc --version
```

### Cargo: the Rust build tool and package manager

When you install Rustup you’ll also get the latest stable version of the Rust build tool and package manager, also known as Cargo. Cargo does lots of things:

- build your project with 
```sh
    cargo build
```
- run your project with 
```sh
    cargo run
```
- test your project with 
```sh
    cargo test
```
- build documentation for your project with 
```sh
    cargo doc
```
- publish a library to crates.io with 
```sh
    cargo publish
```

To test that you have Rust and Cargo installed, you can run this in your terminal of choice:

```sh
cargo --version
```

Here’s a concise English explanation of **Cargo** (for your self-learning or README file):

---

### **What is Cargo?**  
**Cargo** is Rust’s official **build system** and **package manager**. It automates compiling code, managing dependencies, running tests, and more—similar to `npm` (JavaScript) or `pip` (Python), but optimized for Rust’s needs.

---

### **Key Features of Cargo**  
1. **Project Creation**  
   - `cargo new my_project` generates a new project with:  
     - `src/main.rs` (entry file).  
     - `Cargo.toml` (config file for metadata/dependencies).  

2. **Dependency Management**  
   - Add packages (*crates*) to `Cargo.toml`:  
     ```toml
     [dependencies]
     serde = "1.0"  # Example: JSON serialization crate
     ```  
   - Run `cargo build` to install dependencies.  

3. **Building & Running**  
   - `cargo build`: Compiles code (debug mode).  
   - `cargo build --release`: Optimized compilation (for production).  
   - `cargo run`: Compiles + executes the program.  

4. **Testing**  
   - `cargo test`: Runs all tests in the project.  

5. **Documentation**  
   - `cargo doc --open`: Generates and opens HTML docs for your code.  

---

### **Example Workflow**  
1. Create a project:  
   ```bash
   cargo new hello_rust
   cd hello_rust
   ```  
2. Edit `src/main.rs`:  
   ```rust
   fn main() {
       println!("Hello, Cargo!");
   }
   ```  
3. Run it:  
   ```bash
   cargo run
   ```  
   Output:  
   ```
   Hello, Cargo!
   ```  

---

### **Why Use Cargo?**  
- **Simplicity**: Handles complex tasks (dependency resolution, compilation) effortlessly.  
- **Ecosystem**: Integrates with [crates.io](https://crates.io) (Rust’s package registry).  
- **Safety**: Ensures dependency compatibility and secure builds.  

Ideal for beginners and large-scale projects alike! 🦀  

--- 

**Tip**: Include this in your README to explain your project’s setup. Example:  
```markdown
## Build & Run  
1. Install [Rust](https://rust-lang.org).  
2. Clone this repo: `git clone <repo_url>`.  
3. Run: `cargo run`.  
```