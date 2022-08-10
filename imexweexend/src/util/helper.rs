pub fn blur_effect(blur: f32, infile: &String, outfile: &String) {
  let img = image::open(infile).expect("Failed to open INFILE.");
  let img2 = img.blur(blur);
  img2.save(outfile).expect("Failed writing OUTFILE.");
}

pub fn brighten(bright_rate: i32, infile: &String, outfile: &String) {
  let img = image::open(infile).expect("Failed to open INFILE.");
  let img2 = img.brighten(bright_rate);

  img2.save(outfile).expect("Failed writing OUTFILE.");
}

pub fn crop_image(x: u32, y: u32, width: u32, height: u32, infile: &String, outfile: &String) {
  let mut img = image::open(infile).expect("Failed to open INFILE");
  img
  .crop(x, y, width, height)
  .save(outfile)
  .expect("Failed to save to OUTFILE");

}

pub fn grayscale_effect(infile: &String, outfile: &String) {
  let img = image::open(infile).expect("Failed to open INFILE");
  let img2 = img.grayscale();
  img2.save(outfile).expect("Failed to save OUTFILE");
  println!("Grayscale {} success", outfile);
}

pub fn invert_image(infile: &String, outfile: &String) {
  let mut img = image::open(infile).expect("Failed to open INFILE");
  img.invert();
  img.save(outfile).expect("Failed to save to OUTFILE");
  println!("Invert image {} success", outfile);
}


pub fn fractal_effect(outfile: &String) {
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
  println!("Fractal {} success", outfile);
}

pub fn generate(outfile: &String, r: u8, g: u8, b: u8) {
  let width:  u32 = 800;
  let height: u32 = 800;

  let mut imgbuf = image::ImageBuffer::new(width, height);

  for (_, _, pixel) in imgbuf.enumerate_pixels_mut() {
      *pixel = image::Rgb([r, g, b]);
  }

  imgbuf.save(outfile).unwrap();
  println!("Generate {} success", outfile);
}