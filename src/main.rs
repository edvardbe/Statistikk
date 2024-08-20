

fn average(numbers: &[f64]) -> f64 {
    let mut sum = 0.0;
    for number in numbers {
        sum += number;
    }
    return sum / numbers.len() as f64
}

fn median(numberM: &[f64]) -> f64 {
    let mut numbers = numberM.to_vec();
    numbers.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let mid = numbers.len() / 2;
    if numbers.len() % 2 == 0 {
        return (numbers[mid - 1] + numbers[mid]) / 2.0
    } else {
        return numbers[mid]
    }
}

fn variance(numbers: &[f64]) -> f64 {
    let avg = average(numbers);
    let mut sum = 0.0;
    for number in numbers {
        sum += (number - avg).abs();
    }
    return sum / (numbers.len() - 1) as f64
}

fn standard_deviation(numbers: &[f64]) -> f64 {
    let avg = average(numbers);
    let mut sum = 0.0;
    for number in numbers {
        sum += (number - avg).powi(2);
    }
    return (sum / numbers.len() as f64).sqrt()
}

fn average_amount(numbers: &[f64]) -> f64 {
    let mut sum = 0.0;
    let mut count = 1.0;
    let mut amount = 0.0;
    for number in numbers {
        sum += number*count;
        count += 1.0;
        amount += number;
    }
    return sum / amount;
}
fn main() {
    let mut numbers = vec![0.1109, 0.0951, 0.0862, 0.1015, 0.1579, 0.0981, 0.0925, 0.0987];
    

    println!("Average: {}", average(&numbers));
    println!("Median: {}", median(&numbers));
    numbers.retain(|&x| x <= 0.12);

    println!("Average: {}", average(&numbers));
    println!("Median: {}", median(&numbers));

    numbers = vec![0.18, 0.93, 0.66, 0.21, 0.87, 0.38];

    println!("Variance: {}", variance(&numbers));

}