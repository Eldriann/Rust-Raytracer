use std::process;
use rust_raytracer;
use clap::{App, Arg};

fn main() {
    let matches = App::new("rust_raytracer")
        .version("0.1.0")
        .author("Julian Frabel <julian.frabel@epitech.eu>")
        .about("A basic ray tracer written in rust")
        .arg(Arg::with_name("scene")
            .short("s")
            .long("scene")
            .help("Sets the json scene file to use. Will assume scene.json by default")
            .takes_value(true))
        .arg(Arg::with_name("output")
            .short("o")
            .long("output")
            .help("Sets the output file to write the result to. Will assume output.png by default")
            .takes_value(true))
        .arg(Arg::with_name("pass")
            .short("p")
            .long("pass")
            .help("Sets the number of passes to compute for a ray. Will assume 3 by default")
            .takes_value(true))
        .get_matches();

    let nb_pass = matches.value_of("pass").unwrap_or("3").parse().unwrap_or_else(|_| {
        eprintln!("pass argument expect a number");
        process::exit(1);
    });

    let config = rust_raytracer::Config::new(
        matches.value_of("scene").unwrap_or("scene.json").to_string(),
        matches.value_of("output").unwrap_or("output.png").to_string(),
        nb_pass
    );

    if let Err(e) = rust_raytracer::run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}
