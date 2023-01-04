use std::cmp::min;

pub enum HorizontalSplit {
    CellsinTop(u16),
    CellsInBottom(u16),
    PercentInTop(u8),
    PercentInBottom(u8),
}

pub enum VerticalSplit {
    CellsinLeft(u16),
    CellsInRight(u16),
    PercentInLeft(u8),
    PercentInRight(u8),
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Coord {
    pub row: u16,
    pub col: u16,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Rect {
    pub coord: Coord,
    pub width: u16,
    pub height: u16,
}

impl Rect {
    pub fn vertical_split(&self, split: VerticalSplit) -> (Rect, Rect) {
        let helper = |cells_in_left: u16| -> (Rect, Rect) {
            let left = Rect {
                coord: self.coord,
                width: min(cells_in_left, self.width),
                height: self.height,
            };
            let right = Rect {
                coord: Coord {
                    row: self.coord.row,
                    col: self.coord.col + left.width,
                },
                width: self.width - left.width,
                height: self.height,
            };
            (left, right)
        };

        match split {
            VerticalSplit::CellsinLeft(cells_in_left) => helper(cells_in_left),
            VerticalSplit::CellsInRight(cells_in_right) => {
                let cells_in_left = self.width - min(cells_in_right, self.width);
                helper(cells_in_left)
            }
            VerticalSplit::PercentInLeft(percent_in_left) => {
                assert!(percent_in_left <= 100, "ERROR: Percent cannot exceed 100.");
                let cells_in_left =
                    (((percent_in_left as f32) / 100.0) * (self.width as f32)) as u16;
                helper(cells_in_left)
            }
            VerticalSplit::PercentInRight(percent_in_right) => {
                assert!(percent_in_right <= 100, "ERROR: Percent cannot exceed 100.");
                let cells_in_left =
                    ((((100 - percent_in_right) as f32) / 100.0) * (self.width as f32)) as u16;
                helper(cells_in_left)
            }
        }
    }

    pub fn horizontal_split(&self, split: HorizontalSplit) -> (Rect, Rect) {
        let helper = |cells_in_top: u16| -> (Rect, Rect) {
            let top = Rect {
                coord: self.coord,
                width: self.width,
                height: min(cells_in_top, self.height),
            };
            let bottom = Rect {
                coord: Coord {
                    row: self.coord.row + top.height,
                    col: self.coord.col,
                },
                width: self.width,
                height: self.height - top.height,
            };
            (top, bottom)
        };

        match split {
            HorizontalSplit::CellsinTop(cells_in_top) => helper(cells_in_top),
            HorizontalSplit::CellsInBottom(cells_in_bottom) => {
                let cells_in_top = self.height - min(cells_in_bottom, self.height);
                helper(cells_in_top)
            }
            HorizontalSplit::PercentInTop(percent_in_top) => {
                assert!(percent_in_top <= 100, "ERROR: Percent cannot exceed 100.");
                let cells_in_top =
                    (((percent_in_top as f32) / 100.0) * (self.height as f32)) as u16;
                helper(cells_in_top)
            }
            HorizontalSplit::PercentInBottom(percent_in_bottom) => {
                assert!(
                    percent_in_bottom <= 100,
                    "ERROR: Percent cannot exceed 100."
                );
                let cells_in_top =
                    ((((100 - percent_in_bottom) as f32) / 100.0) * (self.height as f32)) as u16;
                helper(cells_in_top)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Coord, HorizontalSplit, Rect, VerticalSplit};

    #[test]
    fn it_splits_vertically_by_cells() {
        let r1 = Rect {
            coord: Coord { row: 0, col: 0 },
            width: 100,
            height: 50,
        };
        let r2 = r1;

        let (r1l, r1r) = r1.vertical_split(VerticalSplit::CellsinLeft(30));
        assert_eq!(
            r1l,
            Rect {
                coord: Coord { row: 0, col: 0 },
                width: 30,
                height: 50
            }
        );
        assert_eq!(
            r1r,
            Rect {
                coord: Coord { row: 0, col: 30 },
                width: 70,
                height: 50
            }
        );

        let (r2l, r2r) = r2.vertical_split(VerticalSplit::CellsInRight(60));
        assert_eq!(
            r2l,
            Rect {
                coord: Coord { row: 0, col: 0 },
                width: 40,
                height: 50
            }
        );
        assert_eq!(
            r2r,
            Rect {
                coord: Coord { row: 0, col: 40 },
                width: 60,
                height: 50
            }
        );
    }

    #[test]
    fn it_splits_vertically_by_percent() {
        let r1 = Rect {
            coord: Coord { row: 0, col: 0 },
            width: 100,
            height: 50,
        };
        let r2 = r1;

        let (r1l, r1r) = r1.vertical_split(VerticalSplit::PercentInLeft(30));
        assert_eq!(
            r1l,
            Rect {
                coord: Coord { row: 0, col: 0 },
                width: 30,
                height: 50
            }
        );
        assert_eq!(
            r1r,
            Rect {
                coord: Coord { row: 0, col: 30 },
                width: 70,
                height: 50
            }
        );

        let (r2l, r2r) = r2.vertical_split(VerticalSplit::PercentInRight(60));
        assert_eq!(
            r2l,
            Rect {
                coord: Coord { row: 0, col: 0 },
                width: 40,
                height: 50
            }
        );
        assert_eq!(
            r2r,
            Rect {
                coord: Coord { row: 0, col: 40 },
                width: 60,
                height: 50
            }
        );
    }

    #[test]
    fn it_splits_vertically_overflow() {
        let r1 = Rect {
            coord: Coord { row: 0, col: 0 },
            width: 100,
            height: 50,
        };
        let r2 = r1;

        let (r1l, r1r) = r1.vertical_split(VerticalSplit::CellsinLeft(150));
        assert_eq!(
            r1l,
            Rect {
                coord: Coord { row: 0, col: 0 },
                width: 100,
                height: 50
            }
        );
        assert_eq!(
            r1r,
            Rect {
                coord: Coord { row: 0, col: 100 },
                width: 0,
                height: 50
            }
        );

        let (r2l, r2r) = r2.vertical_split(VerticalSplit::CellsInRight(150));
        assert_eq!(
            r2l,
            Rect {
                coord: Coord { row: 0, col: 0 },
                width: 0,
                height: 50
            }
        );
        assert_eq!(
            r2r,
            Rect {
                coord: Coord { row: 0, col: 0 },
                width: 100,
                height: 50
            }
        );
    }

    #[test]
    fn it_splits_horizontally_by_cells() {
        let r1 = Rect {
            coord: Coord { row: 0, col: 0 },
            width: 100,
            height: 50,
        };
        let r2 = r1;

        let (r1t, r1b) = r1.horizontal_split(HorizontalSplit::CellsinTop(30));
        assert_eq!(
            r1t,
            Rect {
                coord: Coord { row: 0, col: 0 },
                width: 100,
                height: 30
            }
        );
        assert_eq!(
            r1b,
            Rect {
                coord: Coord { row: 30, col: 0 },
                width: 100,
                height: 20
            }
        );

        let (r2t, r2b) = r2.horizontal_split(HorizontalSplit::CellsInBottom(10));
        assert_eq!(
            r2t,
            Rect {
                coord: Coord { row: 0, col: 0 },
                width: 100,
                height: 40
            }
        );
        assert_eq!(
            r2b,
            Rect {
                coord: Coord { row: 40, col: 0 },
                width: 100,
                height: 10
            }
        );
    }

    #[test]
    fn it_splits_horizontally_by_percent() {
        let r1 = Rect {
            coord: Coord { row: 0, col: 0 },
            width: 100,
            height: 50,
        };
        let r2 = r1;

        let (r1t, r1b) = r1.horizontal_split(HorizontalSplit::PercentInTop(60));
        assert_eq!(
            r1t,
            Rect {
                coord: Coord { row: 0, col: 0 },
                width: 100,
                height: 30
            }
        );
        assert_eq!(
            r1b,
            Rect {
                coord: Coord { row: 30, col: 0 },
                width: 100,
                height: 20
            }
        );

        let (r2t, r2b) = r2.horizontal_split(HorizontalSplit::PercentInBottom(20));
        assert_eq!(
            r2t,
            Rect {
                coord: Coord { row: 0, col: 0 },
                width: 100,
                height: 40
            }
        );
        assert_eq!(
            r2b,
            Rect {
                coord: Coord { row: 40, col: 0 },
                width: 100,
                height: 10
            }
        );
    }

    #[test]
    fn it_splits_horizontally_overflow() {
        let r1 = Rect {
            coord: Coord { row: 0, col: 0 },
            width: 100,
            height: 50,
        };
        let r2 = r1;

        let (r1t, r1b) = r1.horizontal_split(HorizontalSplit::CellsinTop(70));
        assert_eq!(
            r1t,
            Rect {
                coord: Coord { row: 0, col: 0 },
                width: 100,
                height: 50
            }
        );
        assert_eq!(
            r1b,
            Rect {
                coord: Coord { row: 50, col: 0 },
                width: 100,
                height: 0
            }
        );

        let (r2t, r2b) = r2.horizontal_split(HorizontalSplit::CellsInBottom(60));
        assert_eq!(
            r2t,
            Rect {
                coord: Coord { row: 0, col: 0 },
                width: 100,
                height: 0
            }
        );
        assert_eq!(
            r2b,
            Rect {
                coord: Coord { row: 0, col: 0 },
                width: 100,
                height: 50
            }
        );
    }
}
