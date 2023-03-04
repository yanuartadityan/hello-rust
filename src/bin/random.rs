use rand::{Rng, SeedableRng};
use rand::rngs::StdRng;

fn main(){
    // generate single number from range
    let rand_num: u8 = rand::thread_rng().gen_range(0..10);
    println!("randomed number from 0..10: {}", rand_num);

    // generate float number from range
    let rand_num_f: f32 = rand::thread_rng().gen_range(0.0..10.0);
    println!("randomed number from 0..10: {:.2}", rand_num_f);

    // generate random using seed
    let mut generator = StdRng::seed_from_u64(234);
    println!("random using seed-{}: {:.2}", 234, generator.gen::<f32>());
}