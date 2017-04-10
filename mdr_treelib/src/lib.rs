#[macro_use] extern crate cpython;
#[macro_use] extern crate ndarray;
extern crate num;

//use cpython;
use cpython::{Python, PyObject, PyResult, ObjectProtocol};

#[cfg(test)]
mod tests {
    // TODO: Put some Python test cases in here
    #[test]
    fn it_works() {
    }
}

// Ord is not defined for f64 but PartialOrd is
fn max2<T: PartialOrd>(n1: T, n2: T)->T {
    if n1 > n2 { n1 } else { n2 }
}

fn max3<T: PartialOrd>(n1: T, n2: T, n3: T)->T {
    if n1 > n2 { max2(n1, n3) } else { max2(n2, n3) }
}

fn is_none(token: Python, input: &PyObject) -> bool {
    let none = token.None();
    input == &none
}

pub fn tree_size(py: Python, input: &PyObject) -> PyResult<usize> {
    let mut accum = 1;
    for child in input.iter(py)?.into_iter() {
        accum += tree_size(py, &child?)?
    }
    return Ok(accum)
}

pub fn tree_depth(py: Python, input: &PyObject) -> PyResult<usize> {
    if input.len(py)? == 0 {
        return Ok(1)
    }
    let mut maximum = 0;
    for child in input.iter(py)?.into_iter() {
        maximum = max2(maximum, tree_depth(py, &child?)?);
    }
    Ok(maximum + 1)
}

fn ts_have_same_tags(py: Python, t1: &PyObject, t2: &PyObject) -> PyResult<bool> {
    let t1h = t1.getattr(py, "tag")?.hash(py)?;
    let t2h = t2.getattr(py, "tag")?.hash(py)?;
    Ok(t1h == t2h)
}

// Returns Result(abort with zero?, array to build, length of t1, length of t2)
fn prepare_matrix<T>(py: Python, t1: &PyObject, t2: &PyObject) -> PyResult<(bool, ndarray::Array2<T>, usize, usize)>
  where T: PartialOrd + std::clone::Clone + num::Zero
{
    if is_none(py, t1) || is_none(py, t2) {
        return Ok((true, ndarray::Array2::<T>::zeros((0,0)), 0, 0))
    }
    if !ts_have_same_tags(py, t1, t2)? {
        return Ok((true, ndarray::Array2::<T>::zeros((0,0)), 0, 0))
    }
    let m = t1.len(py)?;
    let n = t2.len(py)?;
    let matrix = ndarray::Array2::<T>::zeros((m+1, n+1));
    return Ok((false, matrix, m, n))
}

pub fn simple_tree_match_rs(py: Python, t1: &PyObject, t2: &PyObject) -> PyResult<u32> {
    let (abort, mut matrix, _, _) = prepare_matrix::<u32>(py, t1, t2)?;
    if abort { return Ok(0) }
    for i in 1..matrix.shape()[0] {
        for j in 1..matrix.shape()[1] {
            let opt1 = matrix[[i, j-1]];
            let opt2 = matrix[[i-1, j]];
            let im1 = &t1.get_item(py, i-1)?;
            let jm1 = &t2.get_item(py, j-1)?;
            let opt3 = matrix[[i-1, j-1]] + simple_tree_match_rs(py, im1, jm1)?;
            matrix[[i,j]] = max3(opt1, opt2, opt3);
        }
    }
    Ok(1 + matrix[[matrix.shape()[0]-1, matrix.shape()[1]-1]])
}

pub fn clustered_tree_match_rs(py: Python, t1: &PyObject, t2: &PyObject, c1: f64, c2: f64) -> PyResult<f64> {
    let (abort, mut matrix, m, n) = prepare_matrix::<f64>(py, t1, t2)?;
    if abort { return Ok(0.0) }
    for i in 1..matrix.shape()[0] {
        for j in 1..matrix.shape()[1] {
            let opt1 = matrix[[i, j-1]];
            let opt2 = matrix[[i-1, j]];
            let im1 = &t1.get_item(py, i-1)?;
            let jm1 = &t2.get_item(py, j-1)?;
            let opt3 = matrix[[i-1, j-1]] + clustered_tree_match_rs(py, im1, jm1, m as f64, n as f64)?;
            matrix[[i, j]] = max3(opt1, opt2, opt3);
        }
    }
    if m > 0 || n > 0 {
        Ok(matrix[[m,n]] / (1.0 * max2(c1, c2)))
    } else {
        Ok(matrix[[m,n]] + (1.0 / max2(c1, c2)))
    }
}

// Adapted from https://github.com/scrapinghub/pydepta/blob/master/pydepta/trees_cython.pyx
pub fn depta_tree_match_rs(py: Python, t1: &PyObject, t2: &PyObject) -> PyResult<f64> {
    let (abort, mut matrix, m, n) = prepare_matrix::<f64>(py, t1, t2)?;
    if abort { return Ok(0.0) }
    for i in 1..m+1 {
        for j in 1..n+1 {
            matrix[[i, j]] = max2(matrix[[i, j-1]], matrix[[i-1, j]]);
            matrix[[i, j]] = max2(matrix[[i, j]], matrix[[i-1, j-1]] + depta_tree_match_rs(py, &t1.get_item(py, i-1)?, &t2.get_item(py, j-1)?)?);
        }
    }
    return Ok(1. + matrix[[m-1, n-1]])
}

py_module_initializer!(mdrtreelib, init_treelib, PyInit__treelib, |py, m| {
    m.add(py, "__doc__", "Experimental Rust replacement for Cython code in mdr.")?;
    m.add(py, "tree_size", py_fn!(py, tree_size(input: &PyObject)))?;
    m.add(py, "tree_depth", py_fn!(py, tree_depth(input: &PyObject)))?;
    m.add(py, "_simple_tree_match", py_fn!(py, simple_tree_match_rs(t1: &PyObject, t2: &PyObject)))?;
    m.add(py, "_clustered_tree_match", py_fn!(py, clustered_tree_match_rs(t1: &PyObject, t2: &PyObject, c1: f64, f2: f64)))?;
    m.add(py, "depta_tree_match", py_fn!(py, depta_tree_match_rs(t1: &PyObject, t2: &PyObject)))?;
    Ok(())
});
