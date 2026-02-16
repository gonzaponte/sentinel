use std::io:: Result;
use std::path::Path;

use clap::Parser;
use nalgebra::Point3;
use indicatif::ProgressBar;

use sentinel::configure::Configure;
use sentinel::geometry::Geometry;
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

    #[arg(short, long)]
    output: String,

    #[arg(long, default_value_t=false)]
    overwrite: bool,
}

pub fn main() -> Result<()> {
    let args = CLI::parse();
    let conf = Configure::new(&args.conf).unwrap();
    let path = Path::new(&args.output);
    if path.exists() & !args.overwrite {
        return invalid_input!("Outfile already exists!");
    }

    let header   = "x0 y0 z0 x1 y1 z1 t".split(" ")
                                        .map(|v| v.to_string())
                                        .collect::<Vec<String>>();
    let mut writer = CsvWriter::new(&path.to_str().unwrap(), " ", header).unwrap();
    let geometry   = Geometry::new(conf.rmin, conf.form_factor, conf.zmin, conf.zmax);
    let field      = Field::from_file(&conf.field_file);
    let tracker    = Tracker::new(field, geometry, conf.t_step);

    let pb = ProgressBar::new(args.n_events as u64);
    for _ in 0..args.n_events {
        let (x0, y0, z0) = (0., 0., -9.);
        let trajectory = tracker.propagate_from(Point3::new(x0, y0, z0));
        let last = trajectory.last().unwrap();
        let (x1, y1, z1) = (last.x, last.y, last.z);
        let t = trajectory.len() as f64 * conf.t_step;
        writer.write(vec![x0, y0, z0, x1, y1, z1, t]).unwrap();
        pb.inc(1);
    }
    pb.finish();

    Ok(())
}


#[cfg(test)]
mod tests {
    use nalgebra::{Point2, Point3, Vector2};
    use sentinel::field_point::FieldPoint;
    use sentinel::field::Field;
    use sentinel::tracker::Tracker;
    use sentinel::geometry::Geometry;


    fn homogeneous_field() -> Field {
        let points   = vec![
            FieldPoint::new(Point2::<f64>::new(0., 11.), 1., 0, Vector2::new(0., -1.)), // startpoint
            FieldPoint::new(Point2::<f64>::new(0.,  1.), 1., 0, Vector2::new(0., -1.)), //
            FieldPoint::new(Point2::<f64>::new(0.,  0.), 1., 0, Vector2::zeros()     ), //   endpoint
        ];
        Field::new(points)
    }

    #[test]
    fn test_it_runs() {
        let geometry = Geometry::new(1., 1., 0., 10.);
        let tracker  = Tracker::new(homogeneous_field(), geometry, 1e-2);
        let t = tracker.propagate_from(Point3::new(0., 0., -9.));
        assert!(t.len() > 100, "Track length too short: {}", t.len());
        assert!(t.last().unwrap().z > -2e-2); // close to zmin
    }

    #[test]
    fn test_it_runs_slow() {
        let geometry = Geometry::new(1., 1., 0., 10.);
        let tracker  = Tracker::new(homogeneous_field(), geometry, 5e-4);
        let t = tracker.propagate_from(Point3::new(0., 0., -9.));

        assert!(t.len() > 1000, "Track length too short: {}", t.len());
        assert!(t.last().unwrap().z > -1e-3); // close to zmin
    }
}
