# baby-step

Implementation in Rust of the baby-step giant-step algorithm for finding
discrete logarithms.

## Usage

```
$ cargo run <order> <generator> <input>
```

Where ```order``` is the order of the cyclic group, ```generator``` is a
generator of the group, and ```input``` is the member of the group we
want the discrete logarithm of.

The final result x will be such that

```
generator^x mod order = input
```

