#[macro_use] extern crate cpython;
#[macro_use] extern crate ndarray;

//use cpython;
use cpython::{Python, PyObject, PyResult, PyIterator, PyClone, ObjectProtocol};

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}

// Recursively iterate the child nodes of a tree. All nodes must be iterables.
fn rs_tree_size(token: Python, input: PyIterator) -> PyResult<usize> {
    let mut accum = 1;  // For this node
    for child in input.into_iter() {
        match child {
            Ok(child_obj) => {
                match child_obj.iter(token) {
                    Ok(child_iter) => {
                        match rs_tree_size(token, child_iter) {
                            Ok(num) => { accum += num },
                            Err(e) => return Err(e),
                        }
                    },
                    Err(e) => return Err(e),
                }
            },
            Err(e) => return Err(e),
        }
    }
    Ok(accum)
}

/* From the below Cython.
 * Clearly, it's a lot longer in rust. But, we get type safety, clear compileable
 * code, and it might be faster.
def tree_size(t):
    if len(t) == 0:
        return 1
    return sum(tree_size(child) for child in t) + 1
*/
pub fn py_tree_size(token: Python, input: &PyObject) -> PyResult<usize> {
    let my_input_ref = input.clone_ref(token);
    match my_input_ref.iter(token) {
        Ok(x_iter) => rs_tree_size(token, x_iter),
        Err(e) => Err(e),
    }
}

fn ts_have_same_tags(py: Python, t1: &PyObject, t2: &PyObject) -> bool {
    if !(t1.hasattr(py, "tag").expect("Hasattr failed?") && t2.hasattr(py, "tag").expect("Hasattr failed?")) {
        return false // TODO This mimics the original Cython behaviour but perhaps a TypeError is more appropriate!
    }
    let t1tag = t1.getattr(py, "tag").expect("Getattr failed for 'tag' field after validating that it exists");
    let t2tag = t2.getattr(py, "tag").expect("Getattr failed for 'tag' field after validating that it exists");
    t1tag.compare(py, t2tag).expect("Comparing strings should be possible.") == std::cmp::Ordering::Equal
}

/* Original Cython
// Looks a bit like Smith-waterman for trees..
def _simple_tree_match(t1, t2):
    if t1 is None or t2 is None:
        return 0
    if t1.tag != t2.tag:
        return 0
    m = np.zeros((len(t1) + 1, len(t2) + 1), np.int)
    for i in range(1, m.shape[0]):
        for j in range(1, m.shape[1]):
            m[i, j] = max(m[i, j - 1], m[i - 1, j], m[i - 1, j - 1] + _simple_tree_match(t1[i - 1], t2[j - 1]))
    return 1 + m[m.shape[0]-1, m.shape[1]-1]
*/
// TODO Untested as yet
pub fn simple_tree_match_rs(py: Python, t1: &PyObject, t2: &PyObject) -> PyResult<u32> {
    let none = py.None();
    if t1 == &none || t2 == &none { println!("One of t1/t2 is None."); return Ok(0) }  // Does this work?
    if !ts_have_same_tags(py, t1, t2) {
        println!("Aborting comparison; not same tags.");
        return Ok(0)
    }
    let t1len = t1.len(py).expect("Len failed on argument t1");
    let t2len = t2.len(py).expect("Len failed on argument t2");
    let mut m = ndarray::Array2::<u32>::zeros((t1len + 1, t2len + 1));
    for i in 1..m.shape()[0] {
        for j in 1..m.shape()[1] {
            let opt1 = m[[i, j-1]]; // [i][j-1];
            let opt2 = m[[i-1, j]]; // [i-1][j];
            let opt3_inc = match simple_tree_match_rs(py,
                &t1.get_item(py, i-1).expect("expected Item at i-1"),
                &t2.get_item(py, j-1).expect("expected Item at j-1")) {
                    Ok(i) => i,
                    Err(e) => return Err(e),
                };
            let opt3 = opt3_inc + m[[i-1, j-1]]; // + opt3_inc;
            // TODO: 3-item generic max() function
            m[[i,j]] = max3(opt1, opt2, opt3);
        }
    }
    Ok(1 + m[[m.shape()[0]-1, m.shape()[1]-1]])  // m[m.shape()[0]-1][m.shape[1]-1])
}

fn max2<T: PartialOrd>(n1: T, n2: T)->T {
    if n1 > n2 {
        n1
    } else {
        n2
    }
}

fn max3<T: PartialOrd>(n1: T, n2: T, n3: T)->T {
    let mut m = n1;
    if n2 > m { m = n2 }
    if n3 > m { m = n3 }
    m
}

// Defaults for c1, c2 = 1.
/* Replaces:
def _clustered_tree_match(t1, t2, c1, c2):
    if t1 is None or t2 is None:
        return 0.0
    if t1.tag != t2.tag:
        return 0.0
    m = len(t1)
    n = len(t2)
    matrix = np.zeros((m+1, n+1), np.float)
    for i from 1 <= i < matrix.shape[0]:
        for j from 1 <= j < matrix.shape[1]:
            matrix[i, j] = max(matrix[i, j - 1], matrix[i - 1, j],
                matrix[i - 1, j - 1] + _clustered_tree_match(t1[i - 1], t2[j - 1], m, n))

    # XXX: m and n?
    if m or n:
        return matrix[m, n] / (1.0 * max(c1, c2))
    else:
        return matrix[m, n] + (1.0 / max(c1, c2))
*/
pub fn clustered_tree_match_rs(py: Python, t1: &PyObject, t2: &PyObject, c1: f64, c2: f64) -> PyResult<f64> {
    let none = py.None();
    if t1 == &none || t2 == &none { return Ok(0.0) }  // Does this work?
    if !ts_have_same_tags(py, t1, t2) {
        return Ok(0.0)
    }
    let m = t1.len(py).expect("Len failed on argument t1");
    let n = t2.len(py).expect("Len failed on argument t2");
    let mut matrix = ndarray::Array2::<f64>::zeros((m+1, n+1));
    for i in 1..matrix.shape()[0] {
        for j in 1..matrix.shape()[1] {
            let opt1 = matrix[[i, j-1]];
            let opt2 = matrix[[i-1, j]];
            let opt3_inc = match clustered_tree_match_rs(py,
                &t1.get_item(py, i-1).expect("expected Item at i-1"),
                &t2.get_item(py, j-1).expect("expected Item at j-1"),
                m as f64,
                n as f64) {
                    Ok(i) => i,
                    Err(e) => return Err(e),
                };
            let opt3 = matrix[[i-1, j-1]] + opt3_inc;
            matrix[[i, j]] = max3(opt1, opt2, opt3);
        }
    }
    if m > 0 || n > 0 {
        Ok(matrix[[m,n]] / (1.0 * max2(c1, c2)))
    } else {
        Ok(matrix[[m,n]] + (1.0 / max2(c1, c2)))
    }
}

py_module_initializer!(libtreefuncs, initlibtreefuncs, PyInit_libtreefuncs, |py, m| {
    m.add(py, "__doc__", "Experimental Rust replacement for Cython code in mdr.")?;
    m.add(py, "tree_size", py_fn!(py, py_tree_size(input: &PyObject)))?;
    m.add(py, "_simple_tree_match", py_fn!(py, simple_tree_match_rs(t1: &PyObject, t2: &PyObject)))?;
    m.add(py, "_clustered_tree_match", py_fn!(py, clustered_tree_match_rs(t1: &PyObject, t2: &PyObject, c1: f64, f2: f64)))?;
    Ok(())
});
