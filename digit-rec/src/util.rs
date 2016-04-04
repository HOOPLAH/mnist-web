use std::io::{Read, Write};
use ga::Array;

use msgpack::{encode, decode};

fn read_array<T: Read>(reader: &mut T) -> Result<Array<f32>, decode::ValueReadError> {
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

fn write_array<T: Write>(writer: &mut T, array: Array<f32>) -> Result<(), encode::ValueWriteError> {
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
