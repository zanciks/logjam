Logjam is a program for collecting and using logs from game files in use with QA.

TODO:
1. Finish plugin management
2. Move all created plugins to their own places, probably making this into a cargo workspace
3. ~~Create better build script for generating portable executable with plugins (Makefile?)~~ done with cargo make. 
4. Create GUI, possibly leaving it up to each plugin to provide with templates
5. Create example plugins for games with public bug logging (Minecraft for example)
6. Create and link program with browser plugins, to send data to forms (Jira, Github, ADO)

To create a portable version of logjam, run
`cargo install --force cargo-make` followed by `cargo make`
