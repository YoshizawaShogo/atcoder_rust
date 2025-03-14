use cargo_snippet::snippet;

#[snippet("union_find")]
#[allow(dead_code)]
#[derive(Debug)]
struct UnionFind {
    parents: Vec<i32>,
}

#[snippet("union_find")]
#[allow(dead_code)]
impl UnionFind {
    fn new(size: usize) -> Self {
        Self {
            parents: vec![-1; size],
        }
    }
    fn root(&mut self, n: usize) -> usize {
        let p = self.parents[n];
        if p < 0 {
            n
        } else {
            let r = self.root(p as usize);
            self.parents[n] = r as i32;
            r
        }
    }
    fn size(&mut self, n: usize) -> usize {
        let r = self.root(n);
        (-self.parents[r]) as usize
    }
    fn len(&self) -> usize {
        self.parents.len()
    }
    fn is_connected(&mut self, n1: usize, n2: usize) -> bool {
        let r1 = self.root(n1);
        let r2 = self.root(n2);
        r1 == r2
    }
    fn union(&mut self, n1: usize, n2: usize) {
        let mut r1 = self.root(n1);
        let mut r2 = self.root(n2);
        let s1 = self.size(r1);
        let s2 = self.size(r2);

        if s1 < s2 {
            std::mem::swap(&mut r1, &mut r2);
        }
        self.parents[r2] = r1 as i32;
        let size = self.size(r1) + self.size(r2);
        self.parents[r1] = -(size as i32);
    }
    fn groups(&mut self) -> Vec<Vec<usize>> {
        let mut groups = vec![vec![]; self.len()];
        for i in 0..self.len() {
            let r = self.root(i);
            groups[r].push(i);
        }
        groups.into_iter().filter(|g| g.len() != 0).collect()
    }
}

#[test]
fn union_find_test() {
    let mut uf = UnionFind::new(10);
    uf.union(0, 1);
    assert_eq!(uf.size(0), 2);
    assert_eq!(uf.size(0), uf.size(1));
    assert!(uf.is_connected(0, 1));
    assert!(!uf.is_connected(0, 2));

    uf.union(0, 9);
    assert_eq!(uf.groups()[0], vec![0, 1, 9]);
}

#[snippet("potential_union_find")]
#[allow(dead_code)]
#[derive(Debug)]
struct PotentialUnionFind<T> {
    parents: Vec<i32>,
    potentials: Vec<T>,
}

#[snippet("potential_union_find")]
#[allow(dead_code)]
impl<T> PotentialUnionFind<T>
where
    T: Default
        + Clone
        + Copy
        + std::ops::Neg<Output = T>
        + std::ops::Add<Output = T>
        + std::ops::AddAssign
        + std::ops::Sub<Output = T>
        + std::ops::SubAssign
        + std::cmp::PartialEq,
{
    fn new(size: usize) -> Self {
        Self {
            parents: vec![-1; size],
            potentials: vec![T::default(); size],
        }
    }
    fn root(&mut self, n: usize) -> usize {
        let pa = self.parents[n];
        if pa < 0 {
            n
        } else {
            let r = self.root(pa as usize);
            let po = self.potentials[pa as usize];
            self.parents[n] = r as i32;
            self.potentials[n] += po;
            r
        }
    }
    fn potential(&mut self, n: usize) -> T {
        self.root(n);
        self.potentials[n]
    }
    fn potential_diff(&mut self, n1: usize, n2: usize) -> T {
        self.potential(n2) - self.potential(n1)
    }
    fn size(&mut self, n: usize) -> usize {
        let r = self.root(n);
        (-self.parents[r]) as usize
    }
    fn len(&self) -> usize {
        self.parents.len()
    }
    fn is_connected(&mut self, n1: usize, n2: usize) -> bool {
        let r1 = self.root(n1);
        let r2 = self.root(n2);
        r1 == r2
    }
    fn union(&mut self, n1: usize, n2: usize, mut potential: T) {
        let mut r1 = self.root(n1);
        let mut r2 = self.root(n2);

        if self.size(r1) < self.size(r2) {
            std::mem::swap(&mut r1, &mut r2);
            potential = -potential;
        }
        self.parents[r2] = r1 as i32;
        self.potentials[r2] = self.potential(n1) + potential - self.potential(n2);
        let size = self.size(r1) + self.size(r2);
        self.parents[r1] = -(size as i32);
    }
    fn groups(&mut self) -> Vec<Vec<usize>> {
        let mut groups = vec![vec![]; self.len()];
        for i in 0..self.len() {
            let r = self.root(i);
            groups[r].push(i);
        }
        groups.into_iter().filter(|g| g.len() != 0).collect()
    }
    fn is_valid(&mut self, n1: usize, n2: usize, po: T) -> bool {
        if !self.is_connected(n1, n2) {
            return false;
        }
        self.potential(n2) - self.potential(n1) == po
    }
}

#[test]
fn potential_union_find_test() {
    let mut uf = PotentialUnionFind::<i64>::new(10);
    uf.union(0, 1, 3);
    assert_eq!(uf.size(0), 2);
    assert_eq!(uf.size(0), uf.size(1));

    assert_eq!(uf.potential(1), 3);

    assert!(uf.is_connected(0, 1));
    assert!(!uf.is_connected(0, 2));

    uf.union(0, 9, 5);
    assert_eq!(uf.potential_diff(1, 9), 2);
    assert_eq!(uf.groups()[0], vec![0, 1, 9]);
}
