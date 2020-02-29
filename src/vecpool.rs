use crate::*;

#[derive(Debug)]
pub struct VectorPool {
    store: Vec<Vec<Intersection>>,
}

impl VectorPool {
    pub fn new() -> VectorPool {
        VectorPool { store: vec![]}
    }

    pub fn acquire(&mut self) -> Vec<Intersection> {
        match self.store.pop() {
            Some(v) => v,
            None => Vec::with_capacity(16)
        }
    }

    pub fn place_back(&mut self, mut v: Vec<Intersection>) {
        v.clear();
        self.store.push(v);
    }

    pub fn size(&self) -> usize {
        self.store.len()
    }
}

#[cfg(test)]
mod test {
    use crate::*;
    use crate::vecpool::VectorPool;

    #[allow(non_snake_case)]
    #[test]
    fn acquiring_a_vector___allows_you_to_append_intersections() {
        let mut pool = VectorPool::new();
        let obj = unit_sphere();
        let mut v = pool.acquire();
        v.extend(Some(intersection(6.0, &obj)));
        assert_eq!(v.get(0), Some(&intersection(6.0, &obj)));

        v.extend(Some(intersection(7.0, &obj)));
        assert_eq!(v.get(0), Some(&intersection(6.0, &obj)));
        assert_eq!(v.get(1), Some(&intersection(7.0, &obj)));
    }

    #[allow(non_snake_case)]
    #[test]
    fn after_disposal___vector_is_moved_back_to_pool() {
        let mut pool = VectorPool::new();
        let obj = unit_sphere();
        assert_eq!(pool.size(), 0);

        {
            let mut v = pool.acquire();
            let i = intersection(6.0, &obj);
            v.extend(Some(i));
            assert_eq!(v.get(0), Some(&intersection(6.0, &obj)));
            pool.place_back(v);
        }

        assert_eq!(pool.size(), 1);
    }

    #[allow(non_snake_case)]
    #[test]
    fn after_placing_back___on_acquiring_the_vector_again___vector_is_empty() {
        let mut pool = VectorPool::new();

        {
            let mut v1 = pool.acquire();
            let obj1 = unit_sphere();
            let obj2 = cube();
            v1.push(intersection(4.1, &obj1));
            v1.push(intersection(5.0, &obj2));
            pool.place_back(v1);
        }

        {
            let mut v2 = pool.acquire();
            assert_eq!(v2.len(), 0);
            pool.place_back(v2);
        }
        assert_eq!(pool.size(), 1);
    }

    #[allow(non_snake_case)]
    #[test]
    fn acquire_several_vectors___and_use_them() {
        let mut pool = VectorPool::new();
        let mut v1 = pool.acquire();
        let mut v2 = pool.acquire();
        let mut v3 = pool.acquire();

        let obj1 = cube();
        let obj2 = unit_sphere();
        let obj3 = glass_sphere();

        v1.push(intersection(3.0, &obj1));
        v2.push(intersection(3.0, &obj2));
        v3.push(intersection(3.0, &obj3));
        pool.place_back(v1);
        pool.place_back(v2);
        pool.place_back(v3);

        assert_eq!(pool.size(), 3);
    }

    #[allow(non_snake_case)]
    #[test]
    fn a_vectors_capacity_increases_with_use() {
        let mut pool = VectorPool::new();
        let mut v1 = pool.acquire();
        let cap1 = v1.capacity();
        println!("{}", v1.capacity());
        let o = unit_sphere();

        for i in 0..30 {
            v1.push(intersection(3.0, &o));
        }
        pool.place_back(v1);
        let v2 = pool.acquire();
        println!("{}", v2.capacity());
        assert!(cap1 < v2.capacity());
    }
}