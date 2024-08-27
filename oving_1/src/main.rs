

fn average(numbers: &[f64]) -> f64 {
    let mut sum = 0.0;
    for number in numbers {
        sum += number;
    }
    return sum / numbers.len() as f64
}

fn median(number_m: &[f64]) -> f64 {
    let mut numbers = number_m.to_vec();
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
    return (sum / (numbers.len() - 1) as f64).sqrt()
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

fn correlation_coefficient(numbers1: &[f64], numbers2: &[f64]) -> f64 {
    let avg1 = average(numbers1);
    let avg2 = average(numbers2);
    let mut sum = 0.0;
    for i in 0..numbers1.len() {
        sum += (numbers1[i] - avg1) * (numbers2[i] - avg2);
    }
    return (sum / ((numbers1.len() - 1) as f64 * standard_deviation(numbers1) * standard_deviation(numbers2)))
}

fn linear_regression(numbers1: &[f64], numbers2: &[f64]) -> (f64, f64) {
    let avg1 = average(numbers1);
    let avg2 = average(numbers2);
    let mut sum1 = 0.0;
    let mut sum2 = 0.0;
    for i in 0..numbers1.len() {
        sum1 += (numbers1[i] - avg1) * (numbers2[i] - avg2);
        sum2 += (numbers1[i] - avg1).powi(2);
    }
    let slope = sum1 / sum2;
    let intercept = avg2 - slope * avg1;
    return (slope, intercept)
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
    

    let avg_heart = vec![159.0, 170.0, 155.0, 160.0, 159.0];

    let calories = vec![381.0, 343.0, 282.0, 253.0, 299.0];
    println!("Correlation coefficient: {}", correlation_coefficient(&avg_heart, &calories));

    println!("Linear regression: {:?}", linear_regression(&avg_heart, &calories));

}