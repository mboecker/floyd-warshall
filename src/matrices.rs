use std::fmt;

/// This represents a sequence of nodes.
#[derive(Clone, Debug)]
pub struct Path<T> {
    v: Vec<T>,
    len: usize,
    exists: bool,
}

impl<T> Path<T>
where
    T: Clone,
{
    pub(crate) fn set_vector(&mut self, t: Vec<T>) {
        self.v = t
    }

    pub fn get_slice<'a>(&'a self) -> &'a [T] {
        &self.v
    }

    pub fn iter<'a>(&'a self) -> impl DoubleEndedIterator<Item = &'a T> {
        self.v.iter()
    }

    pub fn len(&self) -> usize {
        assert!(self.exists);
        self.len
    }

    pub(crate) fn set_len(&mut self, v: usize) {
        self.len = v;
        self.exists = true;
    }

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

/// This matrix is a solution to the APSP problem, calculated by the Floyd-Warshall algorithm. It contains the intermediate nodes on the shortest path between every two nodes.
pub struct PathMatrix {
    m: Box<[Path<usize>]>,
    n: usize,
}

impl PathMatrix {
    /// Creates a new ```PathMatrix``` with the given dimension (n * n).
    pub fn new(n: usize) -> PathMatrix {
        let m = vec![Path::default(); n * n].into();
        PathMatrix { m, n }
    }

    /// This method computes the "inner index" into the ```Vec``` by using the given X-Y-coordinates into the matrix.
    fn idx(&self, mut i: usize, mut j: usize) -> usize {
        // We only fill one half of the matrix.
        if i > j {
            ::std::mem::swap(&mut i, &mut j);
        }
        assert!(i <= j);

        i + self.n * j
    }

    /// This method returns the value at the given position.
    pub fn get_path_len(&self, i: usize, j: usize) -> usize {
        let idx = self.idx(i, j);
        self.m[idx].len()
    }

    /// This method returns the shortest path possible between i and i.
    pub fn get_path(&self, i: usize, j: usize) -> &Path<usize> {
        let idx = self.idx(i, j);
        &self.m[idx]
    }

    /// This method returns the shortest path possible between i and i as an iterator.
    pub fn get_path_iter<'a>(
        &'a self,
        i: usize,
        j: usize,
    ) -> impl DoubleEndedIterator<Item = &'a usize> {
        let idx = self.idx(i, j);
        self.m[idx].iter()
    }

    pub fn does_path_exist(&self, i: usize, j: usize) -> bool {
        let idx = self.idx(i, j);
        self.m[idx].exists()
    }

    pub(crate) fn get_path_mut(&mut self, i: usize, j: usize) -> &mut Path<usize> {
        let idx = self.idx(i, j);
        &mut self.m[idx]
    }

    /// This method updates the value at the given position.
    pub fn set_path_len(&mut self, i: usize, j: usize, v: usize) {
        let idx = self.idx(i, j);
        self.m[idx].set_len(v);
    }
}

impl fmt::Debug for PathMatrix {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use std::result::Result;

        for j in 0..self.n {
            let from = j * self.n;
            let to = j * self.n + j + 1;
            writeln!(f, "{:?}", &self.m[from..to])?
        }

        Result::Ok(())
    }
}

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