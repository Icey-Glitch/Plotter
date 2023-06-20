pub fn line_of_best_fit(points: &[(f64, f64)]) -> (f64, String) {
    let n = points.len() as f64;
    let (sum_x, sum_y, sum_xy, sum_x_squared, sum_y_squared) = points.iter().fold((0.0, 0.0, 0.0, 0.0, 0.0), |(sx, sy, sxy, sx2, sy2), &(x, y)| (sx + x, sy + y, sxy + x * y, sx2 + x * x, sy2 + y * y));

    let r = (n * sum_xy - sum_x * sum_y) / ((n * sum_x_squared - sum_x * sum_x).sqrt() * (n * sum_y_squared - sum_y * sum_y).sqrt());
    let b = (n * sum_xy - sum_x * sum_y) / (n * sum_x_squared - sum_x * sum_x);
    let a = (sum_y - b * sum_x) / n;

    let equation = format!("y = {:.2}x + {:.2}", b, a);

    (r, equation)
}


pub fn plot_graph(data: &[f64], equation: &str, break_value: Option<f64>, scale: Option<f64>) {
    let (min_value, max_value) = get_min_max(data);
    let (start_row, end_row) = get_row_range(min_value, max_value, break_value);
    let row_labels = get_row_labels(min_value, max_value, end_row - start_row);
    print_rows(data, min_value, max_value, start_row, end_row);
    print_scale(scale);
    print_equation(equation);
    print_line_of_best_fit(data, min_value, max_value, equation);
    print_row_labels(row_labels);
}

fn get_min_max(data: &[f64]) -> (f64, f64) {
    let max_value = data.iter().fold(std::f64::MIN, |a, &b| a.max(b));
    let min_value = data.iter().fold(std::f64::MAX, |a, &b| a.min(b));
    (min_value, max_value)
}

fn get_row_range(min_value: f64, max_value: f64, break_value: Option<f64>) -> (usize, usize) {
    let range = max_value - min_value;
    let num_rows = 10;
    let row_height = range / num_rows as f64;

    let mut start_row = 0;
    let mut end_row = num_rows;
    if let Some(break_value) = break_value {
        let break_row = ((break_value - min_value) / row_height).ceil() as usize;
        if break_row < num_rows {
            end_row = break_row;
        } else {
            start_row = break_row - num_rows;
            end_row = break_row;
        }
    }

    (start_row, end_row)
}

fn get_row_labels(min_value: f64, max_value: f64, num_rows: usize) -> Vec<String> {
    let row_height = (max_value - min_value) / num_rows as f64;
    (0..num_rows)
        .map(|i| format!("{:.2}", min_value + row_height * (num_rows - i) as f64))
        .collect()
}

fn print_rows(data: &[f64], min_value: f64, max_value: f64, start_row: usize, end_row: usize) {
    for row in (start_row..end_row).rev() {
        let row_min = min_value + (max_value - min_value) / 10.0 * row as f64;
        let row_max = row_min + (max_value - min_value) / 10.0;
        let row_str = data
            .iter()
            .map(|&value| if value >= row_min && value < row_max { '*' } else { ' ' })
            .collect::<String>();
        println!("{:.2} | {}", row_max, row_str);
    }
    println!("{:.2} |{}", min_value, "-".repeat(data.len()));
}

fn print_scale(scale: Option<f64>) {
    if let Some(scale) = scale {
        println!("Scale: 1:{}\n", scale);
    }
}

fn print_equation(equation: &str) {
    println!("Line of Best Fit: {}", equation);
}

fn print_line_of_best_fit(data: &[f64], _min_value: f64, max_value: f64, equation: &str) {
    if let Some((slope, intercept)) = parse_equation(equation) {
        let _x1 = 0.0;
        let y1 = intercept;
        let x2 = max_value;
        let y2 = slope * x2 + intercept;
        let line_str = data
            .iter()
            .map(|&value| if value >= y1 && value <= y2 { '*' } else { ' ' })
            .chain("  Line of Best Fit".chars())
            .collect::<String>();
        println!("{}\n", line_str);
    }
}

fn print_row_labels(row_labels: Vec<String>) {
    println!("{}\n", row_labels.join(" | "));
}

fn parse_equation(equation: &str) -> Option<(f64, f64)> {
    let parts: Vec<&str> = equation.split(' ').collect();
    parts.get(0)
        .and_then(|slope| slope.parse().ok())
        .zip(parts.get(2).and_then(|intercept| intercept.parse().ok()))
}