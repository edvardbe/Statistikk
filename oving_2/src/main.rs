fn main() {

    let prob_bensin = 0.449396;
    let prob_diesel = 0.479752;

    println!("Sannsynligheten for at en tilfeldig valgt bil er en bensinbil er: {}", conditional_probability(prob_bensin, prob_diesel));

    println!("Sannsynligheten for at en tilfeldig valgt bil er en dieselbil er: {}", conditional_probability(0.2, 0.5));
}


fn conditional_probability(prob_a: f64, prob_b: f64) -> f64 {
    let prob_a_and_b = prob_a * prob_b;
    let prob_result = prob_a_and_b / prob_a;
    return prob_result;
}