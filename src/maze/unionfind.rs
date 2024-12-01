pub(crate) struct UnionFind {
    data: Vec<isize>
}

impl UnionFind {

    pub(crate) fn new(n: isize) -> Self {
            UnionFind { data: vec![-1; n as usize]
        }
    }
    pub(crate) fn find(&mut self, n: usize) -> isize {
        if self.data[n] < 0 {
            n as isize
        }
        else {
            self.data[n] = self.find(self.data[n] as usize);
            self.data[n]
        }
    }

    pub(crate) fn union(&mut self, x: usize, y: usize) {
        let mut x_root = self.find(x) as usize;
        let mut y_root = self.find(y) as usize;

        if x_root == y_root {
            return;
        }

        if self.data[x_root] < self.data[y_root] {
            std::mem::swap(&mut x_root, &mut y_root);
        }

        self.data[y_root] += self.data[x_root];
        self.data[x_root] = y_root as isize;
    }

    pub(crate) fn num_sets(&self) -> usize {
        let mut num_sets = 0;
        for i in 0..self.data.len() {
            if self.data[i] == -1 {
                num_sets += 1;
            }
        }
        num_sets
    }
}

#[cfg(test)]
mod tests {
    use crate::maze::unionfind::UnionFind;

    #[test]
    fn test_union(){
        let mut uf = UnionFind::new(10);
        assert_eq!(uf.find(1), 1);
        assert_eq!(uf.find(2), 2);
        uf.union(1, 2);
        assert_eq!(uf.find(1), 2);
        assert_eq!(uf.find(2), 2);
        uf.union(1,3);
        uf.union(3,4);

        assert_eq!(uf.find(3), 2);
        assert_eq!(uf.find(4), 2);
        uf.union(5,6);
        uf.union(7,8);
        assert_eq!(uf.find(5), 6);
        assert_eq!(uf.find(6), 6);
        assert_eq!(uf.find(7), 8);
        assert_eq!(uf.find(8), 8);
        uf.union(4, 5);
        uf.union(6,7);
        assert_eq!(uf.find(5), 2);
        assert_eq!(uf.find(6), 2);
        assert_eq!(uf.find(7), 2);
        assert_eq!(uf.find(8), 2);
    }
}