pub struct MaxSegmentTree<T> {
    data: Vec<T>,
    size: usize,
    default: T,
}

impl<T: Copy + Ord> MaxSegmentTree<T> {
    pub fn new(data: Vec<T>, default: T) -> Self {
        let mut w = data.len();
        let mut size = 2;
        while w > 0 {
            size *= 2;
            w = w / 2;
        }

        let mut t = MaxSegmentTree {
            data: vec![default; size],
            size: size / 2,
            default,
        };
        for (i, val) in data.iter().enumerate() {
            t.update(i, *val);
        }

        t
    }

    pub fn update(&mut self, index: usize, val: T) {
        let mut prev = val;
        let mut current = index + self.size - 1;
        self.data[current] = val;

        loop {
            if current == 0 {
                break;
            }

            current = (current - 1) / 2;

            self.data[current] = self.data[current].max(prev);
            prev = self.data[current];
        }
    }

    pub fn query(&self, s: usize, t: usize) -> T {
        let mut check = vec![(s, t, 0, 0, self.size)];
        let mut answer = self.default;

        while let Some((a, b, k, l, r)) = check.pop() {
            if r <= a || b <= l {
                answer = answer.max(self.default);
                continue;
            }

            if a <= l && r <= b {
                answer = answer.max(self.data[k]);
                continue;
            }

            check.push((a, b, 2 * k + 1, l, (l + r) / 2));
            check.push((a, b, 2 * k + 2, (l + r) / 2, r));
        }

        answer
    }
}

#[test]
fn segment_tree() {
    let t = MaxSegmentTree::<usize>::new(vec![2, 1, 3, 0], 0);
    assert_eq!(t.data, vec![3, 2, 3, 2, 1, 3, 0]);

    assert_eq!(t.query(0, 3), 3);
    assert_eq!(t.query(1, 4), 3);
    assert_eq!(t.query(0, 1), 2);
}
