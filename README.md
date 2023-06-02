# Minecraft Beta Chunk Generation

This project aims to accurately replicate and preserve the chunk generation of Minecraft's beta version 1.7.3 by refactoring the source of the original server jar into a Rust binary. The resulting executable will allow for the generation of a predictable square of chunks for both the overworld and nether dimensions.

## Project Overview

The Rust translation is derived from one of my private repositories, in which the terrain generation was already isolated through extensive refactoring. While this Java repository is mostly complete, it is not intended for public release.

One key motivation for such a conversion is leveraging the more explicit memory management features of Rust to speed up the chunk creation process. Achieving an idiomatic translation and ensuring that the behavior of the new code precisely mirrors the original version requires not only a deep understanding of the underlying algorithms but also careful attention to its logic.

Since the Java version already provides the core functionality, this public rendition, along with its additionally [planned features](#planned-features), primarily serves as a means for me to further explore the Rust programming language in a more practical context and to properly document the process for anyone who is interested in it as well. Therefore, I am progressing with the project at a pace that suits my schedule and ensures the quality of the codebase.

## Current Progress

The overall project structure for generating chunks is complete. A `world` object holds general information, such as a `seed`, and is responsible for creating both the `overworld` and `nether` dimensions. Each dimension has its own `chunk manager` which maintains a specific implementation of any given `chunk provider`.

Java's `Random` class has been successfully translated and now provides additional functions for skipping values. The `noise` algorithms, which can be composed as `octaves`, have been fully developed, completing the required randomization features. Possible next steps are as follows:

- Create an abstraction for saving data in the NBT format
- Bundle NBT data correctly into a region file
- Implement block types that can be generated
- Begin translating the chunk generation algorithms
- Write unit tests for randomization features

## Planned Features

After completing the Rust translation, the following features are planned to be implemented in order to make generated worlds automatically compatible with newer Minecraft versions:

- Automatic generation of chunks in the anvil format to apply biomes from release version 1.6.4
- Refactoring and translation of end dimension terrain generation from release version 1.8.9
- Feature to prune each dimension back to its original creation size
- Options to remove bedrock and specific chunk contents depending on the dimension
- Creation of end portal structures and removal of obsidian pillars
- Adjustments to border chunks with the void biome introduced in release version 1.9
- Option to automatically generate and update chunks to be compatible with release version 1.13
- Capability to generate overworld terrain from release version 1.17.1 on one side of the border chunks
- Addition of the chunk height increase without spawning new caves below zero for release version 1.18

## Contribution

Contributions to this project are very welcome. If you are familiar with Rust and would like to help make the code more idiomatic or performant, your input would be greatly appreciated. Please feel free to open a pull request with your changes. 
