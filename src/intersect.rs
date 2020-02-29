use crate::*;
use std::iter::Extend;
use std::ops::Index;
use std::mem;
use std::borrow::Borrow;

pub struct IntersectionSpace {
    pool: Vec<Vec<Intersection>>,
}

impl IntersectionSpace {
    pub fn new() -> IntersectionSpace {
        IntersectionSpace {pool: vec![]}
    }

    pub fn acquire(&mut self) -> Vec<Intersection> {
        match self.pool.pop() {
            Some(mut v) => {
                v
            },
            None => Vec::with_capacity(16)
        }
    }

    pub fn place_back(&mut self, mut v: Vec<Intersection>) {
        v.clear();
        self.pool.push(v);
    }

    pub fn size(&self) -> usize {
        self.pool.len()
    }
}

mod test {
    use crate::*;
    use crate::intersect::IntersectionSpace;

    #[allow(non_snake_case)]
    #[test]
    fn acquiring_a_vector___allows_you_to_append_intersections() {
        let mut space = IntersectionSpace::new();
        let obj = unit_sphere();
        let mut v = space.acquire();
        v.extend(Some(intersection(6.0, &obj)));
        assert_eq!(v.get(0), Some(&intersection(6.0, &obj)));

        v.extend(Some(intersection(7.0, &obj)));
        assert_eq!(v.get(0), Some(&intersection(6.0, &obj)));
        assert_eq!(v.get(1), Some(&intersection(7.0, &obj)));
    }

    #[allow(non_snake_case)]
    #[test]
    fn after_disposal___vector_is_moved_back_to_pool() {
        let mut space = IntersectionSpace::new();
        let obj = unit_sphere();
        assert_eq!(space.size(), 0);

        {
            let mut v = space.acquire();
            let i = intersection(6.0, &obj);
            v.extend(Some(i));
            assert_eq!(v.get(0), Some(&intersection(6.0, &obj)));
            space.place_back(v);
        }

        assert_eq!(space.size(), 1);
    }

    #[allow(non_snake_case)]
    #[test]
    fn after_placing_back___on_acquiring_the_vector_again___vector_is_empty() {
        let mut space = IntersectionSpace::new();

        {
            let mut v1 = space.acquire();
            let obj1 = unit_sphere();
            let obj2 = cube();
            v1.push(intersection(4.1, &obj1));
            v1.push(intersection(5.0, &obj2));
            space.place_back(v1);
        }

        {
            let mut v2 = space.acquire();
            assert_eq!(v2.len(), 0);
            space.place_back(v2);
        }
        assert_eq!(space.size(), 1);
    }

    #[test]
    fn acquire_several_vectors___and_use_them() {
        let mut space = IntersectionSpace::new();
        let mut v1 = space.acquire();
        let mut v2 = space.acquire();
        let mut v3 = space.acquire();

        let obj1 = cube();
        let obj2 = unit_sphere();
        let obj3 = glass_sphere();

        v1.push(intersection(3.0, &obj1));
        v2.push(intersection(3.0, &obj2));
        v3.push(intersection(3.0, &obj3));
        space.place_back(v1);
        space.place_back(v2);
        space.place_back(v3);

        assert_eq!(space.size(), 3);
    }

    #[test]
    fn a_vectors_capacity_increases_with_use() {
        let mut space = IntersectionSpace::new();
        let mut v1 = space.acquire();
        let cap1 = v1.capacity();
        println!("{}", v1.capacity());
        let o = unit_sphere();

        for i in 0..30 {
            v1.push(intersection(3.0, &o));
        }
        space.place_back(v1);
        let v2 = space.acquire();
        println!("{}", v2.capacity());
        assert!(cap1 < v2.capacity());
    }
}