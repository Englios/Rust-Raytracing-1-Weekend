// use std::io;

fn main(){

    //Image

    let image_width = 256;
    let image_height = 256;

    //Render
    println!("P3\n{} {}\n255", image_width, image_height);

    //use loop to iterate over the rows of the image from top to bottom
    for j in 0..image_height {
        for i in 0..image_width{
            let r = i as f64 / (image_width - 1) as f64;
            let g = j as f64 / (image_height -1) as f64;
            let b = 0.0 as f64;

            let ir = 255.99 * r; 
            let ig = 255.99 * g;
            let ib = 255.99 * b;

            println!("{ir} {ig} {ib}")

        }
    }
}