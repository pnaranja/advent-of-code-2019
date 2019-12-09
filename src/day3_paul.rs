//! Given two wire paths, find the closest point to origin where they intersect
//! Return the distance of that point
//!
//! For a wire path: record every single point of it's path
//! Then collect all points that match the other wire path
//! Calculate the "Manhattan distance" of each point and return the point that has the lowest distance

fn create_points_per_path_cmd(origin : (i32,i32), cmd: &str) -> Vec<(i32,i32)>{
    vec!((0,0))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_points_per_path_cmd(){
	assert_eq!(create_points_per_path_cmd( (0,0), "R3"), vec!((1,0),(2,0),(3,0)));
    }
    
}