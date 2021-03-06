use anyhow::Result;
use env_logger;
use std::env;

mod days;

#[macro_use]
extern crate scan_rules;

#[tokio::main]
async fn main() -> Result<()> {
    env::set_var("RUST_LOG", "DEBUG");
    env_logger::init();

    // print_day(1);
    // days::one::solve().await?;
    //
    // print_day(2);
    // days::two::solve().await?;
    //
    // print_day(3);
    // days::three::solve().await?;
    //
    // print_day(4);
    // days::four::solve().await?;
    //
    // print_day(5);
    // days::five::solve().await?;
    //
    // print_day(6);
    // days::six::solve().await?;
    //
    // print_day(7);
    // days::seven::solve().await?;
    //
    // print_day(8);
    // days::eight::solve().await?;
    //
    // print_day(9);
    // days::nine::solve().await?;

    print_day(10);
    days::ten::solve().await?;

    Ok(())
}

fn print_day(day: i32) {
    println!("---------------------- DAY {} -----------------------", day);
}
