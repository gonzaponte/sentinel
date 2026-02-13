use std::fs::{File, read_to_string};
use std::io::{Result, Write};
use std::fmt;

use crate::invalid_input;

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
    Ok(
        read_to_string(filename)?
            .split("\n")
            .enumerate()
            .filter(|(i,_)| *i>=skiprows)
            .map(|(_,l)| process_line(l, delimiter))
            .filter(|data| !data.is_empty() )
            .collect()
    )
}


pub struct CsvWriter {
    file: File,
    delimiter: String,
    ncols: usize,
}

impl CsvWriter {
    pub fn new(filename: &str, delimiter: &str, header: Vec<String>) -> Result<Self> {
        let delimiter  = delimiter.to_string() + " ";
        let file       = File::create(filename)?;
        let mut writer = CsvWriter{file, delimiter, ncols: header.len()};

        writer.write(header)?;
        Ok(writer)
    }

    pub fn write<T: fmt::Display> (&mut self, values: Vec<T>) -> Result<usize> {
        if values.len() != self.ncols {
            return invalid_input!("Unexpected number of columns");
        }
        let values = values.into_iter().map(|v| format!("{}", v)).collect::<Vec<_>>();
        let line   = values.join(&self.delimiter) + "\n";
        self.file.write(line.as_bytes())
    }
}

impl Drop for CsvWriter {
    fn drop(&mut self) {
        self.file.flush().unwrap();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;
    use tempfile::{NamedTempFile, TempDir, tempdir};

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

    fn tempfile(stem: &str) -> (TempDir, String) {
        let dir  = tempdir().unwrap();
        let file = dir.path()
                      .join(stem)
                      .to_str()
                      .unwrap()
                      .to_string();
        (dir, file)
    }

    #[test]
    fn test_csv_writer() {
        let (_dir, filename) = tempfile("test_csv_writer");
        CsvWriter::new( &filename
                      , " "
                      , vec!["a".to_string(), "b".to_string()]
                      ).unwrap();

        let header = read_to_string(filename).unwrap();
        assert_eq!(header, "a  b\n");
    }

    #[test]
    fn test_csv_writer_write_ok() {
        let (_dir, filename) = tempfile("test_csv_writer_write_ok");
        let mut writer       = CsvWriter::new( &filename
                                             , " "
                                             , vec!["a".to_string(), "b".to_string()]
                                             ).unwrap();
        writer.write(vec![1, 2]).unwrap();
        writer.write(vec![3, 4]).unwrap();
        writer.write(vec![5, 6]).unwrap();
        drop(writer); // close file

        let header = read_to_string(filename).unwrap();
        assert_eq!(header, "a  b\n1  2\n3  4\n5  6\n");
    }

    #[test]
    fn test_csv_writer_write_err() {
        let (_dir, filename) = tempfile("test_csv_writer_write_ok");
        let mut writer       = CsvWriter::new( &filename
                                             , " "
                                             , vec!["a".to_string(), "b".to_string()]
                                             ).unwrap();
        assert!(writer.write(vec![1      ]).is_err());
        assert!(writer.write(vec![1, 2   ]).is_ok ());
        assert!(writer.write(vec![1, 2, 3]).is_err());
    }
}
