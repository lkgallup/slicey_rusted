# Geometry
The geometry module in slicey handles the necessary IO for working with geometry files such as STL files. Currently, only STL files are supported but we should plan on supporting 3mf files, among others.

The main hook for this (when working with STL files) is the following

```rust
# extern crate slicey;
# use slicey::geometry::STLMesh;
# use std;
pub fn main() {
    let file_name = "./slicey/book/src/3DBenchy.stl";
    // let mesh = STLMesh::new(file_name.to_string());
    // println!("mesh = {:?}", mesh);
}
```
