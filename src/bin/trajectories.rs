use std::io:: Result;
use std::path::Path;

use clap::Parser;
use indicatif::{ProgressBar, ProgressStyle};
use rayon::prelude::*;

use sentinel::configure::Configure;
use sentinel::geometry::Cone;
use sentinel::field::Field;
use sentinel::tracker::Tracker;
use sentinel::io::CsvWriter;
use sentinel::invalid_input;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct CLI {

    #[arg(short, long)]
    conf: String,

    #[arg(short, long)]
    n_events: usize,

    #[arg(short, long, default_value_t=1000)]
    batch_size: usize,

    #[arg(short, long)]
    output: String,

    #[arg(long, default_value_t=false)]
    overwrite: bool,

    #[arg(short, long, default_value_t=10)]
    threads: usize,
}

pub fn main() -> Result<()> {
    let args = CLI::parse();
    let conf = Configure::new(&args.conf).unwrap();
    let path = Path::new(&args.output);
    if path.exists() & !args.overwrite {
        return invalid_input!("Outfile already exists!");
    }

    rayon::ThreadPoolBuilder::new().num_threads(args.threads).build_global().unwrap();

    let header   = "event x y z t".split(" ")
                                  .map(|v| v.to_string())
                                  .collect::<Vec<String>>();
    let mut writer = CsvWriter::new(&path.to_str().unwrap(), " ", header).unwrap();
    let geometry   = Cone::new(conf.rmin, conf.form_factor, conf.zmax);
    let field      = Field::from_file(&conf.field_file, conf.field_to_mm, conf.field_to_Vpercm, true);
    let tracker    = Tracker::new(field, geometry.clone(), conf.t_step);
    let sampler    = conf.generator.sampler(&geometry);

    let nbatch = args.n_events.div_ceil(args.batch_size);
    let pb = ProgressBar::new(nbatch as u64);
    pb.set_style(
        ProgressStyle::with_template(
            "{spinner:.magenta} [{elapsed_precise}] \
             [{bar:40.bold.#c000ff/#5a00aa}] \
             {pos:>7}/{len:7} \
             {percent:>3}% \
             ETA {eta_precise}"
        ).unwrap()
         .progress_chars("█▉▊▋▌▍▎▏ ") // other: "█▓░" "█▇▆▅▄▃▂▁ "
    );
    pb.reset(); // force drawing
    for batch in 0..nbatch {
        let data : Vec<Vec<Vec<f64>>> =
        (0..args.batch_size).into_par_iter()
                            .map( |evt| {
                                let evt          = evt + batch * args.batch_size;
                                let starting_pos = sampler.sample(&geometry);
                                tracker.propagate_from(starting_pos)
                                       .into_iter()
                                       .enumerate()
                                       .map(|(i, p)| vec![evt as f64, p.x, p.y, p.z, i as f64 * conf.t_step])
                                       .collect::<Vec<Vec<f64>>>()
                            })
                            .collect();
        data.into_iter().for_each(|evt| {
            for row in evt {
                writer.write(row).unwrap();
            }
        });
        pb.inc(1);
    }
    pb.finish();

    Ok(())
}
