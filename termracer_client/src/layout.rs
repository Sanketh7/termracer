use crate::rect::{Rect};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum HorizontalSplitKind {
    CellsInTop(u16),
    CellsInBottom(u16),
    PercentInTop(u8),
    PercentInBottom(u8),
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum VerticalSplitKind {
    CellsInLeft(u16),
    CellsInRight(u16),
    PercentInLeft(u8),
    PercentInRight(u8),
}

#[derive(Debug, PartialEq, Eq)]
pub enum SplitNode {
    Vertical {
        kind: VerticalSplitKind,
        left: Box<SplitNode>,
        right: Box<SplitNode>,
    },
    Horizontal {
        kind: HorizontalSplitKind,
        top: Box<SplitNode>,
        bottom: Box<SplitNode>,
    },
    Leaf(usize),
}

pub struct Layout {
    split_tree: SplitNode,
    regions: Vec<Rect>,
}

impl Layout {
    pub fn new(bounds: Rect) -> Self {
        Layout {
            // default region covers entire window bounds
            regions: vec![bounds],
            split_tree: SplitNode::Leaf(0),
        }
    }

    pub fn vertical_split(
        &mut self,
        split: VerticalSplitKind,
        region_index: usize,
    ) -> (usize, usize) {
        let region = self
            .regions
            .get_mut(region_index)
            .expect("ERROR: Failed to split region -- invalid region index.");
        let (left, right) = region.vertical_split(split);
        *region = left;
        self.regions.push(right);

        let left_index = region_index;
        let right_index = self.regions.len() - 1;

        let split_node = self
            .get_split_leaf_mut(region_index)
            .expect("ERROR: Failed to split region -- could not find split leaf.");
        *split_node = SplitNode::Vertical {
            kind: split,
            left: Box::new(SplitNode::Leaf(left_index)),
            right: Box::new(SplitNode::Leaf(right_index)),
        };

        (left_index, right_index)
    }

    pub fn horizontal_split(
        &mut self,
        split: HorizontalSplitKind,
        region_index: usize,
    ) -> (usize, usize) {
        let region = self
            .regions
            .get_mut(region_index)
            .expect("ERROR: Failed to split region -- invalid region index.");
        let (top, bottom) = region.horizontal_split(split);
        *region = top;
        self.regions.push(bottom);

        let top_index = region_index;
        let bottom_index = self.regions.len() - 1;

        let split_node = self
            .get_split_leaf_mut(region_index)
            .expect("ERROR: Failed to split region --could not find split leaf.");
        *split_node = SplitNode::Horizontal {
            kind: split,
            top: Box::new(SplitNode::Leaf(top_index)),
            bottom: Box::new(SplitNode::Leaf(bottom_index)),
        };

        (region_index, self.regions.len() - 1)
    }

    pub fn regions(&self) -> &Vec<Rect> {
        &self.regions
    }

    pub fn region(&self, region_index: usize) -> Option<&Rect> {
        self.regions.get(region_index)
    }

    pub fn resize(&mut self, new_bounds: Rect) {
        fn helper(regions: &mut Vec<Rect>, node: &SplitNode, bounds: Rect) {
            match node {
                SplitNode::Vertical {
                    kind,
                    ref left,
                    ref right,
                } => {
                    let (left_bounds, right_bounds) = bounds.vertical_split(*kind);
                    helper(regions, left, left_bounds);
                    helper(regions, right, right_bounds);
                }
                SplitNode::Horizontal {
                    kind,
                    ref top,
                    ref bottom,
                } => {
                    let (top_bounds, bottom_bounds) = bounds.horizontal_split(*kind);
                    helper(regions, top, top_bounds);
                    helper(regions, bottom, bottom_bounds);
                }
                SplitNode::Leaf(i) => regions[*i] = bounds,
            }
        }
        helper(
            &mut self.regions,
            &self.split_tree,
            new_bounds
        );
    }

    fn get_split_leaf_mut(&mut self, region_index: usize) -> Option<&mut SplitNode> {
        fn helper(node: &mut SplitNode, target: usize) -> Option<&mut SplitNode> {
            match node {
                &mut SplitNode::Vertical {
                    kind: _,
                    ref mut left,
                    ref mut right,
                } => helper(left, target).or_else(|| helper(right, target)),
                &mut SplitNode::Horizontal {
                    kind: _,
                    ref mut top,
                    ref mut bottom,
                } => helper(top, target).or_else(|| helper(bottom, target)),
                SplitNode::Leaf(i) => {
                    if *i == target {
                        Some(node)
                    } else {
                        None
                    }
                }
            }
        }
        helper(&mut self.split_tree, region_index)
    }

}

#[cfg(test)]
mod tests {
    use crate::{layout::{VerticalSplitKind, HorizontalSplitKind, Layout, SplitNode}, rect::{Coord, Rect}};

    #[test]
    fn it_splits_vertically() {
        let mut layout = Layout::new(Rect {coord: Coord {row: 0, col: 0}, width: 100, height: 50});
        let (left, right) = layout.vertical_split(VerticalSplitKind::CellsInLeft(30), 0);

        assert_eq!((left, right), (0, 1));
        assert_eq!(layout.regions[left].width, 30);
        assert_eq!(layout.regions[right].width, 70);
    }

    #[test]
    fn it_splits_horizontally() {
        let mut layout = Layout::new(Rect {coord: Coord {row: 0, col: 0}, width: 100, height: 50});
        let (top, bottom) = layout.horizontal_split(HorizontalSplitKind::CellsInTop(30), 0);

        assert_eq!((top, bottom), (0, 1));
        assert_eq!(layout.regions[top].height, 30);
        assert_eq!(layout.regions[bottom].height, 20);
    }

    #[test]
    fn it_gets_split_leaf() {
        let mut layout = Layout::new(Rect {coord: Coord {row: 0, col: 0}, width: 100, height: 50});
        let (top, bottom) = layout.horizontal_split(HorizontalSplitKind::CellsInTop(20), 0);
        let (bottom_left, bottom_right) =
            layout.vertical_split(VerticalSplitKind::CellsInLeft(30), bottom);

        assert_eq!(
            layout.get_split_leaf_mut(top),
            Some(&mut SplitNode::Leaf(top))
        );
        assert_eq!(
            layout.get_split_leaf_mut(bottom_left),
            Some(&mut SplitNode::Leaf(bottom_left))
        );
        assert_eq!(
            layout.get_split_leaf_mut(bottom_right),
            Some(&mut SplitNode::Leaf(bottom_right))
        );
    }

    #[test]
    fn it_resizes() {
        let mut layout = Layout::new(Rect {coord: Coord {row: 0, col: 0}, width: 100, height: 50});
        let (top, bottom) = layout.horizontal_split(HorizontalSplitKind::CellsInTop(20), 0);
        let (bottom_left, bottom_right) =
            layout.vertical_split(VerticalSplitKind::PercentInLeft(40), bottom);

        layout.resize(Rect {coord: Coord {row: 0, col: 0}, width: 75, height: 100});

        assert_eq!(layout.regions[top], Rect {coord: Coord {row: 0, col: 0}, width: 75, height: 20});
        assert_eq!(layout.regions[bottom_left], Rect {coord: Coord {row: 20, col: 0}, width: 30, height: 80});
        assert_eq!(layout.regions[bottom_right], Rect {coord: Coord {row: 20, col: 30}, width: 45, height: 80});
    }
}
