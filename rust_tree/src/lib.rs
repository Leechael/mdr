#[macro_use] extern crate cpython;
use cpython::*;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}

// Walks iterator-tree to get total length of elements and elements-of-elements.
// This consumes the Iterator, so make sure to call obj.clone_ref() and pass the
// new reference to this function.
pub fn tree_size(token: Python, input: Result<PyIterator, PyErr>) -> Result<usize, PyErr> {
    match input {
        Err(E) => { Err(E) },
        Ok(tree) => {
            let tree_iter = tree.into_iter();
            if let (0, _) = tree_iter.size_hint() {
                return Ok(1)
            }
            let mut accum = 1;  // 1 for this node
            for child in tree_iter {
                match subtree_size(token.clone(), child) {
                    Ok(num) => { accum += num },
                    Err(E) => return Err(E),
                }
            }
            Ok(accum)
        },
    }
}

fn subtree_size(token: Python, x: Result<PyObject, PyErr>) -> Result<usize, PyErr> {
    if let Ok(pyx) = x {
        let x_cpy = pyx.clone_ref(token.clone());
        let x_maybe_iter = PyIterator::from_object(token.clone(), x_cpy);
        match x_maybe_iter {
            Ok(x_iter) => tree_size(token.clone(), Ok(x_iter)),
            Err(dce) => Err(PyErr::new::<exc::TypeError, String>(token.clone(), format!("Error inside subtree_size, possibly tree node was not an iterator? '{:?}'", dce))),
        }
    } else {
        Err(x.err().expect("Should be an error here?"))
    }
}

// This compiles, but is not usable because the type it specifies is infinite.
use std::iter::IntoIterator;
pub fn inf_tree_size<T: IntoIterator<Item=T>>(tree: T) -> usize {
    tree.into_iter().fold(1, |acc, child| acc + inf_tree_size(child))
}

/* Original Cython
cimport cython
import numpy as np
cimport numpy as np

def tree_size(t):
    if len(t) == 0:
        return 1
    return sum(tree_size(child) for child in t) + 1

@cython.boundscheck(False)
@cython.wraparound(False)
def _simple_tree_match(t1, t2):

    if t1 is None or t2 is None:
        return 0

    if t1.tag != t2.tag:
        return 0

    m = np.zeros((len(t1) + 1, len(t2) + 1), np.int)

    for i from 1 <= i < m.shape[0]:
        for j from 1 <= j < m.shape[1]:
            m[i, j] = max(m[i, j - 1], m[i - 1, j], m[i - 1, j - 1] + _simple_tree_match(t1[i - 1], t2[j - 1]))
    return 1 + m[m.shape[0]-1, m.shape[1]-1]

@cython.boundscheck(False)
@cython.wraparound(False)
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
