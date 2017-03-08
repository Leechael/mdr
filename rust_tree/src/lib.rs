#[macro_use] extern crate cpython;
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

py_module_initializer!(libtreefuncs, initlibtreefuncs, PyInit_libtreefuncs, |py, m| {
    m.add(py, "__doc__", "Experimental Rust replacement for Cython code in mdr.")?;
    m.add(py, "tree_size", py_fn!(py, py_tree_size(input: &PyObject)))?;
    Ok(())
});

/* Original Cython
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

/*
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
