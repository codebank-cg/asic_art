mod cli;

use asic_art_lib::{loader, renderer, resizer, writer};

fn main() {
    let args = cli::parse();

    let img = loader::load(&args.input).unwrap_or_else(|e| {
        eprintln!("Error: {e}");
        std::process::exit(1);
    });

    let opts = resizer::ResizeOptions {
        width: args.width,
        height: args.height,
        scale: args.scale,
    };

    let resized = resizer::resize(&img, &opts);
    let rows = renderer::render(&resized);

    if let Some(output_path) = &args.output {
        writer::write_file(&rows, output_path).unwrap_or_else(|e| {
            eprintln!("Error: {e}");
            std::process::exit(1);
        });
    } else {
        writer::write_stdout(&rows);
    }
}
