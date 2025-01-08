use io3d::data_types::*;


use super::LinkedProperty3D;

//  //  //  //  //  //  //  //
#[derive(Debug, PartialEq)]
pub struct Evaluation {
    count: Continuous,
    positive_count: Continuous,
}

pub fn evaluate_for_discrete(
    pattern: &LinkedProperty3D<Discrete>,
    property: &LinkedProperty3D<Discrete>,
    position: &RelIJK,
) -> Evaluation {
    let mut result = Evaluation {
        count: 0.,
        positive_count: 0.,
    };

    for i in 0..pattern.grid.get_i_max() {
        for j in 0..pattern.grid.get_j_max() {
            for k in 0..pattern.grid.get_k_max() {
                let pattern_coord = IJK { i, j, k };
                let property_coord = pattern_coord + *position;
                match (
                    pattern.value(&pattern_coord),
                    property.value(&property_coord),
                ) {
                    (Ok(Some(v1)), Ok(Some(v2))) => {
                        result.count += 1.;
                        if v1 == v2 {
                            result.positive_count += 1.;
                        }
                    }
                    _ => (),
                }
            }
        }
    }

    result
}

//  //  //  //  //  //  //  //
//        TESTS             //
//  //  //  //  //  //  //  //
#[cfg(test)]
mod mid_matching {
    use super::*;
    use crate::*;
    static A: Option<Discrete> = Some(1);
    static B: Option<Discrete> = Some(2);
    static C: Option<Discrete> = Some(3);

    #[test]
    fn check_none_2() {
        let pattern_grid = Grid::new(1, 2, 1);
        let pattern_array = [None, B];
        let pattern_data = Property::<Discrete>::from_data(pattern_array.into());
        let pattern = LinkedProperty3D::<Discrete>::from_property(pattern_data, &pattern_grid);

        let grid = Grid::new(3, 3, 1);
        let array = [A, A, A, A, A, A, A, A, C];
        let data = Property::<Discrete>::from_data(array.into());
        let property = LinkedProperty3D::<Discrete>::from_property(data, &grid);

        let position = RelIJK { i: 2, j: 2, k: 0 };
        let ev = evaluate_for_discrete(&pattern, &property, &position);
        assert!(
            ev == Evaluation {
                count: 0.,
                positive_count: 0.
            }
        );
    }
    #[test]
    fn check_none_1() {
        let pattern_grid = Grid::new(1, 2, 1);
        let pattern_array = [C, B];
        let pattern_data = Property::<Discrete>::from_data(pattern_array.into());
        let pattern = LinkedProperty3D::<Discrete>::from_property(pattern_data, &pattern_grid);

        let grid = Grid::new(3, 3, 1);
        let array = [A, A, A, A, A, A, A, A, None];
        let data = Property::<Discrete>::from_data(array.into());
        let property = LinkedProperty3D::<Discrete>::from_property(data, &grid);

        let position = RelIJK { i: 2, j: 2, k: 0 };
        let ev = evaluate_for_discrete(&pattern, &property, &position);
        assert!(
            ev == Evaluation {
                count: 0.,
                positive_count: 0.
            }
        );
    }

    #[test]
    fn check_3() {
        let pattern_grid = Grid::new(1, 2, 1);
        let pattern_array = [C, B];
        let pattern_data = Property::<Discrete>::from_data(pattern_array.into());
        let pattern = LinkedProperty3D::<Discrete>::from_property(pattern_data, &pattern_grid);

        let grid = Grid::new(3, 3, 1);
        let array = [A, A, A, A, A, A, A, A, C];
        let data = Property::<Discrete>::from_data(array.into());
        let property = LinkedProperty3D::<Discrete>::from_property(data, &grid);

        let position = RelIJK { i: 2, j: 2, k: 0 };
        let ev = evaluate_for_discrete(&pattern, &property, &position);
        assert!(
            ev == Evaluation {
                count: 1.,
                positive_count: 1.
            }
        );
    }

    #[test]
    fn check_2() {
        let pattern_grid = Grid::new(1, 2, 1);
        let pattern_array = [C, B];
        let pattern_data = Property::<Discrete>::from_data(pattern_array.into());
        let pattern = LinkedProperty3D::<Discrete>::from_property(pattern_data, &pattern_grid);

        let grid = Grid::new(3, 3, 1);
        let array = [A, A, C, A, A, B, A, A, A];
        let data = Property::<Discrete>::from_data(array.into());
        let property = LinkedProperty3D::<Discrete>::from_property(data, &grid);

        let position = RelIJK { i: 2, j: 0, k: 0 };
        let ev = evaluate_for_discrete(&pattern, &property, &position);
        assert!(
            ev == Evaluation {
                count: 2.,
                positive_count: 2.
            }
        );
    }
    #[test]
    fn check_1() {
        let pattern_grid = Grid::new(2, 1, 1);
        let pattern_array = [C, B];
        let pattern_data = Property::<Discrete>::from_data(pattern_array.into());
        let pattern = LinkedProperty3D::<Discrete>::from_property(pattern_data, &pattern_grid);

        let grid = Grid::new(3, 3, 1);
        let array = [A, A, A, C, B, A, A, A, A];
        let data = Property::<Discrete>::from_data(array.into());
        let property = LinkedProperty3D::<Discrete>::from_property(data, &grid);

        let position = RelIJK { i: 0, j: 1, k: 0 };
        let ev = evaluate_for_discrete(&pattern, &property, &position);
        assert!(
            ev == Evaluation {
                count: 2.,
                positive_count: 2.
            }
        );
    }
}

#[cfg(test)]
mod simple_matching {
    use super::*;
    use crate::*;
    static O_1: Option<Discrete> = Some(1);
    static O_2: Option<Discrete> = Some(2);

    #[test]
    fn check_1_pattern_4() {
        let pattern_grid = Grid::new(1, 1, 1);
        let pattern_array = [O_2];
        let pattern_data = Property::<Discrete>::from_data(pattern_array.into());
        let pattern = LinkedProperty3D::<Discrete>::from_property(pattern_data, &pattern_grid);

        let grid = Grid::new(3, 3, 1);
        let array = [O_1, O_1, O_1, O_2, O_1, O_1, O_1, O_1, O_1];
        let data = Property::<Discrete>::from_data(array.into());
        let property = LinkedProperty3D::<Discrete>::from_property(data, &grid);

        let position = RelIJK { i: 0, j: 1, k: 0 };
        let ev = evaluate_for_discrete(&pattern, &property, &position);
        assert!(
            ev == Evaluation {
                count: 1.,
                positive_count: 1.
            }
        );
    }
    #[test]
    fn check_1_pattern_3() {
        let pattern_grid = Grid::new(1, 1, 1);
        let pattern_array = [O_2];
        let pattern_data = Property::<Discrete>::from_data(pattern_array.into());
        let pattern = LinkedProperty3D::<Discrete>::from_property(pattern_data, &pattern_grid);

        let grid = Grid::new(2, 2, 2);
        let array = [O_1, O_1, O_2, O_1, O_1, O_1, O_1, O_1];
        let data = Property::<Discrete>::from_data(array.into());
        let property = LinkedProperty3D::<Discrete>::from_property(data, &grid);

        let position = RelIJK { i: 0, j: 1, k: 0 };
        let ev = evaluate_for_discrete(&pattern, &property, &position);
        assert!(
            ev == Evaluation {
                count: 1.,
                positive_count: 1.
            }
        );
    }
    #[test]
    fn check_1_pattern_2() {
        let pattern_grid = Grid::new(1, 1, 1);
        let pattern_array = [O_2];
        let pattern_data = Property::<Discrete>::from_data(pattern_array.into());
        let pattern = LinkedProperty3D::<Discrete>::from_property(pattern_data, &pattern_grid);

        let grid = Grid::new(2, 2, 2);
        let array = [O_1, O_1, O_1, O_1, O_1, O_1, O_1, O_2];
        let data = Property::<Discrete>::from_data(array.into());
        let property = LinkedProperty3D::<Discrete>::from_property(data, &grid);

        let position = RelIJK { i: 1, j: 1, k: 1 };
        let ev = evaluate_for_discrete(&pattern, &property, &position);
        assert!(
            ev == Evaluation {
                count: 1.,
                positive_count: 1.
            }
        );
    }
    #[test]
    fn check_1_pattern_1() {
        let pattern_grid = Grid::new(1, 1, 1);
        let pattern_array = [O_2];
        let pattern_data = Property::<Discrete>::from_data(pattern_array.into());
        let pattern = LinkedProperty3D::<Discrete>::from_property(pattern_data, &pattern_grid);

        let grid = Grid::new(2, 2, 2);
        let array = [O_1, O_2, O_1, O_1, O_1, O_1, O_1, O_1];
        let data = Property::<Discrete>::from_data(array.into());
        let property = LinkedProperty3D::<Discrete>::from_property(data, &grid);

        let position = RelIJK { i: 0, j: 0, k: 0 };
        let ev = evaluate_for_discrete(&pattern, &property, &position);
        assert!(
            ev == Evaluation {
                count: 1.,
                positive_count: 0.
            }
        );
    }

    #[test]
    fn check_itself_shift() {
        let pattern_grid = Grid::new(3, 3, 1);
        let pattern_array = [O_2, O_2, O_1, O_2, O_2, O_1, O_1, O_1, O_1];
        let pattern_data = Property::<Discrete>::from_data(pattern_array.into());
        let pattern = LinkedProperty3D::<Discrete>::from_property(pattern_data, &pattern_grid);

        let position = RelIJK { i: 1, j: 1, k: 0 };
        let ev = evaluate_for_discrete(&pattern, &pattern, &position);
        assert!(
            ev == Evaluation {
                count: 4.,
                positive_count: 1.
            }
        );
    }
    #[test]
    fn check_itself() {
        let pattern_grid = Grid::new(3, 3, 1);
        let pattern_array = [O_1, O_1, O_1, O_1, O_2, O_1, O_1, O_1, O_1];
        let pattern_data = Property::<Discrete>::from_data(pattern_array.into());
        let pattern = LinkedProperty3D::<Discrete>::from_property(pattern_data, &pattern_grid);

        let position = RelIJK { i: 0, j: 0, k: 0 };
        let ev = evaluate_for_discrete(&pattern, &pattern, &position);
        assert!(
            ev == Evaluation {
                count: 9.,
                positive_count: 9.
            }
        );
    }
}
