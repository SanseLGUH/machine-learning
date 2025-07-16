use rand::Rng;

fn bounded_rlu(result: f64) -> f64 {
    result.clamp(0.0, 1.0)
}

fn final_calculation(pred: f64, bias: f64) -> f64 {
    pred + bias
}

fn truncate_to_one_decimal(x: f64) -> f64 {
    (x * 10.0).floor() / 10.0
}

fn rules(result: f64) -> f64 {
    let distance = (result - 0.5).abs();

    if distance < 0.01 {
        println!("✅ Perfect HIT at {:.1}", result);
        return 1.0;
    }

    // Graded reward between 0.0 and 1.0 based on closeness to 0.5
    1.0 - (distance * 2.0).min(1.0)
}

fn mse(y_true: f64, y_pred: f64) -> f64 {
    (y_true - y_pred).powi(2)
}

fn main() {
    let mut pred: f64 = 0.1;
    let mut bias: f64 = 0.4;

    let mut velocity_pred: f64 = 0.0;
    let mut velocity_bias: f64 = 0.0;

    let learning_rate: f64 = 0.2;
    let momentum: f64 = 0.9;

    let mut step: u64 = 0;

    loop {
        // Forward pass
        let output = bounded_rlu(final_calculation(pred, bias));
        let rounded = truncate_to_one_decimal(output);
        let expected = rules(rounded);

        // Compute loss & gradient
        let loss = mse(expected, output);
        let grad = output - expected;

        // Momentum update
        velocity_pred = momentum * velocity_pred - learning_rate * grad;
        velocity_bias = momentum * velocity_bias - learning_rate * grad;

        pred += velocity_pred;
        bias += velocity_bias;

        // Clamp to reasonable range
        pred = pred.clamp(-1.0, 1.0);
        bias = bias.clamp(-1.0, 1.0);

        // Debug output
        println!(
            "[{}] pred = {:.4}, bias = {:.4}, output = {:.4}, rounded = {:.1}, expected = {:.2}, loss = {:.5}",
            step, pred, bias, output, rounded, expected, loss
        );

        step += 1;
    }
}
