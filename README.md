# rusty_python
Getting familiar with Py03


### Building the module
``` zsh
% source venv/bin/activate
% maturin develop --release
```
### Using the module

```python
import rusty_python as rp

result = rp.sum_as_string(10,8)
```
