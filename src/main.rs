pub mod vectorizer;

fn main() {
    let config = vtracer::Config {
        color_mode: vtracer::ColorMode::Color,
        hierarchical: vtracer::Hierarchical::Stacked,
        mode: visioncortex::PathSimplifyMode::Spline,
        filter_speckle: 4,
        color_precision: 8,
        layer_difference: 48,
        corner_threshold: 180,
        length_threshold: 4.0,
        splice_threshold: 45,
        max_iterations: 10,
        path_precision: Some(2),
    };

    let c = vtracer::convert_image_to_svg(
        std::path::Path::new("/home/silas/Projects/vectorizer/pic.jpg"),
        std::path::Path::new("/home/silas/Projects/vectorizer/pic.svg"),
        config,
    );

    match c {
        Ok(_) => println!("It worked!"),
        Err(x) => println!("{}", x),
    }
}
