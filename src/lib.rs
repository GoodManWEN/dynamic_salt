#[macro_use] mod macros;

use pyo3::prelude::*;
use pyo3::wrap_pyfunction;
use pyo3::types::{PyBytes};
use seahash::hash;

pub trait AnyExt {
    fn type_name(&self) -> &'static str;
}

impl<T> AnyExt for T {
    fn type_name(&self) -> &'static str {
        std::any::type_name::<T>()
    }
}

#[pyfunction]
fn salt_core (
    py: Python,
    input: &[u8],
) -> PyObject {
    let secret = b"MySecret";
    let hash_result = hash(&[input , secret].concat());
    let mut hash_masked: [u8; 8] = [0; 8];
    const MASK_0: u64 = 0xFF00000000000000;
    const MASK_1: u64 = 0x00FF000000000000;
    const MASK_2: u64 = 0x0000FF0000000000;
    const MASK_3: u64 = 0x000000FF00000000;
    const MASK_4: u64 = 0x00000000FF000000;
    const MASK_5: u64 = 0x0000000000FF0000;
    const MASK_6: u64 = 0x000000000000FF00;
    const MASK_7: u64 = 0x00000000000000FF;
    hash_masked[0] = ((hash_result & MASK_0) >> 56) as u8;
    hash_masked[1] = ((hash_result & MASK_1) >> 48) as u8;
    hash_masked[2] = ((hash_result & MASK_2) >> 40) as u8;
    hash_masked[3] = ((hash_result & MASK_3) >> 32) as u8;
    hash_masked[4] = ((hash_result & MASK_4) >> 24) as u8;
    hash_masked[5] = ((hash_result & MASK_5) >> 16) as u8;
    hash_masked[6] = ((hash_result & MASK_6) >> 8) as u8;
    hash_masked[7] = (hash_result & MASK_7) as u8;
    if hash_masked[0] == 0 {
        hash_masked[0] = 1;
    }
    for i in 1..8{
        if hash_masked[i] == 0 {
            hash_masked[i] = hash_masked[i-1];
        }
    }
    let mut mask_cursor: usize = 0;
    let mut output: Vec<u8> = Vec::new();
    let reset_limit = (hash_masked.iter().fold(127u8, |a, &b| a.max(b))) as u16;
    let mut recur_sum:u16 = 127 - 1;
    for &i in input {
        output.push(i);
        recur_sum += i as u16;
        if recur_sum >= reset_limit {
            output.push(hash_masked[mask_cursor]);
            mask_cursor = (mask_cursor + 1) & 7;
            recur_sum %= reset_limit;
        }
    }
    PyBytes::new(py, &output).into()
}

#[pymodule]
fn dsalt(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(salt_core, m)?)?;
    Ok(())
}
