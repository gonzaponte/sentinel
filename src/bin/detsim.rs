use std::io:: Result;
use std::path::Path;
use std::rc::Rc;
use std::time::Instant;

use clap::Parser;
use rayon::prelude::*;
use hdf5_metno as hdf5;

use sentinel::invalid_input;
use sentinel::io::{Writer, Hdf5Writer, read_hdf5};
use sentinel::io::hdf5_types::{IonizationHit, SensorHit};
use sentinel::lt::{S1LightTable, S2LightTable, DriftTable};
use sentinel::medium::Medium;
use sentinel::progress::MaybeProgressBar;
use sentinel::random::{poisson, multiexpo, normal, uniform};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct CLI {

    #[arg(short, long)]
    input: String,

    #[arg(short, long)]
    output: String,

    #[arg(long, value_enum, default_value_t=Medium::Xenon)]
    medium: Medium,

    #[arg(short, long)]
    maps: String,

    #[arg(short, long)]
    lifetime: f64,

    #[arg(long, default_value_t=false)]
    overwrite: bool,

    #[arg(short, long, default_value_t=10)]
    threads: usize,

    #[arg(long, default_value_t=false)]
    batch: bool,
}

pub fn main() -> Result<()> {
    let timer = Instant::now();

    let args = CLI::parse();
    let path = Path::new(&args.output);
    if path.exists() & !args.overwrite {
        return invalid_input!("Outfile already exists!");
    }

    let s1_lt = S1LightTable::from_file(&args.maps)?;
    let s2_lt = S2LightTable::from_file(&args.maps)?;
    let drift =   DriftTable::from_file(&args.maps)?;

    let (fractions, constants) = args.medium.time_constants();

    let ofile  = hdf5::File::create(&path.to_str().unwrap()).unwrap();
    let writer = Hdf5Writer::<SensorHit>::new(Rc::new(ofile), "sensor_hits", 1024).unwrap();

    println!("Initialization time: {:?}", timer.elapsed().as_secs_f64());

    let ihits   = read_hdf5::<IonizationHit>(&args.input, "/MC/ionization_hits")?;
    let n_ihits = ihits.len();

    let pb = MaybeProgressBar::new(ihits.len(), args.batch);

    rayon::ThreadPoolBuilder::new().num_threads(args.threads).build_global().unwrap();

    let timer = Instant::now();


    let mut sensor_hits : Vec<SensorHit> =
        ihits.par_iter()
             .flat_map_iter(|ihit| {
                 pb.inc(1);
                 let event = ihit.event;
                 let n_ave = ihit.e as f64 * 1e6 / args.medium.w_s();// hits come in MeV, w_s is in eV
                 let n_ph  = poisson(n_ave) as f64;
                 s1_lt.get(ihit.z, ihit.x, ihit.y)
                      .iter()
                      .map(move |p| (p.sensor_id, poisson(p.prob*n_ph)))
                      .filter(|(_, n)| *n>0)
                      .flat_map(move |(sid, n)| (0..n).map(move |_| (event, sid)))
             })
             .map(|(event, sensor_id)| {
                 let t = multiexpo(&fractions, &constants);
                 SensorHit{event, sensor_id, t: t as f32}
             })
             .collect();

    let mut s2_sensor_hits : Vec<SensorHit> =
        ihits.into_par_iter()
             .flat_map_iter(|ihit| {
                 pb.inc(1);
                 let event                = ihit.event;
                 let n_ave                = ihit.e as f64 * 1e6 / args.medium.w_i(); // hits come in MeV, w_i is in eV
                 let n_ie                 = poisson(n_ave);
                 let (endpoints, ie_time) = drift.get(ihit.z, ihit.x, ihit.y, n_ie, args.lifetime);

                 endpoints.into_iter().map(move |ep| (event, ie_time, ep))
             })
             .flat_map_iter(|(evt, ie_time, ep)| {
                 // HACK: smear arrival time by a tiny bit to emulate diffusion
                 let ie_time_diff = normal(ie_time, 10.0);
                 s2_lt.get(ep.x, ep.y)
                      .iter()
                      .map(|p| (p.sensor_id, poisson(p.prob*args.medium.light_yield())))
                      .filter(|(_, n)| *n>0)
                      .flat_map(move |(sid, n)| (0..n).map(move |_| (evt, ie_time_diff, sid)))
             })
             .map(|(event, ie_time_diff, sensor_id)| {
                 // HACK: uniform emission in time within el emission range
                 let t = ie_time_diff + uniform(0.0, 100.0) + multiexpo(&fractions, &constants);
                 SensorHit{event, sensor_id, t: t as f32}
             })
             .collect();

    pb.finish();

    sensor_hits.append(&mut s2_sensor_hits);
    sensor_hits.sort_by(|a, b| b.event.cmp(&a.event));
    writer.write_many(sensor_hits)?;

    let exe_time = timer.elapsed().as_secs_f64();
    println!( "Execution time for {} ihits: {:.2} s => {:.1} ihit/s or {:.8} s/ihit"
            , n_ihits
            , exe_time
            , n_ihits as f64 / exe_time
            , exe_time / n_ihits as f64
            );
    Ok(())
}
