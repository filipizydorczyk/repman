This project is not maintained anymore. It was more of a first rust approach than actuall project and I found creating own pipelines for that and creating own arch repository better solution.

---

# 🔧 Repository manager

In some cases it's better for me to used binary compiled on my own from source code. Problem with this approach is that I can't easly track if there are any udpates, and I need to use sets of difrrent commands based on repository. Goal of this project is to provide such a tool that I could track selected repositories stored on my local machine with and massivle update and install selected ones.

# ⌛️ Stage of project

Project has already implemented core functionality but there is stuff to do to make it usable.

-   [x] ~~script for~~ installing build in system
-   [ ] refactor commands modules (minimize repetitive code)
-   [x] make possible adding multiple repositories in single command
-   [x] make possible naming multiple repositories in update, install and build commands
-   [x] check command without arg should check all repositories and list ready to update ones
-   [ ] all listing should sort repositories in alphabetical order
-   [ ] add numering for repositories listing
-   [ ] allow providing nums or ranges instead of full names in update, install and build function
-   [ ] find a way to determine which build needs to be installed after update
-   [x] update, install and build commands without any arg should perform action for all repos that need it
-   [x] in real time displaying stdout in install, update and build commands
-   [x] change way how commits are listing in check command for selected repostiory
-   [x] add handling errors for commands

This project is my first one in rust so if anyone will ever boder to look at code and see something awful I would be gratefull for letting me know how things should be.

# 📂 Compile

```
cargo run [options to pass] # for test
cargo build --release # for release build
```

If you want to install release build on your system run `cargo install --path .`. It will install binary in `$HOME/.cargo/bin` so if you want to actually use it add it to your `PATH`.

# 💻 Usage

To see commands use `repman help`.

Compiled version will store config files for repository in `~/.repman` but if u debug with `cargo run` all actions will take place in `${PROJECT_DIRECTORY}/target/.repman`.
