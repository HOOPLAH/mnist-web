use std::fs::File;
use std::io::{BufReader, BufWriter, Read, Write};
use std::path::Path;

use deeplearn::{Graph, VarIndex};
use ga::Array;

use msgpack::{encode, decode};

fn read_var_from_file<P: AsRef<Path>>(path: P, graph: &Graph, v: VarIndex)
                                      -> Result<(), decode::ValueReadError> {
    let ref mut file = BufReader::new(File::open(path).unwrap());
    try!(read_var(file, graph, v));
    Ok(())
}

fn read_var<R: Read>(r: &mut R, graph: &Graph, v: VarIndex)
                     -> Result<(), decode::ValueReadError> {
    let array = try!(read_array(r));
    v.get(graph).set(graph.context(), &array);
    Ok(())
}

fn write_var_to_file<P: AsRef<Path>>(path: P, graph: &Graph, v: VarIndex)
                                     -> Result<(), encode::ValueWriteError> {
    let ref mut file = BufWriter::new(File::open(path).unwrap());
    try!(write_var(file, graph, v));
    Ok(())
}

fn write_var<W: Write>(w: &mut W, graph: &Graph, v: VarIndex)
                       -> Result<(), encode::ValueWriteError> {
    try!(write_array(w, v.get(graph).get(graph.context())));
    Ok(())
}

fn read_array<R: Read>(reader: &mut R) -> Result<Array<f32>, decode::ValueReadError> {
    // Read the shape
    let dim_count = try!(decode::read_array_size(reader));
    let mut shape: Vec<usize> = vec![];
    for _ in 0..dim_count {
        shape.push(try!(decode::read_array_size(reader)) as usize);
    }

    // Calculate the buffer length
    let buf_len = shape.iter().fold(1, |a, b| a*b);

    // Read the buffer
    let mut buf: Vec<f32> = vec![];
    for _ in 0..buf_len {
        buf.push(try!(decode::read_f32(reader)));
    }

    Ok(Array::from_vec(shape, buf))
}

fn write_array<W: Write>(writer: &mut W, array: Array<f32>) -> Result<(), encode::ValueWriteError> {
    try!(encode::write_array_len(writer, array.shape().len() as u32)); // Write the dimension count

    // Write all of the dimensions
    for d in array.shape() {
        try!(encode::write_array_len(writer, *d as u32));
    }

    // Write the array contents
    for f in array.buffer() {
        try!(encode::write_f32(writer, *f));
    }

    Ok(())
}
