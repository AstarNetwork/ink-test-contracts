# Test Contracts
This repository contains the ink! contracts used in Astar Runtime/XCM testing fixtures. 

## Build Contracts
```
cargo build
```

The [build script](build.rs) will automatically all contracts present in [contracts](./contracts/) directory
and place the build artifacts in `fixtures` directory.