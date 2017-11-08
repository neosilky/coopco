#![feature(alloc_system)] extern crate alloc_system;
#[macro_use] extern crate assert_approx_eq;
extern crate rand;
extern crate half;

mod func;

use rand::{thread_rng, Rng};
use half::f16;
use func::Function;
use std::f64;
use std::{thread, time};

const POPULATION_SIZE: usize = 40;
const TARGET_LEN: usize = 16;

#[derive(Clone, Debug)]
struct EvoPheno {
    val: u16,
    fitness: f64,
}

impl EvoPheno {
    fn new(t: u16) -> EvoPheno {
        EvoPheno {
            val: t,
            fitness: f64::MAX
        }
    }

    fn crossover(&self, other: &EvoPheno) -> EvoPheno {
        let mut new = self.val;

        for i in 0..TARGET_LEN {
            if rand::random::<f64>() < 0.6 {
                let j_value = (other.val & (1 << i)) >> i;
                new = (new & (!(1 << i))) | (j_value << i);
            }
        }

        EvoPheno::new(new)
    }

    fn mutate(&self) -> EvoPheno {
        let mut new = self.val;

        for i in 0..TARGET_LEN {
            if rand::random::<f64>() < (1f64 / TARGET_LEN as f64) {
                new ^= 1 << i;
            }
        }

        EvoPheno::new(new)
    }
}

fn make_vec(size: u32, value: u16) -> Vec<f64> {
    (0..size).map(|_| f64::from(f16::from_bits(value))).collect()
}

fn gen_pair() -> (usize, usize) {
    let a_index = thread_rng().gen_range(0, POPULATION_SIZE) as usize;
    let b_index = thread_rng().gen_range(0, POPULATION_SIZE) as usize;
    (a_index, b_index)
}

fn main() {
    let mut population: Vec<EvoPheno> = (0..POPULATION_SIZE).map(|_| EvoPheno::new(thread_rng().gen::<u16>())).collect();

//    let (function, dimensions) = (func::Rastrigin, 20);
//    let (function, dimensions) = (func::Schwefel, 10);
//    let (function, dimensions) = (func::Griewankg, 10);
//    let (function, dimensions) = (func::Ackley, 30);
    let (function, dimensions) = (func::Rosenbrock, 10);

    for p in &mut population {
        p.fitness = function.calc(make_vec(dimensions, p.val));
    }

    for p in &population {
        println!("{:?} = {}", make_vec(1, p.val), p.fitness);
    }

    let mut iterations = 0;
    let crossover = true;

    println!("iteration,fitness,value");

    while iterations < 1000 {
        let (a_index, b_index) = gen_pair();
       let parent1_index = if population[a_index].fitness < population[b_index].fitness {
            a_index
        } else {
            b_index
        };

        let mut child = if crossover {
            let (c_index, d_index) = gen_pair();
            let parent2_index = if population[c_index].fitness < population[d_index].fitness {
                c_index
            } else {
                d_index
            };

            let cross_child = population[parent1_index].crossover(&population[parent2_index]);
            cross_child.mutate()
        } else {
            population[parent1_index].mutate()
        };

        iterations += 1;
        child.fitness = function.calc(make_vec(dimensions, child.val));

        let (c_index, d_index) = gen_pair();

        let new_index = if population[c_index].fitness < population[d_index].fitness {
            d_index
        } else {
            c_index
        };

        std::mem::replace(&mut population[new_index], child);

        print!("{}[2J", 27 as char);

        for p in &population {
            println!("{:?} = {}", make_vec(1, p.val), p.fitness);
        }

        let mut lowest_fitness = f64::MAX;
        let mut best_value = f64::MAX;

        for p in &population {
            if p.fitness < lowest_fitness {
                lowest_fitness = p.fitness;
                best_value = f64::from(f16::from_bits(p.val));
            }
        }

        println!("\n{},{},{}", iterations, best_value, lowest_fitness);

        thread::sleep(time::Duration::from_millis(25));
    }
}
