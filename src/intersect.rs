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

    pub fn acquire(&mut self) -> IntersectionVec {
        let ve : Vec<Intersection> = match self.pool.pop() {
            Some(v) => v,
            None => vec![]
        };
        IntersectionVec { origin: self, internal_vec: Some(ve) }
    }

    pub fn size(&self) -> usize {
        self.pool.len()
    }
}

pub struct IntersectionVec<'a> {
    origin: &'a mut IntersectionSpace,
    internal_vec: Option<Vec<Intersection>>,
}

impl<'a> IntersectionVec<'a> {
    pub fn get(&self, i: usize) -> Option<&Intersection> {
        self.internal_vec.as_ref().and_then(|v|{
            v.get(i)
        })
    }
}

impl<'a> Drop for IntersectionVec<'a> {
    fn drop(&mut self) {
        self.origin.pool.push(self.internal_vec.take().unwrap_or_default()) ;
    }
}

impl Extend<Intersection> for IntersectionVec<'_> {
    fn extend<T: IntoIterator<Item=Intersection>>(&mut self, iter: T) {
        self.internal_vec.get_or_insert_with(||{vec![]}).extend(iter)
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
        }

        assert_eq!(space.size(), 1);
    }

    #[allow(non_snake_case)]
    #[test]
    fn after_multiple_disposals___vectors_are_returned_back_to_pool() {
        let mut space = IntersectionSpace::new();
        let obj = unit_sphere();
        assert_eq!(space.size(), 0);

        {
            let mut v1 = space.acquire();
            let mut v2 = space.acquire();
            let mut v3 = space.acquire();

            // it's not the act of acquiring that increases the pool's size
            assert_eq!(space.size(), 0);
        }

        assert_eq!(space.size(), 3);
    }

}