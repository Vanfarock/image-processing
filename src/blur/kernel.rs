use std::f32::consts::{E, PI};

pub fn get_average_kernel(kernel_size: (u32, u32)) -> Vec<Vec<f32>> {
    let size_x = kernel_size.0 as usize;
    let mut kernel: Vec<Vec<f32>> = vec![];

    for _ in 0..(kernel_size.1) {
        kernel.push(vec![1.0; size_x]);
    }
    kernel
}

pub fn get_gaussian_kernel(kernel_size: (u32, u32), standard_deviation: f32) -> Vec<Vec<f32>> {
    let mut kernel: Vec<Vec<f32>> = vec![];

    let squared_deviation = standard_deviation.powf(2.0);
    let base = 1.0 / (2.0 * PI * squared_deviation) * E;

    for x in 0..(kernel_size.0) {
        let mut row: Vec<f32> = vec![];
        for y in 0..(kernel_size.1) {
            let exponent = -((x.pow(2) + y.pow(2)) as f32) / (2.0 * squared_deviation);

            row.push(base.powf(exponent));
        }
        kernel.push(row);
    }

    kernel
}
