use std::time::{Duration, Instant};
use std::thread;
use rand::prelude::random;

const MAX_MATRIX_SIZE: usize = 20;

type T = u128;
type Matrix = [[T; MAX_MATRIX_SIZE]; MAX_MATRIX_SIZE];

static mut Z: Matrix = [[0; MAX_MATRIX_SIZE]; MAX_MATRIX_SIZE];

fn create_random_matrix() -> Matrix {
    let mut rng: u32 = random();
    let mut a: Matrix = [[rng as u128; MAX_MATRIX_SIZE]; MAX_MATRIX_SIZE];

    for i in 0..MAX_MATRIX_SIZE {
        for j in 0..MAX_MATRIX_SIZE {
            rng = random();
            a[i][j] = rng as u128;
        }
    }

    return a;
}

// fn print_matrix(a: Matrix, label: &str) {
//     println!("{}", label);

//     for i in 0..MAX_MATRIX_SIZE {
//         for j in 0..MAX_MATRIX_SIZE {
//             print!("{}\t", a[i][j]);
//         }
//         println!();
//     }
// }

fn get_matrix_row(i: usize, u: Matrix, v: Matrix) {
    for j in 0..MAX_MATRIX_SIZE {
        // print!("{}x{}={}\t", u[i][j], v[j][i], u[i][j] * v[j][i]);
        // println!("z line is {}, {}x{}", i, u[i][j], v[j][i]);
        unsafe {
            Z[i][j] = u[i][j] * v[j][i];
        }
    }
}

fn multi_matrices(x: Matrix, y: Matrix, parallel: bool) -> Duration {
    let start: Instant;
    let duration: Duration;

    if parallel {
        start = Instant::now();
        let handle = thread::spawn(move || {
            for i in 0..MAX_MATRIX_SIZE {
                get_matrix_row(i, x, y);
            }
        });

        handle.join().unwrap();

        duration = start.elapsed();
    } else {
        start = Instant::now();

        for i in 0..MAX_MATRIX_SIZE {
            get_matrix_row(i, x, y);
        }

        duration = start.elapsed();
        println!("Duration: {:?}", duration);
    }
        
    return duration;
}

fn main() {
    let x: Matrix = create_random_matrix();
    let y: Matrix = create_random_matrix();
    let mut duration: Duration;

    duration = multi_matrices(x, y, false);
    println!("The execution time in the SEQUENTIAL implementation was {:?} ", duration);
    
    duration = multi_matrices(x, y, true);
    println!("The execution time in the PARALLEL implementation was {:?} ", duration);

    // print_matrix(x, "Matriz A");
    // print_matrix(y, "\nMatriz B");

    // unsafe {
    //     print_matrix(z, "\nMatriz Z");
    // }
    
}