use std::io:: Result;
use std::path::Path;
use std::rc::Rc;

use clap::Parser;
use rayon::prelude::*;
use hdf5_metno as hdf5;

use sentinel::configure::Configure;
use sentinel::geometry::Cone;
use sentinel::field::Field;
use sentinel::tracker::Tracker;
use sentinel::io::{Writer, CsvWriter, Hdf5Writer};
use sentinel::io::hdf5_types::TrajectoryPoint;
use sentinel::invalid_input;
use sentinel::progress::MaybeProgressBar;

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

    #[arg(long, default_value_t=false)]
    csv: bool,

    #[arg(long, default_value_t=false)]
    batch: bool,
}

pub fn main() -> Result<()> {
    let args = CLI::parse();
    let conf = Configure::new(&args.conf).unwrap();
    let path = Path::new(&args.output);
    if path.exists() & !args.overwrite {
        return invalid_input!("Outfile already exists!");
    }

    rayon::ThreadPoolBuilder::new().num_threads(args.threads).build_global().unwrap();

    let writer : Box<dyn Writer<TrajectoryPoint>> =
        if args.csv {
            let header = "event x y z t".split(" ")
                                        .map(|v| v.to_string())
                                        .collect::<Vec<String>>();

            Box::new(CsvWriter::new(&path.to_str().unwrap(), " ", header).unwrap())
        }
        else {
            let file = hdf5::File::create(&path.to_str().unwrap()).unwrap();
            Box::new(Hdf5Writer::<TrajectoryPoint>::new(Rc::new(file), "trajectories", 1024).unwrap())
        };

    let geometry   = Cone::new(conf.rmin, conf.form_factor, conf.zmax);
    let field      = Field::from_file(&conf.field_file, conf.field_to_mm, conf.field_to_Vpercm, true);
    let tracker    = Tracker::new(field, geometry.clone(), conf.t_step);
    let sampler    = conf.generator.sampler(&geometry);

    let nbatch = args.n_events.div_ceil(args.batch_size);
    let pb = MaybeProgressBar::new(nbatch, args.batch);

    for batch in 0..nbatch {
        let data : Vec<TrajectoryPoint> =
        (0..args.batch_size).into_par_iter()
                            .map( |evt| {
                                let evt          = evt + batch * args.batch_size;
                                let starting_pos = sampler.sample(&geometry);
                                tracker.propagate_from(starting_pos)
                                       .into_iter()
                                       .enumerate()
                                       .map(|(i, p)| TrajectoryPoint{ event: evt as u32
                                                                    , x    : p.x as f32
                                                                    , y    : p.y as f32
                                                                    , z    : p.z as f32
                                                                    , t    : i as f32 * conf.t_step as f32
                                                                    })
                                       .collect::<Vec<TrajectoryPoint>>()
                            })
                            .flatten()
                            .collect();
        writer.write_many(data)?;
        pb.inc(1);
    }
    pb.finish();

    Ok(())
}
