use std::fs::read_to_string;
use std::io::Result;

fn process_line(line: &str, delimiter: &str) -> Vec<f64> {
    line.trim()
        .split(delimiter)
        .filter(|token| !token.is_empty())
        .map(|token| token.trim())
        .map(|token| token.replace(",", "."))
        .map(|x| x.parse().expect(format!("Could not parse line: {}", x).as_str()))
        .collect()
}

pub fn read_csv(filename: &str, delimiter: &str, skiprows: usize) -> Result<Vec<Vec<f64>>> {
    let data = read_to_string(filename)?
        .split("\n")
        .enumerate()
        .filter(|(i,_)| *i>=skiprows)
        .map(|(_,l)| process_line(l, delimiter))
        .filter(|data| !data.is_empty() )
        .collect::<Vec<Vec<f64>>>();
    Ok(data)
}


#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[test]
    fn test_process_line_simple() {
        let line = "1.23 4.56 7.89";
        let data = process_line(line, " ");
        assert_eq!(data.len(), 3);
        assert_eq!(data[0], 1.23);
        assert_eq!(data[1], 4.56);
        assert_eq!(data[2], 7.89);
    }

    #[test]
    fn test_process_line_extra_spaces() {
        let line = "      1.23        4.56      7.89         ";
        let data = process_line(line, " ");
        assert_eq!(data.len(), 3);
        assert_eq!(data[0], 1.23);
        assert_eq!(data[1], 4.56);
        assert_eq!(data[2], 7.89);
    }

    #[test]
    fn test_process_lines_delimiters() {
        for delimiter in [";", ",", " ", "\t", "|"] {
            let line = format!("  1.23  {}  4.56  {}  7.89  ", delimiter, delimiter);
            let data = process_line(&line, delimiter);
            assert_eq!(data.len(), 3);
            assert_eq!(data[0], 1.23);
            assert_eq!(data[1], 4.56);
            assert_eq!(data[2], 7.89);
        }
    }

    #[test]
    fn test_read_csv_spaces() {
        let mut file = NamedTempFile::new().unwrap();
        writeln!(file, "dump\nsome\nrubbish").unwrap(); // skip 3 rows
        writeln!(file, "0  1  2").unwrap();
        writeln!(file, "3  4  5").unwrap();
        writeln!(file, "6  7  8").unwrap();
        writeln!(file, "9 10 11").unwrap();

        let data  = read_csv(file.path().to_str().unwrap(), " ", 3).expect("Could not read file");
        assert_eq!(data   .len(), 4);
        for row in data.iter() { assert_eq!(row.len(), 3); }
        data.into_iter()
            .flat_map(|row| row.into_iter())
            .enumerate()
            .for_each(|(i,v)| {
                assert_eq!(i, v as usize);
            })
    }

    #[test]
    fn test_read_csv_semicolons() {
        let mut file = NamedTempFile::new().unwrap();
        writeln!(file, "dump\nsome\nrubbish").unwrap(); // skip 3 rows
        writeln!(file, "0 ;  1 ;  2").unwrap();
        writeln!(file, "3 ;  4 ;  5").unwrap();
        writeln!(file, "6 ;  7 ;  8").unwrap();
        writeln!(file, "9 ; 10 ; 11").unwrap();

        let data  = read_csv(file.path().to_str().unwrap(), ";", 3).expect("Could not read file");
        assert_eq!(data   .len(), 4);
        for row in data.iter() { assert_eq!(row.len(), 3); }
        data.into_iter()
            .flat_map(|row| row.into_iter())
            .enumerate()
            .for_each(|(i,v)| {
                assert_eq!(i, v as usize);
            })
    }

    #[test]
    fn test_read_csv_drift_velocity() {
        let fname = "data/drift_velocity_gushchin.dat";
        let data  = read_csv(fname, ";", 1).expect("Could not read file");
        assert_eq!(data   .len(), 67);
        assert_eq!(data[0].len(),  2);
    }


    #[test]
    fn test_read_csv_comsol_slow() {
        let fname = "data/partial_efield.dat";
        let data  = read_csv(fname, " ", 8).expect("Could not read file");
        assert_eq!(data   .len(), 12345 - 8); // the file has 12345 lines in total
        assert_eq!(data[0].len(),  4);
    }

}
