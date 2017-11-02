use std::fmt;
use std::fmt::Debug;

/// This represents a sequence of nodes. The length is also saved, and when ```exists = false```, this means "there is no path".
#[derive(Clone, Debug)]
pub struct Path<T> {
    v: Vec<T>,
    len: usize,
    exists: bool,
}

impl<T> Path<T> {
    pub(crate) fn set_vector(&mut self, t: Vec<T>) {
        self.v = t
    }

    /// Returns the intermediate nodes on this path as a slice.
    pub fn get_slice<'a>(&'a self) -> &'a [T] {
        &self.v
    }

    /// Returns an iterator of the intermediat enodes on this path.
    pub fn iter<'a>(&'a self) -> impl DoubleEndedIterator<Item = &'a T> {
        self.v.iter()
    }

    /// Returns the length of this path.
    pub fn len(&self) -> usize {
        assert!(self.exists);
        self.len
    }

    /// Updates the length of this path. Also removes the "there is not path here"-flag.
    pub(crate) fn set_len(&mut self, v: usize) {
        self.len = v;
        self.exists = true;
    }

    /// Has this path finite length?
    pub fn exists(&self) -> bool {
        self.exists
    }
}

impl<T> AsRef<Vec<T>> for Path<T> {
    fn as_ref(&self) -> &Vec<T> {
        &self.v
    }
}

impl<T> Default for Path<T> {
    fn default() -> Self {
        use std::usize::MAX;
        Path {
            v: Vec::new(),
            len: MAX,
            exists: false,
        }
    }
}

/// This matrix is a solution to the APSP problem, calculated by the Floyd-Warshall algorithm.
/// It contains the intermediate nodes on the shortest path between every two nodes.
#[derive(Debug)]
pub struct PathMatrix<T> {
    m: Box<[Path<T>]>,
    n: usize,
}

impl<T> PathMatrix<T> {
    /// Creates a new ```PathMatrix``` with the given dimension (n * n), where no paths were found yet.
    /// That means, no nodes are yet connected in this matrix.
    pub fn new(n: usize) -> PathMatrix<T> {
        let mut m = vec![];
        let n_elems = 1 + n * (n - 1) / 2;

        for _ in 0..n_elems {
            m.push(Path::default());
        }

        let m = m.into();

        PathMatrix { m, n }
    }

    /// This method computes the "inner index" into the ```Vec``` by using the given X-Y-coordinates into the matrix.
    fn idx(&self, mut i: usize, mut j: usize) -> usize {
        // Because we're only supporting undirected graphs and we only fill one half of the matrix,
        // we can swap the two indices, so that i <= j.
        if i > j {
            ::std::mem::swap(&mut i, &mut j);
        }
        assert!(i <= j);

        if i == j {
            0
        } else {
            // i + self.n * j // This is for the old n x n matrix.
            // j + k + i
            if j < 3 {
                j + i
            } else {
                let k = j - 1;
                self.idx(0, j - 1) + k + i
            }
        }
    }

    /// This method returns the value at the given position.
    pub fn get_path_len(&self, i: usize, j: usize) -> usize {
        let idx = self.idx(i, j);
        self.m[idx].len()
    }

    /// This method returns the shortest path possible between i and i.
    pub fn get_path(&self, i: usize, j: usize) -> &Path<T> {
        let idx = self.idx(i, j);
        &self.m[idx]
    }

    /// This method returns the shortest path possible between i and i as an iterator.
    pub fn get_path_iter<'a>(
        &'a self,
        i: usize,
        j: usize,
    ) -> impl DoubleEndedIterator<Item = &'a T> {
        let idx = self.idx(i, j);
        self.m[idx].iter()
    }

    /// If the matrix contains a path between i and j (which means, it has a set length), this returns true.
    pub fn does_path_exist(&self, i: usize, j: usize) -> bool {
        let idx = self.idx(i, j);
        self.m[idx].exists()
    }

    /// Returns a mutable reference to the path object for the two given nodes.
    pub(crate) fn get_path_mut(&mut self, i: usize, j: usize) -> &mut Path<T> {
        let idx = self.idx(i, j);
        &mut self.m[idx]
    }

    /// This method updates the value at the given position.
    pub fn set_path_len(&mut self, i: usize, j: usize, v: usize) {
        let idx = self.idx(i, j);
        self.m[idx].set_len(v);
    }
}

// impl<T> Debug for PathMatrix<T>
// where
//     T: Debug,
// {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         use std::result::Result;

//         for j in 0..self.n {
//             let from = j * self.n;
//             let to = j * self.n + j + 1;
//             writeln!(f, "{:?}", &self.m[from..to])?
//         }

//         Result::Ok(())
//     }
// }

// /// This matrix is a solution to the APSP problem, calculated by the Floyd-Warshall algorithm. It contains the length of the shortest path for every pair of nodes in a given graph.
// pub struct DistanceMatrix {
//     m: Box<[usize]>,
//     n: usize,
// }

// impl DistanceMatrix {
//     /// Creates a new ```DistanceMatrix``` with the given dimension (n * n).
//     pub(crate) fn new(n: usize) -> DistanceMatrix {
//         use std::usize::MAX;
//         let m = vec![MAX; n * n].into();
//         DistanceMatrix { m, n }
//     }

//     /// This method computes the "inner index" into the ```Vec``` by using the given X-Y-coordinates into the matrix.
//     fn idx(&self, mut i: usize, mut j: usize) -> usize {
//         // We only fill one half of the matrix.
//         if i > j {
//             ::std::mem::swap(&mut i, &mut j);
//         }
//         assert!(i <= j);

//         i + self.n * j
//     }

//     /// This method returns the value at the given position.
//     pub fn get(&self, i: usize, j: usize) -> usize {
//         let idx = self.idx(i, j);
//         self.m[idx]
//     }

//     /// This method updates the value at the given position.
//     pub fn set(&mut self, i: usize, j: usize, v: usize) {
//         let idx = self.idx(i, j);
//         self.m[idx] = v;
//     }
// }

// impl fmt::Debug for DistanceMatrix {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         use std::result::Result;

//         for j in 0..self.n {
//             let from = j * self.n;
//             let to = j * self.n + j + 1;
//             writeln!(f, "{:?}", &self.m[from..to])?
//         }

//         Result::Ok(())
//     }
// }