# 🔧 Repository manager

In some cases it's better for me to used binary compiled on my own from source code. Problem with this approach is that I can't easly track if there are any udpates, and I need to use sets of difrrent commands based on repository. Goal of this project is to provide such a tool that I could track selected repositories stored on my local machine with and massivle update and install selected ones.

# ⌛️ Stage of project

Project has already implemented core functionality but there is stuff to do to make it usable.

-   [ ] script for installing build in system
-   [ ] refactor commands modules (minimize repetitive code)
-   [ ] make possible adding multiple repositories in single command
-   [ ] make possible naming multiple repositories in update, install and build commands
-   [ ] check command without arg should check all repositories and list ready to update ones
-   [ ] all listing should sort repositories in alphabetical order
-   [ ] add numering for repositories listing
-   [ ] allow providing nums or ranges instead of full names in update, install and build function
-   [ ] find a way to determine which build needs to be installed after update
-   [ ] update, install and build commands without any arg should perform action for all repos that need it
-   [ ] in real time displaying stdout in install, update and build commands
-   [ ] change way how commits are listing in check command for selected repostiory
-   [ ] add handling errors for commands

This project is my first one in rust so if anyone will ever boder to look at code and see something awful I would be gratefull for letting me know how things should be.

# 📂 Compile

```
cargo run [options to pass] # for test
cargo build --release # for release build
```

# 💻 Usage

To see commands use `repman help`.

Compiled version will store config files for repository in `~/.repman` but if u debug with `cargo run` all actions will take place in `${PROJECT_DIRECTORY}/target/.repman`.
