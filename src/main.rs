use rand::Rng;

fn random_float(size: f32) -> f32 {
    let mut rng = rand::thread_rng();
    rng.gen_range(0.0..size)
}

fn cost(w: f32, train: &Vec<Vec<i32>>) -> f32 {
    let mut res = 0.0;
    for it in train.iter() {
        let t = it[1] as f32;
        let x = it[0] as f32;
        let y = x * w;
        let err = y - t;
        res += err * err;
    }
    res /= train.len() as f32;
    res
}

fn cost_derivative(a: f32, train: &Vec<Vec<i32>>) -> f32 {
    let h = 1e-3;
    (cost(a + h, train) - cost(a, train)) / h
}

fn main() {
    let train = vec![
        vec![0, 0],
        vec![1, 3],
        vec![2, 6],
        vec![3, 9],
        vec![4, 12],
    ];

    let mut w = random_float(10.0);
    let rate = 1e-3;

    for _ in 0..500 {
        w -= rate * cost_derivative(w, &train);
        println!("w: {}, err: {}", w, cost(w, &train));
    }
}
