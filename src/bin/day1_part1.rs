fn main() {
    let measurements: [u32; 10] = [199, 200, 208, 210, 200, 207, 240, 269, 260, 263];

    println!(
        "measurements {}",
        calculate_increase_measurements(&measurements)
    );
}

fn calculate_increase_measurements(measurements: &[u32]) -> u32 {
    let mut counter: u32 = 0;
    let mut index: usize = 1;

    while index < measurements.len() {
        if measurements[index] >= measurements[index - 1] {
            counter += 1;
        }

        index += 1
    }

    counter
}
