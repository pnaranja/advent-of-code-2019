//! Given two wire paths, find the closest point to origin where they intersect
//! Return the distance of that point
//!
//! For a wire path: record every single point of its path
//! Then collect all points that match the other wire path
//! Calculate the "Manhattan distance" of each point and return the point that has the lowest distance

fn create_points_per_path_cmd(origin: (i32, i32), cmd: &str) -> Option<Vec<(i32, i32)>> {
    let direction: &str = cmd.get(0..1).unwrap(); //Get the first character (expecting R,L,U,D)
    let distance: i32 = cmd.get(1..).unwrap().parse::<i32>().unwrap(); //Get the rest of the characters (expecting numbers)
    let (origin_x, origin_y) = origin;

    match direction {
        "R" => Some(
            (1..distance + 1)
                .map(|i| (origin_x + i, origin_y))
                .collect(),
        ),
        "L" => Some(
            (1..distance + 1)
                .map(|i| (origin_x - i, origin_y))
                .collect(),
        ),
        "U" => Some(
            (1..distance + 1)
                .map(|i| (origin_x, origin_y + i))
                .collect(),
        ),
        "D" => Some(
            (1..distance + 1)
                .map(|i| (origin_x, origin_y - i))
                .collect(),
        ),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_points_per_path_cmd() {
        assert_eq!(
            create_points_per_path_cmd((0, 0), "R3").unwrap(),
            vec!((1, 0), (2, 0), (3, 0))
        );
        assert_eq!(
            create_points_per_path_cmd((4, 0), "L3").unwrap(),
            vec!((3, 0), (2, 0), (1, 0))
        );
        assert_eq!(
            create_points_per_path_cmd((45, 0), "U3").unwrap(),
            vec!((45, 1), (45, 2), (45, 3))
        );
        assert_eq!(
            create_points_per_path_cmd((1, 0), "D3").unwrap(),
            vec!((1, -1), (1, -2), (1, -3))
        );
    }
}
