use crate::{
    math::*,
    regressor::{
        config::Config,
        regressor::Regressor,
    },
};

#[test]
fn test_sgd() {
    let (X, y) = gen_dataset(1000, (0f64, 20f64), 1e-1,
                             |x| { 10f64 + 2f64 * x });
    let sgd = Config::default()
        .to_SGD()
        .fit(X.clone(), y.clone());

    println!("SGD R2 Test Score: {}", sgd.score(&X, &y));

    assert!(sgd.score(&X, &y) > 0.9);
}

#[test]
fn test_adagrad() {
    let (X, y) = gen_dataset(1000, (0f64, 40f64), 1e-1,
                             |x| { 10f64 + 4f64 * x });
    let ag = Config::default()
        .eta(1e-1)
        .to_AdaGrad()
        .fit(X.clone(), y.clone());

    println!("Adagrad R2 Test Score: {}", ag.score(&X, &y));

    assert!(ag.score(&X, &y) > 0.7);
}

#[test]
fn test_rmsprop() {
    let (X, y) = gen_dataset(1000, (30f64, 60f64), 1e-1,
                             |x| { -10f64 + 3f64 * x });
    let rmsp = Config::default()
        .to_RMSProp()
        .fit(X.clone(), y.clone());

    println!("RMSProp R2 Test Score: {}", rmsp.score(&X, &y));

    assert!(rmsp.score(&X, &y) > 0.8);
}

#[test]
fn test_adam() {
    let (X, y) = gen_dataset(1000, (2f64, 16f64), 1e-1,
                             |x| { -2f64 + 10f64 * x });
    let adam = Config::default()
        .to_Adam()
        .fit(X.clone(), y.clone());

    println!("Adam R2 Test Score: {}", adam.score(&X, &y));

    assert!(adam.score(&X, &y) > 0.9);
}