# 🦀 Rust Learning Tasks

This repo contains small Rust projects to practice core Rust concepts.

---

## ✅ Task 1: Ownership Basics

📁 Folder: `task1_ownership`

- Learn variable ownership, borrowing, and memory safety.
- Minimal examples to explain how Rust manages data.

---

## ✅ Task 2: To-Do List CLI

📁 Folder: `task2_todolist`

- Console UI with color.
- Enum + Structs for modeling tasks.
- Interactive Add/List/Complete/Delete options.

---

## ✅ Task 3: Bank System using Traits

📁 Folder: `task3_banksystem`

- Trait for `Account` behavior.
- `Result`-based error handling for deposit and withdraw.
- Uses `impl` blocks, formatted display, and a transfer function.
- Demonstrates trait implementation, method calling, and ownership with mutable references.

💡 Features:
- Create and manage multiple accounts.
- Deposit and withdraw with validation.
- Transfer funds between accounts.
- Use of `Display` and custom error messages.

---

## ✅ Task 4: Password Vault CLI App

📁 Folder: `pass-vault`

- Command-line password manager to store and retrieve login credentials.
- Uses modular code split between `main.rs` and a separate `pentry.rs`.
- Stores data in `passwords.json` using `serde` for JSON serialization.
- Supports adding, listing, and searching saved entries.
- Enhanced terminal interface with ASCII art header.

💡 Features:
- Store `service`, `username`, and `password` per entry.
- Persistent local storage (plain JSON).
- Simple search functionality by service or username.
- CLI interface menu using standard input/output.
- Clean Rust module and struct-based design.

