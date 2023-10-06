use std::{ fmt::Display, cmp::Ordering };
use rand::{ seq::IteratorRandom, thread_rng }; // 0.6.1
use rand::Rng;

use random_choice::random_choice;

fn main() {
    println!("Hello, GA!");
    let pop_size = 10;
    let mut population = init_population(pop_size, 5);

    for gen in 1..20 {
        println!("Generation {} : {:?}", gen, population);
        let mut nextgen_population = Vec::new();
        for _ in 1..pop_size / 2 {
            let parent1 = selection(&population); // # select first parent
            let parent2 = selection(&population); // # select second parent
            let (offspring1, offspring2) = crossover(parent1, parent2); // perform crossover between both parents
            nextgen_population.push(mutate(offspring1));
            nextgen_population.push(mutate(offspring2));
        }
        population = nextgen_population;
    }
}
#[derive(Debug, Clone)]
struct Individual(Vec<i32>);
impl Display for Individual {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "****************")
    }
}
impl Ord for Individual {
    fn cmp(&self, other: &Self) -> Ordering {
        fitness(self).cmp(&fitness(other))
    }
}
impl PartialOrd for Individual {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.0.partial_cmp(&other.0)
    }
}
impl PartialEq for Individual {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl Eq for Individual {}

fn init_population(pop_size: usize, genome_size: usize) -> Vec<Individual> {
    let samples = vec![0, 1];
    let weights: Vec<f64> = vec![1.0, 1.0];
    let mut population = vec![];
    for _ in 1..pop_size {
        let individual = Individual(
            random_choice()
                .random_choice_f64(&samples, &weights, genome_size)
                .into_iter()
                .cloned()
                .collect()
        );
        println!("{:?}", individual);
        //print_type_of(&individual);
        population.push(individual);
    }
    population
}
/*
to solve a simple problem often used for demonstration purposes, known as maxone, where the objective is to find the binary vector with the most 1s.
 */
fn fitness(individual: &Individual) -> i32 {
    individual.0.iter().sum()
}

fn selection(population: &Vec<Individual>) -> Individual {
    let mut rng = thread_rng();
    let sample = population.iter().choose_multiple(&mut rng, 3);
    //println!("sample : {:?}", sample);
    let fitnesses: Vec<i32> = sample
        .iter()
        .map(|i| fitness(i))
        .collect();
    println!("fitnesses : {:?}", fitnesses);
    let winner = sample.iter().max().unwrap();
    println!("winner : {:?}", winner);
    Individual(winner.0.clone())
}
/*
Single-point crossover, which splits two parents at a random point in the middle, and swaps their parts to the right
 */
fn crossover(parent1: Individual, parent2: Individual) -> (Individual, Individual) {
    let mut rng = rand::thread_rng();
    let xo_point = rng.gen_range(1..parent1.0.len());
    let mut v1 = Vec::new();
    v1.extend_from_slice(&parent1.0[1..=xo_point]);
    v1.extend_from_slice(&parent2.0[xo_point..]);
    let mut v2 = Vec::new();
    v2.extend_from_slice(&parent2.0[1..=xo_point]);
    v2.extend_from_slice(&parent1.0[xo_point..]);

    (Individual(v1), Individual(v2))
}

fn mutate(mut individual: Individual) -> Individual {
    let mut rng = rand::thread_rng(); // Create a random number generator
    for i in 0..individual.0.len() {
        if rng.gen::<f64>() < 0.1 {
            individual.0[i] = 1 - individual.0[i]; // Bit flip
        }
    }
    individual
}

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_init_population() {
        init_population(10, 5);
    }
    #[test]
    fn test_selection() {
        let population = init_population(10, 5);
        selection(&population);
    }
    #[test]
    fn test_fitness() {
        let individual = Individual(vec![1, 0, 1, 0, 1]);
        let fitness = fitness(&individual);
        println!("fiteness {:?}", fitness);
        assert_eq!(3, fitness);
    }
    #[test]
    fn test_crossover() {
        let individual1 = Individual(vec![1, 1, 1, 0, 1]);
        let individual2 = Individual(vec![1, 0, 0, 0, 1]);
        let (i1, i2) = crossover(individual1, individual2);
        println!("{:?}", i1);
        println!("{:?}", i2);
    }
    #[test]
    fn test_mutation() {
        for i in 1..10 {
            let individual = Individual(vec![1, 1, 1, 0, 1]);
            let mindividual = mutate(individual);
            println!("{:?}", mindividual);
        }
    }
}
