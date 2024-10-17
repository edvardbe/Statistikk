fn least_squares(x: &Vec<f64>, y: &Vec<f64>) -> (f64, f64) {
    let n = x.len();
    let sum_x = x.iter().sum::<f64>();
    let sum_y = y.iter().sum::<f64>();
    let sum_x_squared = x.iter().map(|x| x.powi(2)).sum::<f64>();
    let sum_xy = x.iter().zip(y.iter()).map(|(x, y)| x * y).sum::<f64>();

    let a = (n as f64 * sum_xy - sum_x * sum_y) / (n as f64 * sum_x_squared - sum_x.powi(2));
    let b = (sum_y - a * sum_x) / n as f64;

    (a, b)
}

fn linear_variance_estimate(n: usize, x: &Vec<f64>,y:  &Vec<f64>, a: f64, b: f64) -> f64 {
    let sum_squared_residuals = x.iter().zip(y.iter()).map(|(x, y)| (y - a * x - b).powi(2)).sum::<f64>();
    sum_squared_residuals / (n - 2) as f64
}

fn main() {
    let x = vec![123.0, 162.0, 120.0, 202.0, 95.0, 120.0];
    let y = vec![25.3, 30.0, 25.7, 40.9, 18.4, 20.3];

    let (a, b) = least_squares(&x, &y);
    println!("a: {}, b: {}", a, b);
    println!("Variance estimate: {}", linear_variance_estimate(x.len(), &x, &y, a, b));
}