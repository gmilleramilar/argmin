// Copyright 2018-2022 argmin developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

use argmin::{
    core::{
        checkpointing::{CheckpointingFrequency, FileCheckpoint},
        observers::ObserverMode,
        CostFunction, Error, Executor, Gradient,
    },
    solver::landweber::Landweber,
};
use argmin_observer_slog::SlogLogger;
use argmin_testfunctions::{rosenbrock_2d, rosenbrock_2d_derivative};

#[derive(Default)]
struct Rosenbrock {}

impl CostFunction for Rosenbrock {
    type Param = Vec<f64>;
    type Output = f64;

    fn cost(&self, p: &Self::Param) -> Result<Self::Output, Error> {
        Ok(rosenbrock_2d(p, 1.0, 100.0))
    }
}

impl Gradient for Rosenbrock {
    type Param = Vec<f64>;
    type Gradient = Vec<f64>;

    fn gradient(&self, p: &Self::Param) -> Result<Self::Gradient, Error> {
        Ok(rosenbrock_2d_derivative(p, 1.0, 100.0))
    }
}

fn run() -> Result<(), Error> {
    // define initial parameter vector
    let init_param: Vec<f64> = vec![1.2, 1.2];

    let iters = 35;
    let solver = Landweber::new(0.001);

    // Configure checkpointing
    let checkpoint = FileCheckpoint::new(
        ".checkpoints",
        "rosenbrock_optim",
        CheckpointingFrequency::Always,
    );

    let res = Executor::new(Rosenbrock {}, solver)
        .configure(|state| state.param(init_param).max_iters(iters))
        .checkpointing(checkpoint)
        .add_observer(SlogLogger::term(), ObserverMode::Always)
        .run()?;

    // Wait a second (lets the logger flush everything before printing to screen again)
    std::thread::sleep(std::time::Duration::from_secs(1));
    println!("{res}");
    Ok(())
}

fn main() {
    if let Err(ref e) = run() {
        println!("{e}");
    }
}
