<!-- PROJECT LOGO & BADGES -->
<div align="center">
<!-- 
<br />
<a href="https://github.com/the-loki-lang/loki">
<img src="https://unsplash.com" alt="Logo" width="80" height="80">
</a>
-->

# Loki

A simple programming language *for you, for all of us.*

[Explore the docs](https://the-loki-lang.github.io/loki/)

[Report Bug](https://github.com/the-loki-lang/loki/issues)

[Request Feature](https://github.com/the-loki-lang/loki/issues)

</div>

> [!IMPORTANT]
> This language is still under development so the
> syntax may be changed in the future.

<!-- TABLE OF CONTENTS -->
<details>
<summary>Table of Contents</summary>
<ol>
<li><a href="#about-the-project">About The Project</a></li>
<li><a href="#getting-started">Getting Started</a></li>
<li><a href="#usage">Usage</a></li>
<li><a href="#roadmap">Roadmap</a></li>
<li><a href="#contributing">Contributing</a></li>
<li><a href="#license">License</a></li>
</ol>
</details>

## About The Project

`loki` is an open-source programming language designed to be simple, easy and fun.

## Getting Started

Follow these simple steps to set up a local copy of the project for development and testing.

### Installation

```zsh
git clone https://github.com/the-loki-lang/loki/
cd loki
```
## Usage

### Run the REPL:

- Generate the executable
```bash
cargo build --release
```

- Run the repl
```bash
target/release/loki
```

## Roadmap

- [x] Build the Lexer
- [x] Build the basic Parser
    - [x] Aritmethic Operation
    - [x] Unary Operation
    - [x] Variable Declaration and Assignment
    - [x] If and Else statement
    - [x] While loop and For loop
    - [ ] Builtin Array and String
    - [ ] Function Declaration and Function Call
- [x] Build the basic Semantic Analyzer
- [ ] Build the Intermidiate Representation
- [ ] Build the Compiler

## Contributing

Contributions make the open-source community an amazing place to learn, inspire, and create. Any contributions you make are **greatly appreciated**.

1. Fork the Project
2. Create your Feature Branch (`git checkout -b feature/AmazingFeature`)
3. Commit your Changes (`git commit -m 'Add some AmazingFeature'`)
4. Push to the Branch (`git push origin feature/AmazingFeature`)
5. Open a Pull Request

## License

Distributed under the **MIT License**. See `LICENSE.txt` for more information.
