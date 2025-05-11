# baml_stream_test

## BAML Submodule

This project includes the [BAML](https://github.com/BoundaryML/baml) repository as a git submodule in the `baml` directory.

### Cloning the repository with submodules

When cloning this repository, use the following command to ensure submodules are initialized:

```sh
git clone --recurse-submodules <repository-url>
```

If you have already cloned the repository, initialize and update submodules with:

```sh
git submodule update --init --recursive
```

### BAML Directory

The BAML codebase is located in the `baml` directory at the root of this project.
