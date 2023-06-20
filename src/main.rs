use rand::Rng;
mod mathi;

fn generate_random_coordinates(num_pairs: usize, x_min: i32, x_max: i32, y_min: i32, y_max: i32) -> Vec<(i32, i32)> {
    let mut rng = rand::thread_rng();
    let mut coordinates = Vec::with_capacity(num_pairs);
    for _ in 0..num_pairs {
        let x = rng.gen_range(x_min..=x_max);
        let y = rng.gen_range(y_min..=y_max);
        coordinates.push((x, y));
    }
    coordinates
}

fn main() {
    let coordinates = generate_random_coordinates(50, 0, 10, 0, 10);
    let coordinates_f64: Vec<(f64, f64)> = coordinates.iter().map(|&(x, y)| (x as f64, y as f64)).collect();
    let flattened_coordinates: Vec<f64> = coordinates_f64.iter().flat_map(|&(x, y)| vec![x, y]).collect();
    let (r, equation) = mathi::line_of_best_fit(&coordinates_f64);
    
    mathi::plot_graph(flattened_coordinates.as_slice(), &equation, Some(5.0), Some(10.0));
    println!("r = {:.2}, equation: {}", r, equation);
}
