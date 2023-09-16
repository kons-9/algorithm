pub struct UnionFind {
    par: Vec<usize>,
    rank: Vec<usize>,
}

impl UnionFind {
    pub fn new(n: usize) -> Self {
        UnionFind {
            par: (0..n).collect(),
            rank: vec![0; n],
        }
    }

    fn find_root(&mut self, x: usize) -> usize {
        if self.par[x] == x {
            x
        } else {
            self.par[x] = self.find_root(self.par[x]);
            self.par[x]
        }
    }

    pub fn merge_tree(&mut self, x: usize, y: usize) {
        let x = self.find_root(x);
        let y = self.find_root(y);
        if x == y {
            return;
        }
        if self.rank[x] < self.rank[y] {
            self.par[x] = y;
        } else {
            self.par[y] = x;
            if self.rank[x] == self.rank[y] {
                self.rank[x] += 1;
            }
        }
    }

    pub fn is_same(&mut self, x: usize, y: usize) -> bool {
        self.find_root(x) == self.find_root(y)
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn test_union_find() {
        use super::UnionFind;
        let mut uf = UnionFind::new(10);

        uf.merge_tree(0, 1);
        uf.merge_tree(1, 2);

        uf.merge_tree(3, 4);

        uf.merge_tree(5, 6);

        uf.merge_tree(7, 8);
        uf.merge_tree(8, 9);

        assert!(uf.is_same(0, 2));
        assert!(uf.is_same(1, 2));
        assert!(uf.is_same(0, 1));

        assert!(uf.is_same(3, 4));
        assert!(uf.is_same(4, 3));

        assert!(uf.is_same(5, 6));

        assert!(uf.is_same(7, 9));
        assert!(uf.is_same(8, 9));
        assert!(uf.is_same(7, 8));

        assert!(!uf.is_same(0, 3));
        assert!(!uf.is_same(5, 8));
        assert!(!uf.is_same(2, 7));
        assert!(!uf.is_same(4, 9));
        assert!(!uf.is_same(1, 6));

        uf.merge_tree(0, 9);
        assert!(uf.is_same(0, 9));
        assert!(uf.is_same(1, 9));
        assert!(uf.is_same(2, 7));
        assert!(uf.is_same(1, 8));
        assert!(uf.is_same(2, 8));
        assert!(uf.is_same(0, 1));
        assert!(uf.is_same(8, 9));
    }
}
