// ai thats traning in real-time 

use rand::Rng;

fn rlu(result: f64) -> f64 {
    result.max(0.)
}

fn final_calculation(w: Vec<f64>, b: f64) -> f64 {
    0.
}

fn weights(mem: &Vec<f64>) -> Vec<f64> {
    Vec::new()
}

const BIAS_MIN: f64 = -1.;
const BIAS_MAX: f64 = 1.;

// just random float number
fn bias() -> f64 {
    let mut rng = rand::rng();
    rng.gen_range(BIAS_MIN..BIAS_MAX)
}

fn main() {
    let mut mem: Vec<f64> = Vec::new();

    loop {
        let w = weights(&mem);
        let b = bias();

        let result = final_calculation(w, b);
        let r_with_rlu = rlu(result);

        println!("{}", r_with_rlu);
    }

}