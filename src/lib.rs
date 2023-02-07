// use rayon::par_iter::{IntoParallelIterator, ParallelIterator};
use rayon::prelude::{IntoParallelIterator, ParallelIterator};

/// Apply Composite Simpson 1/3 Rule to integrate y over x, works best for regularly spaced data.
/// Number of samples given must be odd
pub fn simpson(y: &Vec<f64>, x: &Vec<f64>) -> Result<f64, String> {
    // Handle case where ax and y are not of equal length
    if x.len() != y.len() {
        return Err("x and y must be of equal length".to_string());
    }

    // x and y should be odd to have an even number of intevals
    if x.len() % 2 == 0 {
        return Err("The length of x and y must be an odd number".to_string());
    }

    // h = (b-a)/n, where n is number of sub intervals
    let h: f64 = (x[x.len() - 1] - x[0]) / (x.len() as f64 - 1.0);
    // sum all x and y components using Composite simpson rule
    let mut integral: f64 = (0..x.len())
        .into_par_iter()
        .map(|i| {
            if i == 0 || i == x.len() - 1 {
                y[i]
            } else if i % 2 == 0 {
                2.0 * y[i]
            } else {
                4.0 * y[i]
            }
        })
        .sum();
    // append with 3/8 *h
    integral *= (1.0 / 3.0) * h;
    Ok(integral)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        let y = vec![0.0_f64, 1.0_f64, 2.0_f64, 3.0_f64, 4.0_f64];
        let x = vec![0.0_f64, 1.0_f64, 2.0_f64, 3.0_f64, 4.0_f64];
        let result = simpson(&y, &x).unwrap();
        assert_eq!(result, 8.0);
    }

    #[test]
    fn long() {
        let x: Vec<f64> = (0..1_000_001).map(f64::from).collect();
        let y: Vec<f64> = x.iter().map(|x| x.sin()).collect();

        let result = simpson(&y, &x).unwrap();
        assert_eq!(result.floor(), 0.0);
    }

    #[test]
    fn unequal_vec() {
        let y = vec![0.0_f64, 1.0_f64, 2.0_f64, 3.0_f64];
        let x = vec![0.0_f64, 1.0_f64, 2.0_f64];
        let result = simpson(&y, &x);
        assert!(result.is_err());
    }

    #[test]
    fn even_length_vec() {
        let y = vec![0.0_f64, 1.0_f64, 2.0_f64, 3.0_f64];
        let x = vec![0.0_f64, 1.0_f64, 2.0_f64, 3.0_f64];
        let result = simpson(&y, &x);
        assert!(result.is_err());
    }
}
