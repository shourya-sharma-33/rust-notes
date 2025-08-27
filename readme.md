# 🦀 Rust Notes

This repository contains all of my **Rust programming language notes** and practice code.  
I’m using **Cargo** to run my programs, and you can find your own way to run them if you want — that’s up to you.  
I’m just keeping these here as part of my personal learning journey.

### 🔹 How I Run My Code
- To run the **main file**:
  ```bash
  cargo run
````

* To run any **example file**:

  ```bash
  cargo run --example <file_name>
  ```

That’s it. These are my personal notes, experiments, and small programs.
I’m not maintaining this repo for others, so feel free to explore or run the code your own way. 🙂

```

---

Now, about your other question:  
> “If I'm gonna make a folder inside the `examples` directory, how will I do it?”  

Yes, you **can** create folders inside the `examples` directory, but by default, Cargo does **not** automatically recognize nested example files.  

There are **two options**:  

### **Option 1 — Keep all example `.rs` files flat** ✅ *(Recommended)*
- Put all `.rs` files **directly** under `examples/`
- Example:
```

examples/
├── hello.rs
├── loops.rs
├── ownership.rs

````

Then you can run:
```bash
cargo run --example hello
````

### **Option 2 — Use folders inside `examples/`** *(Needs extra config)*

* Example structure:

  ```
  examples/
  ├── basics/
  │   ├── variables.rs
  │   ├── loops.rs
  ├── advanced/
  │   ├── ownership.rs
  ```

But then you **cannot** just do:

```bash
cargo run --example basics/variables
```

Cargo **won’t allow** that by default.

To make nested folders work, you’d need to **turn each subfolder into its own Cargo binary target** by editing `Cargo.toml` — I can show you how if you want.

