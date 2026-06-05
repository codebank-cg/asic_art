mod cli;

use asic_art_lib::{loader, player, resizer, source, writer};
use asic_art_lib::resizer::ResizeOptions;

fn main() {
    let args = cli::parse();

    let opts = ResizeOptions {
        width: args.width,
        height: args.height,
        scale: args.scale,
    };

    let bw = args.bw;

    if let Some(video_path) = &args.video {
        let mut src = source::VideoFileSource::open(video_path, None)
            .unwrap_or_else(|e| { eprintln!("Error: {e}"); std::process::exit(1); });
        player::play(&mut src, &opts, Some(video_path), bw)
            .unwrap_or_else(|e| { eprintln!("Error: {e}"); std::process::exit(1); });
        return;
    }

    if let Some(cam_arg) = &args.cam {
        let device_index = cam_arg.unwrap_or(0);
        let mut src = source::WebcamSource::open(device_index, args.fps)
            .unwrap_or_else(|e| { eprintln!("Error: {e}"); std::process::exit(1); });
        player::play(&mut src, &opts, None, bw)
            .unwrap_or_else(|e| { eprintln!("Error: {e}"); std::process::exit(1); });
        return;
    }

    // Single-image path.
    let input = args.input.as_ref().unwrap();
    let img = loader::load(input).unwrap_or_else(|e| {
        eprintln!("Error: {e}");
        std::process::exit(1);
    });

    let resized = resizer::resize(&img, &opts);

    if let Some(output_path) = &args.output {
        writer::write_file(&resized, output_path, bw).unwrap_or_else(|e| {
            eprintln!("Error: {e}");
            std::process::exit(1);
        });
    } else {
        writer::write_stdout(&resized, bw);
    }
}
