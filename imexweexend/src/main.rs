use clap::{arg, command, App, Arg, Command};
use imexcenger::util::helper::{blur_effect, fractal_effect, generate, brighten, invert_image, crop_image, grayscale_effect};
use std::process;

fn cli() -> App<'static> {
    command!()
        .arg(
            Arg::new("fractal")
                .long("fractal")
                .help("create an fractal image, with <output> as argument")
                .takes_value(true)
                .required(false),
        )
        .arg(
            Arg::new("generate")
                .long("generate")
                .multiple_values(true)
                .takes_value(true)
                .required(false),
        )
        .arg(
            Arg::new("blur")
                .long("blur")
                .multiple_values(true)
                .takes_value(true)
                .required(false),
        )
        .arg(
            Arg::new("bright")
                .long("bright")
                .multiple_values(true)
                .takes_value(true)
                .required(false),
        )
        .arg(
            Arg::new("crop")
                .long("crop")
                .multiple_values(true)
                .takes_value(true)
                .required(false),
        )
        .arg(
            Arg::new("grayscale")
                .long("grayscale")
                .multiple_values(true)
                .takes_value(true)
                .required(false),
        )
        .arg(
            Arg::new("invert")
                .long("invert")
                .multiple_values(true)
                .takes_value(true)
                .required(false),
        )
}

fn main() {
    // TODO: cargo run infile.png outfile.png --blur 2.5 --invert --rotate 180 --brighten 10

    let matches = cli().get_matches();

    if let Some(fractal) = matches.get_many::<String>("fractal") {
        let args = fractal.collect::<Vec<&String>>();
        fractal_effect(&args[0]);
        process::exit(256)
    }

    if let Some(gen) = matches.get_many::<String>("generate") {
        let args = gen.collect::<Vec<&String>>();
        generate(
            args[0],
            args[1].parse::<u8>().unwrap(),
            args[2].parse::<u8>().unwrap(),
            args[3].parse::<u8>().unwrap(),
        );
        process::exit(256)
    }

    if let Some(blur) = matches.get_many::<String>("blur") {
        let args = blur.collect::<Vec<&String>>();
        blur_effect(
            args[2].parse::<f32>().unwrap(), 
            args[0], 
            args[1]
        );
    }

    if let Some(bright) = matches.get_many::<String>("bright") {
        let args = bright.collect::<Vec<&String>>();
        brighten(
            args[2].parse::<i32>().unwrap(), 
            args[0], 
            args[1]
        );
    }

    if let Some(crop) = matches.get_many::<String>("crop") {
        let args = crop.collect::<Vec<&String>>();
        crop_image(
            args[4].parse::<u32>().unwrap(), 
            args[5].parse::<u32>().unwrap(), 
            args[2].parse::<u32>().unwrap(),
            args[3].parse::<u32>().unwrap(),
            args[0],
            args[1]
        );
    }

    if let Some(grayscale) = matches.get_many::<String>("grayscale") {
        let args = grayscale.collect::<Vec<&String>>();
        grayscale_effect(
            args[0],
            args[1], 
        );
    }

    if let Some(invert) = matches.get_many::<String>("invert") {
        let args = invert.collect::<Vec<&String>>();
        invert_image(
            args[0], 
            args[1]
        );
    }

}
