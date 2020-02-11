use crate::*;

/// Arrange objects from a scene into a hierarchical
/// structure to speed up finding intersections.
///
/// Where a scene contains n = 10k objects or more, computing
/// intersections along a ray can be a very costly O(n) search.
/// This function groups together those objects that occupy
/// similar regions of space, hoping to speed up the search by
/// allowing the intersection algorithm to quickly discard irrelevant
/// regions and the objects therein, only computing
/// intersections for those objects in regions that
/// intersect the ray.
///
/// This algorithm defines regions by recursively performing a
/// binary partition on the original bounding box encompassing all the
/// objects in the scene.  Objects are then placed into the
/// smallest region that fully contains them.
pub fn binary_partition(depth: usize, scene: Vec<Object>) -> Object {
    // TODO make the depth parameter unnecessary.  Decide an appropriate depth internally.
    let box_map = bbox_map(depth, scene);
    box_map.groups()
}

pub fn bbox_map(depth: usize, scene: Vec<Object>) -> BoundingBoxMap {
    let flattened = flatten(&scene);
    let mut box_map = BoundingBoxMap::create(depth, Bounds::enclose(&flattened));
    for o in flattened {
        box_map.put(o);
    }
    box_map
}

fn bounding_box_tree(depth: usize, enclosure: Bounds) -> Vec<Bounds> {
    let mut v = vec![];
    v.push(enclosure.clone());
    for i in 1..(depth+1) {
        append_generation(&mut v, i);
    }
    return v;
}

fn count_in_generation(gen: usize) -> usize {
    1 << (3 * gen)
}

fn tree_size(n: usize) -> usize {
    let mut s = 1;
    for i in 1..=n {
        s += count_in_generation(i);
    }
    return s;
}

fn first_item_of_gen(n: usize) -> usize {
    if n <= 0 {
        return 0;
    }
    tree_size(n - 1)
}

// Defined only for gen >= 1.  v must always be prepared with at least
// the first bounding box for generation 0 in it.
fn append_generation(v: &mut Vec<Bounds>, gen: usize) {
    let prev_gen = gen - 1;
    let first_item_prev_gen = first_item_of_gen(prev_gen);
    for i in 0..count_in_generation(prev_gen) {
        append_bbox(v, &v[first_item_prev_gen + i].clone());
    }
}

fn append_bbox(boxes: &mut Vec<Bounds>, enclosure: &Bounds) {
    let min = enclosure.min();
    let max = enclosure.max();
    let one = (enclosure.max() - enclosure.min()).scale(0.5);
    let i = vector(one.x(), 0.0, 0.0);
    let j = vector(0.0, one.y(), 0.0);
    let k = vector(0.0, 0.0, one.z());

    boxes.push(Bounds::new(min, min + one));
    boxes.push(Bounds::new(min + k, max - i - j));
    boxes.push(Bounds::new(min + j, max - i - k));
    boxes.push(Bounds::new(min + j + k, max - i));

    boxes.push(Bounds::new(min + i, min + i + one));
    boxes.push(Bounds::new(min + i + k, min + i + k + one));
    boxes.push(Bounds::new(min + i + j, min + i + j + one));
    boxes.push(Bounds::new(min + i + j + k, min + i + j + k + one));
}

#[derive(Debug)]
pub struct BoundingBoxMap {
    depth: usize,
    bounding_boxes: Vec<Bounds>,
    mapping: HashMap<usize, Vec<Object>>,
}

impl BoundingBoxMap {
    pub fn create(depth: usize, enclosure: Bounds) -> BoundingBoxMap {
        BoundingBoxMap {
            depth: depth,
            bounding_boxes: bounding_box_tree(depth, enclosure),
            mapping: HashMap::with_capacity(tree_size(depth))
        }
    }

    pub fn put(&mut self, o: Object) -> Bounds {
        for i in (0..self.bounding_boxes.len()).rev() {
            if self.bounding_boxes[i].contains(&o.bounds()) {
                return self.place(i, o);
            }
        }
        return self.place(0, o);
    }

    pub fn iter(&self) -> BoundingBoxMapIterator {
        BoundingBoxMapIterator::new(self)
    }

    fn place(&mut self, index: usize, o: Object) -> Bounds {
        if self.mapping.contains_key(&index) {
            self.mapping.get_mut(&index).unwrap().push(o);
        } else {
            self.mapping.insert(index, vec![o]);
        }

        self.bounding_boxes[index].clone()
    }

    pub fn groups(&self) -> Object {
        match self.create_node(0, 0) {
            None => group(vec![]),
            Some(obj) => obj,
        }
    }

    fn create_node(&self, index: usize, gen: usize) -> Option<Object> {
        let mut v = vec![];
        let members: Option<Object> = self.create_members(index);

        if members.is_some() {
            v.push(members.unwrap());
        }

        // TODO need to calculate the descendant 8 cells' indices and give them to extend method.
        self.extend_with_children(gen + 1, &mut v);

        if v.len() > 0 {
            Some(group(v))
        } else {
            None
        }
    }

    fn create_members(&self, index: usize) -> Option<Object> {
        match self.mapping.get(&index) {
            None => None,
            Some(v) => Some(group(v.clone())),
        }
    }

    fn extend_with_children(&self, gen: usize, dest: &mut Vec<Object>) {
        let start = first_item_of_gen(gen);
        let end = first_item_of_gen(gen + 1);
        if start < self.bounding_boxes.len() {
            for i in start..end {
                self.create_node(i, gen).and_then(|o| {
                    dest.push(o);
                    Some(())
                });
            }
        }
    }
}

pub struct BoundingBoxMapIterator<'a> {
    current_index: usize,
    map: &'a BoundingBoxMap,
}

impl BoundingBoxMapIterator<'_> {
    fn new(map: &BoundingBoxMap) -> BoundingBoxMapIterator {
        BoundingBoxMapIterator { current_index: 0, map }
    }
}

impl Iterator for BoundingBoxMapIterator<'_> {
    type Item = (usize, Bounds, Vec<Object>);
    fn next(&mut self) -> Option<Self::Item> {
        for i in self.current_index..self.map.bounding_boxes.len() {
            let opt = self.map.mapping.get(&i);
            if opt.is_some() {
                self.current_index = i + 1;
                return Some((i, self.map.bounding_boxes[i], opt.unwrap().clone()))
            } else {
                self.current_index = i + 1;
            }
        }
        return None;
    }
}

/// Takes a list of objects which may contain nested groups.
/// The result is a linear list of objects which have no groups.
/// In other words, the leaf objects from the original list.
pub fn flatten(input: &[Object]) -> Vec<Object> {
    let mut v: Vec<Object> = vec![];
    for o in input {
        if o.is_group() {
            let mut inner = flatten(o.children());
            v.append(&mut inner);
        } else {
            v.push(o.clone());
        }
    }
    v
}


#[cfg(test)]
mod test_partition {
    use super::*;
    #[allow(non_snake_case)]
    #[test]
    fn bounding_box_map___put_associates_object_with_smallest_bounding_box_that_contains_it() {
        let mut box_map = BoundingBoxMap::create(1, Bounds::unit());
        let o = unit_sphere().set_object_to_world_spc(translation(-0.25, -0.25, -0.25) * scaling(0.25, 0.25, 0.25)).clone();
        assert_eq!(box_map.put(o), Bounds::new(point(-1.0, -1.0, -1.0), point(0.0, 0.0, 0.0)));
    }

    #[allow(non_snake_case)]
    #[test]
    fn size_of_box_tree_is_related_to_power_of_2() {
        assert_eq!(bounding_box_tree(0, Bounds::unit()).len(), 1);
        assert_eq!(bounding_box_tree(1, Bounds::unit()).len(), 8 + 1);
        assert_eq!(bounding_box_tree(2, Bounds::unit()).len(), 64 + 8 + 1);
        assert_eq!(bounding_box_tree(3, Bounds::unit()).len(), 512 + 64 + 8 + 1);
        assert_eq!(bounding_box_tree(4, Bounds::unit()).len(), 4096 + 512 + 64 + 8 + 1);
    }

    #[allow(non_snake_case)]
    #[test]
    fn bounding_boxes_depth0() {
        assert_eq!(bounding_box_tree(0, Bounds::unit()), vec![Bounds::unit()]);
    }

    #[allow(non_snake_case)]
    #[test]
    fn bounding_boxes_depth1() {
        assert_eq!(
            bounding_box_tree(1, Bounds::new(point(0.0, 0.0, 0.0), point(1.0, 1.0, 1.0))),
            vec![
                Bounds::new(point(0.0, 0.0, 0.0), point(1.0, 1.0, 1.0)), // 0th lvl
                Bounds::new(point(0.0, 0.0, 0.0), point(0.5, 0.5, 0.5)), // 1st lvl, 0
                Bounds::new(point(0.0, 0.0, 0.5), point(0.5, 0.5, 1.0)), // 1st lvl, 1
                Bounds::new(point(0.0, 0.5, 0.0), point(0.5, 1.0, 0.5)), // 1st lvl, 2
                Bounds::new(point(0.0, 0.5, 0.5), point(0.5, 1.0, 1.0)), // 1st lvl, 3
                Bounds::new(point(0.5, 0.0, 0.0), point(1.0, 0.5, 0.5)), // 1st lvl, 4
                Bounds::new(point(0.5, 0.0, 0.5), point(1.0, 0.5, 1.0)), // 1st lvl, 5
                Bounds::new(point(0.5, 0.5, 0.0), point(1.0, 1.0, 0.5)), // 1st lvl, 6
                Bounds::new(point(0.5, 0.5, 0.5), point(1.0, 1.0, 1.0)), // 1st lvl, 7
            ]
        );
    }

    #[allow(non_snake_case)]
    #[test]
    fn bounding_boxes_depth2() {
        let all = bounding_box_tree(2, Bounds::new(point(0.0, 0.0, 0.0), point(1.0, 1.0, 1.0)));
        assert_eq!(
            &all[9..=16],
            [
                Bounds::new(point(0.00, 0.00, 0.00), point(0.25, 0.25, 0.25)),
                Bounds::new(point(0.00, 0.00, 0.25), point(0.25, 0.25, 0.50)),
                Bounds::new(point(0.00, 0.25, 0.00), point(0.25, 0.50, 0.25)),
                Bounds::new(point(0.00, 0.25, 0.25), point(0.25, 0.50, 0.50)),
                Bounds::new(point(0.25, 0.00, 0.00), point(0.50, 0.25, 0.25)),
                Bounds::new(point(0.25, 0.00, 0.25), point(0.50, 0.25, 0.50)),
                Bounds::new(point(0.25, 0.25, 0.00), point(0.50, 0.50, 0.25)),
                Bounds::new(point(0.25, 0.25, 0.25), point(0.50, 0.50, 0.50)),
            ]
        );
        assert_eq!(
            &all[65..=72],
            [
                Bounds::new(point(0.50, 0.50, 0.50), point(0.75, 0.75, 0.75)),
                Bounds::new(point(0.50, 0.50, 0.75), point(0.75, 0.75, 1.00)),
                Bounds::new(point(0.50, 0.75, 0.50), point(0.75, 1.00, 0.75)),
                Bounds::new(point(0.50, 0.75, 0.75), point(0.75, 1.00, 1.00)),
                Bounds::new(point(0.75, 0.50, 0.50), point(1.00, 0.75, 0.75)),
                Bounds::new(point(0.75, 0.50, 0.75), point(1.00, 0.75, 1.00)),
                Bounds::new(point(0.75, 0.75, 0.50), point(1.00, 1.00, 0.75)),
                Bounds::new(point(0.75, 0.75, 0.75), point(1.00, 1.00, 1.00)),
            ]
        );
    }

    #[allow(non_snake_case)]
    #[test]
    fn bounding_box_iterator___no_elements___returns_none() {
        let map = BoundingBoxMap::create(2, Bounds::unit());
        assert_eq!(map.iter().next(), None);
    }

    #[allow(non_snake_case)]
    #[test]
    fn bounding_box_iterator___one_element___gets_returned() {
        let mut map = BoundingBoxMap::create(2, Bounds::unit());
        map.put(glass_sphere());
        let mut iter = map.iter();
        assert_eq!(iter.next(), Some((0, Bounds::unit(), vec![glass_sphere()])));
        assert_eq!(iter.next(), None);
    }

    #[allow(non_snake_case)]
    #[test]
    fn bounding_box_iterator___multiple_entries___returned_in_order_of_bounding_box_they_occupy() {
        let s10 = unit_sphere().set_object_to_world_spc( translation(-0.25, -0.25, -0.25) * scaling(0.25, 0.25, 0.25)).clone();
        let s11 = unit_sphere().set_object_to_world_spc( translation(-0.25, -0.25,  0.25) * scaling(0.25, 0.25, 0.25)).clone();
        let s12 = unit_sphere().set_object_to_world_spc( translation(-0.25,  0.25, -0.25) * scaling(0.25, 0.25, 0.25)).clone();
        let s13 = unit_sphere().set_object_to_world_spc( translation(-0.25,  0.25,  0.25) * scaling(0.25, 0.25, 0.25)).clone();

        let mut map = BoundingBoxMap::create(2, Bounds::unit());
        map.put(s10.clone());
        map.put(s11.clone());
        map.put(s12.clone());
        map.put(s13.clone());
        let mut iter = map.iter();
        assert_eq!(iter.next(), Some((16, Bounds::new(point(-0.5, -0.5, -0.5), point(0.0, 0.0, 0.0)), vec![s10])));
        assert_eq!(iter.next(), Some((23, Bounds::new(point(-0.5, -0.5,  0.0), point(0.0, 0.0, 0.5)), vec![s11])));
        assert_eq!(iter.next(), Some((30, Bounds::new(point(-0.5,  0.0, -0.5), point(0.0, 0.5, 0.0)), vec![s12])));
        assert_eq!(iter.next(), Some((37, Bounds::new(point(-0.5,  0.0,  0.0), point(0.0, 0.5, 0.5)), vec![s13])));
        assert_eq!(iter.next(), None);
    }
}
