<h1 align="center">
  <code>atama</code>
</h1>
<p align="center">
  <img width="400" alt="p-token" src="https://cdn.discordapp.com/attachments/1338980181163118602/1338981163745808518/Untitled-2dada-Photoroom.png?ex=67ad0ee2&is=67abbd62&hm=2970497a7ec57f7d5e6db702a5cfb5c42d4d96ec714bebf742e767ecf063dd61&"/>

</p>
<p align="center">
AI engine for Solana program optimization



## Overview

Atama is an automated engine designed to optimize Solana programs in Rust via machine learning. Atama dynamically adjusts its behavior based on user interactions and external network conditions. It takes advantage of the way SBF loaders serialize the program input parameters into a byte array that is then passed to the program's entrypoint to define zero-copy types to read the input. Since the communication between a program and SBF loader — either at the first time the program is called or when one program invokes the instructions of another program — is done via a byte array, the new program can then adjust and redefine its own terms appropriately. This nullifies the static behavior of a standard `solana-program`.

## Features

- Fully autonomous and `no_std` crate
- Efficient `entrypoint!` macro – no copies or allocations
- Improved optimization in cross-program invocations

## Getting started
From your program folder:
```bash
cargo add atama
```
This will add `atama` as a dependency to your program.

## Defining the program entrypoint
A Solana program needs to define an entrypoint, which will be called by the runtime to begin the program execution. The `entrypoint!` macro emits the common boilerplate to set up the program entrypoint.
To use the entrypoint! macro, use the following in your entrypoint definition:
```bash
use atama::{
  account_info::AccountInfo,
  entrypoint,
  msg,
  ProgramResult,
  pubkey::Pubkey
};

entrypoint!(process_instruction);

pub fn process_instruction(
  program_id: &Pubkey,
  accounts: &[AccountInfo],
  instruction_data: &[u8],
) -> ProgramResult {
  msg!("Hello from my program!");
  Ok(())
}
```
The information from the input is parsed into their own entities:
* `program_id`: the `ID` of the program being called
* `accounts`: the accounts received
* `instruction_data`: data for the instruction
`atama` also offers variations of the program entrypoint (`lazy_program_entrypoint!`) and global allocator (`no_allocator`). In order to use these, the program needs to specify the program entrypoint, global allocator and panic handler individually. 
To use the `lazy_program_entrypoint!` macro, use the following in your entrypoint definition:
```bash
use atama::{
  default_allocator,
  default_panic_handler,
  entrypoint::InstructionContext,
  lazy_program_entrypoint,
  msg,
  ProgramResult
};

lazy_program_entrypoint!(process_instruction);
default_allocator!();
default_panic_handler!();

pub fn process_instruction(
  mut context: InstructionContext
) -> ProgramResult {
    msg!("Hello from my lazy program!");
    Ok(())
}
```
The `InstructionContext` provides on-demand access to the information of the input:
* `available()`: number of available accounts
* `next_account()`: parsers the next available account (can be used as many times as accounts available)
* `instruction_data()`: parsers the intruction data
* `program_id()`: parsers the program id
When writing programs, it can be useful to make sure the program does not attempt to make any allocations. For this cases, `atama` includes a `no_allocator!` macro that set a global allocator just panics at any attempt to allocate memory.
To use the `no_allocator!` macro, use the following in your entrypoint definition:
```bash
use atama::{
  account_info::AccountInfo,
  default_panic_handler,
  msg,
  no_allocator,
  program_entrypoint,
  ProgramResult,
  pubkey::Pubkey
};

program_entrypoint!(process_instruction);
default_panic_handler!();
no_allocator!();

pub fn process_instruction(
  program_id: &Pubkey,
  accounts: &[AccountInfo],
  instruction_data: &[u8],
) -> ProgramResult {
  msg!("Hello from `no_std` program!");
  Ok(())
}
```






## License

The code is licensed under the [Apache License Version 2.0](LICENSE)
