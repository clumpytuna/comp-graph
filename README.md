# Computational graph lib
A tiny computational graph library. Imlpements a dynamically built DAG with simple operations such as Add, Mul, Pow, etc. 

### Example usage
Below is an example showing possible supported operations:

```rust,skt-template
let mut x1 = create_input();
let mut x2 = create_input();
let mut x3 = create_input();

let mut graph = add(
    x1.clone(),
    mul(
        x2.clone(),
        sin(
            add(
                x2.clone(),
                pow_f32(
                    x3.clone(),
                    3f32
                )
            )
        )
    ),
);

x1.set(1f32);
x2.set(2f32);
x3.set(3f32);


let mut result = graph.compute();

```
        
