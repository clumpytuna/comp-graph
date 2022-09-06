use crate::operations::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_add() {
        let mut x1 = create_input();
        let mut x2 = create_input();

        x1.set(1f32);
        x2.set(2f32);

        let mut graph = add(x1.clone(), x2.clone());
        assert_eq!(graph.compute(), 3f32);
    }

    #[test]
    fn complex_add_and_recalculate() {
        let mut x1 = create_input();
        let mut x2 = create_input();
        let mut x3 = create_input();

        x1.set(1f32);
        x2.set(2f32);
        x3.set(3f32);

        let mut graph = add(add(x1.clone(), x2.clone()), x3.clone());
        assert_eq!(graph.compute(), 1f32 + 2f32 + 3f32);

        x1.set(2f32);
        x2.set(3f32);
        x3.set(4f32);

        assert_eq!(graph.compute(), 2f32 + 3f32 + 4f32);
    }

    #[test]
    fn add_same() {
        let mut x1 = create_input();

        x1.set(1f32);

        let mut graph = add(x1.clone(), add(x1.clone(), add(x1.clone(), x1.clone())));

        assert_eq!(graph.compute(), 1f32 + 1f32 + 1f32 + 1f32);
    }

    #[test]
    fn example() {
        let mut x1 = create_input();
        let mut x2 = create_input();
        let mut x3 = create_input();

        let mut graph = add(
            x1.clone(),
            mul(x2.clone(), sin(add(x2.clone(), pow_f32(x3.clone(), 3f32)))),
        );
        x1.set(1f32);
        x2.set(2f32);
        x3.set(3f32);

        let mut result = graph.compute();
        result = round(result, 5);
        assert_eq!(round(result, 5), -0.32727);

        x1.set(2f32);
        x2.set(3f32);
        x3.set(4f32);
        result = graph.compute();
        result = round(result, 5);
        assert_eq!(round(result, 5), -0.56656);
    }
}
