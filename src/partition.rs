use crate::*;
use std::boxed::Box;
use std::borrow::Borrow;
use std::vec::IntoIter;

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

#[derive(Debug)]
struct TreeNode {
    bbox: Bounds,
    members: Vec<Object>,
    children: Option<[Box<TreeNode>; 8]>,
}

impl TreeNode {
    fn with(bbox: Bounds) -> TreeNode {
        TreeNode { bbox, members: vec![], children: None }
    }

    fn put(&mut self, obj: &Object) -> Option<Bounds> {
        if self.bbox.contains(&obj.bounds()) {
            match self.children.as_mut() {
                None => {
                    self.members.push(obj.clone());
                    Some(self.bbox)
                },
                Some(cs) => {
                    for i in cs.iter_mut() {
                        let x = i.put(obj);
                        if x.is_some() {
                            return x;
                        }
                    }
                    self.members.push(obj.clone());
                    return Some(self.bbox);
                }
            }
        } else {
            None
        }
    }

}

fn append_children<'a>(current: &'a TreeNode, acc: &mut Vec<&'a TreeNode>) {
    current.children.as_ref().and_then(|children| {
        for c in children.iter() {
            acc.push(c.borrow());
        }
        Some(())
    });
}

fn bounding_box_tree(depth: usize, enclosure: Bounds) -> TreeNode {
    let mut root = TreeNode::with(enclosure);
    subdivide(&mut root, 0, depth);
    return root;
}

/// Defined only for gen >= 1.  Recursively subdivide the box for this
/// node by generating further child nodes and deciding what their bounding boxes are.
fn subdivide(node: &mut TreeNode, gen: usize, max_depth: usize) {
    if gen < max_depth {
        let min = node.bbox.min();
        let max = node.bbox.max();
        let one = (node.bbox.max() - node.bbox.min()).scale(0.5);
        let i = vector(one.x(), 0.0, 0.0);
        let j = vector(0.0, one.y(), 0.0);
        let k = vector(0.0, 0.0, one.z());

        let mut b1: Box<TreeNode> = Box::new(TreeNode::with(Bounds::new(min, min + one)));
        let mut b2: Box<TreeNode> = Box::new(TreeNode::with(Bounds::new(min + k, max - i - j)));
        let mut b3: Box<TreeNode> = Box::new(TreeNode::with(Bounds::new(min + j, max - i - k)));
        let mut b4: Box<TreeNode> = Box::new(TreeNode::with(Bounds::new(min + j + k, max - i)));
        let mut b5: Box<TreeNode> = Box::new(TreeNode::with(Bounds::new(min + i, min + i + one)));
        let mut b6: Box<TreeNode> = Box::new(TreeNode::with(Bounds::new(min + i + k, min + i + k + one)));
        let mut b7: Box<TreeNode> = Box::new(TreeNode::with(Bounds::new(min + i + j, min + i + j + one)));
        let mut b8: Box<TreeNode> = Box::new(TreeNode::with(Bounds::new(min + i + j + k, min + i + j + k + one)));

        let nextgen = gen + 1;
        subdivide(b1.as_mut(), nextgen, max_depth);
        subdivide(b2.as_mut(), nextgen, max_depth);
        subdivide(b3.as_mut(), nextgen, max_depth);
        subdivide(b4.as_mut(), nextgen, max_depth);
        subdivide(b5.as_mut(), nextgen, max_depth);
        subdivide(b6.as_mut(), nextgen, max_depth);
        subdivide(b7.as_mut(), nextgen, max_depth);
        subdivide(b8.as_mut(), nextgen, max_depth);

        node.children = Some([b1, b2, b3, b4, b5, b6, b7, b8]);
    }
}

#[derive(Debug)]
pub struct BoundingBoxMap {
    bounding_boxes: TreeNode,
}

impl BoundingBoxMap {
    pub fn create(depth: usize, enclosure: Bounds) -> BoundingBoxMap {
        BoundingBoxMap {
            bounding_boxes: bounding_box_tree(depth, enclosure),
        }
    }

    pub fn put(&mut self, o: Object) -> Option<Bounds> {
        self.bounding_boxes.put(&o)
    }

    pub fn iter(&self) -> IntoIter<(Bounds, Vec<Object>)> {
        let mut v: Vec<(Bounds, Vec<Object>)> = vec![];
        let mut visitor = |node: &TreeNode| {
            if node.members.len() > 0 {
                v.push((node.bbox, node.members.clone()));
            }
        };
        self.breadth_first_traverse(&mut visitor);
        v.into_iter()
    }

    fn bounds_vec(&self) -> Vec<Bounds> {
        let mut v: Vec<Bounds> = vec![];
        self.breadth_first_traverse(&mut |node: &TreeNode|{
            v.push(node.bbox);
        });
        v
    }

    pub fn breadth_first_traverse<F>(&self, consumer: &mut F)
        where F: FnMut(&TreeNode) -> ()
    {
        let mut nodes: Vec<&TreeNode> = vec![&self.bounding_boxes];
        while !nodes.is_empty() {
            let i = nodes.remove(0);
            consumer(i);
            append_children(i, &mut nodes);
        }
    }

    pub fn groups(&self) -> Object {
        match create_node(&self.bounding_boxes) {
            None => group(vec![]),
            Some(obj) => obj,
        }
    }
}

fn create_node(node: &TreeNode) -> Option<Object> {
    let mut node_vec = vec![];
    if node.members.len() > 0 {
        node_vec.push(group(node.members.clone()));
    }
    node.children.as_ref().and_then(|arr|{
        for i in arr.iter() {
            create_node(i.borrow()).and_then(|o| {
                node_vec.push(o);
                Some(())
            });
        }
        Some(())
    });

    if node_vec.len() > 0 {
        Some(group(node_vec))
    } else {
        None
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
        assert_eq!(box_map.put(o),
                   Some(Bounds::new(point(-1.0, -1.0, -1.0),
                                    point(0.0, 0.0, 0.0))));
    }

    fn vec_of_bounds(tree: BoundingBoxMap) -> Vec<Bounds> {
        tree.bounds_vec()
    }

    #[allow(non_snake_case)]
    #[test]
    fn size_of_box_tree_is_related_to_power_of_2() {
        assert_eq!(vec_of_bounds(BoundingBoxMap::create(0, Bounds::unit())).len(), 1);
        assert_eq!(vec_of_bounds(BoundingBoxMap::create(1, Bounds::unit())).len(), 8 + 1);
        assert_eq!(vec_of_bounds(BoundingBoxMap::create(2, Bounds::unit())).len(), 64 + 8 + 1);
        assert_eq!(vec_of_bounds(BoundingBoxMap::create(3, Bounds::unit())).len(), 512 + 64 + 8 + 1);
        assert_eq!(vec_of_bounds(BoundingBoxMap::create(4, Bounds::unit())).len(), 4096 + 512 + 64 + 8 + 1);
    }

    #[allow(non_snake_case)]
    #[test]
    fn bounding_boxes_depth0() {
        assert_eq!(vec_of_bounds(BoundingBoxMap::create(0, Bounds::unit())),
                   vec![Bounds::unit()]);
    }

    #[allow(non_snake_case)]
    #[test]
    fn bounding_boxes_depth1() {
        assert_eq!(
            vec_of_bounds(BoundingBoxMap::create(1, Bounds::new(point(0.0, 0.0, 0.0), point(1.0, 1.0, 1.0)))),
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
        let enclosure = Bounds::new(point(0.0, 0.0, 0.0), point(1.0, 1.0, 1.0));
        let all = vec_of_bounds(BoundingBoxMap::create(2, enclosure));
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
        assert_eq!(map.put(glass_sphere()).unwrap(), Bounds::unit());

        let mut iter = map.iter();
        assert_eq!(iter.next(), Some((Bounds::unit(), vec![glass_sphere()])));
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
        assert_eq!(iter.next(), Some((Bounds::new(point(-0.5, -0.5, -0.5), point(0.0, 0.0, 0.0)), vec![s10])));
        assert_eq!(iter.next(), Some((Bounds::new(point(-0.5, -0.5,  0.0), point(0.0, 0.0, 0.5)), vec![s11])));
        assert_eq!(iter.next(), Some((Bounds::new(point(-0.5,  0.0, -0.5), point(0.0, 0.5, 0.0)), vec![s12])));
        assert_eq!(iter.next(), Some((Bounds::new(point(-0.5,  0.0,  0.0), point(0.0, 0.5, 0.5)), vec![s13])));
        assert_eq!(iter.next(), None);
    }
}
