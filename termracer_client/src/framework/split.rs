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
