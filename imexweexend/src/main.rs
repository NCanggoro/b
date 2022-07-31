
use clap::{command, Arg};

fn main() {
    let matches = command!() 
        .arg(Arg::new("blur")
            .help("Take 3 arguments blur_rate(f32) input(filename) output(filename)")
            .short('B')
            .required(false)
            .multiple_values(true)
            .takes_value(true)
        )
        .arg(Arg::new("fractal")
            .short('F')
            .required(false)
            .multiple_values(true)
            .takes_value(true)
        )
        .get_matches();

    if let Some(blur) = matches.get_many::<String>("blur") {
        let args = blur.collect::<Vec<&String>>();
        blur_effect(args[0].parse::<f32>().unwrap(), &args[1], &args[2]);
    }

    if let Some(fractal) = matches.get_many::<String>("fractal") {
        let args = fractal.collect::<Vec<&String>>();
        fractal_effect(&args[0]);
    }

}

fn blur_effect(blur: f32, infile: &String, outfile: &String) {
    let img = image::open(infile).expect("Failed to open INFILE.");
    let img2 = img.blur(blur);
    img2.save(outfile).expect("Failed writing OUTFILE.");
}

fn brighten(bright_rate: i32, infile: &String, outfile: &String) {
    let img = image::open(infile).expect("Failed to open INFILE.");
    let img2 = img.brighten(bright_rate);

    img2.save(outfile).expect("Failed writing OUTFILE.");
}

fn crop(x: u32, y: u32, width: u32, height: u32, infile: &String, outfile: &String) {
    let img = image::open(infile).expect("Failed to open INFILE");
    let img2 = img.crop(x, y, width, height);
    img2.save(outfile).expect("Failed to save OUTFILE");

}

fn grayscale(infile: &String, oufile: &String) {
    let img = image::open(infile).expect("Failed to open INFILE");
    let img2 = img.grayscale();
    img2.save(outfile).expect("Failed to save OUTFILE");
}

fn invert(infile: String, outfile: String) {
    let img = image::open(infile).expect("Failed to open INFILE");
    let img2 = img.invert();
    img2.save(outfile).expect("Failed to save to OUTFILE");
}

fn fractal_effect(outfile: &String) {
    //make it customizable maybe ?
    let width = 800;
    let height = 800;

    let mut imgbuf = image::ImageBuffer::new(width, height);

    let scale_x = 3.0 / width as f32;
    let scale_y = 3.0 / height as f32;

    // Iterate over the coordinates and pixels of the image
    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        // Use red and blue to be a pretty gradient background
        // TODO: set argument for each gradient distance
        let red = (0.3 * x as f32) as u8;
        let blue = (0.3 * y as f32) as u8;

        // Use green as the fractal foreground (here is the fractal math part)
        // Or set the fractal as argument
        let cx = y as f32 * scale_x - 1.5;
        let cy = x as f32 * scale_y - 1.5;

        let c = num_complex::Complex::new(-0.4, 0.6);
        let mut z = num_complex::Complex::new(cx, cy);

        let mut green = 0;
        while green < 255 && z.norm() <= 2.0 {
            z = z * z + c;
            green += 1;
        }

        // set the pixel. red, green, and blue are u8 
        *pixel = image::Rgb([red, green, blue]);
    }

    imgbuf.save(outfile).unwrap();
}