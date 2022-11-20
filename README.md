# rusty_python
## Getting familiar with Py03

[Documentation](https://pyo3.rs/v0.17.3/)

### Building the module
---
``` zsh
% python -m venv venv
% source venv/bin/activate
% pip install maturin
% maturin develop --release
```
### Using the module

```python
>>> import rusty_python as rp
>>> rp.calculate_volume("stl/Stanford_Bunny.stl")
127622.1796875
```
---
## Functions

| Name | Output | Description |
| --- | :---: | --- |
| calculate_volume( `stl_path` ) | float | Volume of mesh |
| calculate_surface_area( `stl_path` ) | float | Surface area of mesh |
| calculate_centroid( `stl_path` ) | [float, float, float] | AABB Centroid coordinates |

*This library uses very basic calculations and does not do any sort of validation for watertightness, manifold meshes, etc. and is purely for demonstration purposes.*