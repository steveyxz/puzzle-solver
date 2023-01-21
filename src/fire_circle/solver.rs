use std::fmt::Debug;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn flicker_test() {
        assert_eq!(
            FireCircle([true, true, false, false, false].to_vec()).flicker(3),
            FireCircle([true, true, true, true, true].to_vec())
        );
        assert_ne!(
            FireCircle([true, true, false, true, false].to_vec()).flicker(1),
            FireCircle([true, true, true, true, true].to_vec())
        )
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct FireCircle(pub Vec<bool>);

pub type SingleSolution = Vec<(FireCircle, i32)>;

impl FireCircle {
    pub fn flicker(&self, index: usize) -> FireCircle {
        let mut cloned = self.clone();
        let length = self.0.len();
        cloned.0[index] = !cloned.0[index];
        if index == length - 1 {
            cloned.0[0] = !cloned.0[0];
        } else {
            cloned.0[index + 1] = !cloned.0[index + 1];
        }
        if index == 0 {
            cloned.0[length - 1] = !cloned.0[length - 1];
        } else {
            cloned.0[index - 1] = !cloned.0[index - 1];
        }
        cloned
    }

    pub fn is_solved(&self) -> bool {
        for ele in &self.0 {
            if !ele {
                return false;
            }
        }
        true
    }

    pub fn solve(
        &self,
        current_trail: &Vec<(FireCircle, i32)>,
        current_solutions: &mut Vec<SingleSolution>,
        depth: i32,
        last_index: i32,
    ) {
        //println!("The depth of this context is: {}", depth);
        let current_state = current_trail.last();
        if current_state.is_some() {
            if current_state.unwrap().0.is_solved() {
                current_solutions.push(current_trail.clone());
                return;
            }
        }
        if depth == 0 {
            return;
        }
        //println!("The current trail is: {:?}", current_trail);
        for flame_i in 0..current_state.unwrap().0 .0.len() {
            if flame_i as i32 == last_index {
                continue;
            }
            let new_circle = current_state.unwrap().0.flicker(flame_i);
            let new_depth = depth - 1;
            let mut new_trail = current_trail.clone();
            new_trail.push((new_circle, flame_i as i32));
            //println!("After flick index {}, new trail {:?}", flame_i, new_trail);
            self.solve(&new_trail, current_solutions, new_depth, flame_i as i32);
        }
    }
}

pub struct Solution {
    pub shortest_solution: SingleSolution,
    pub all_solutions: Vec<SingleSolution>,
}

impl Debug for Solution {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut optimal_solution = String::new();
        let mut counter = 0;
        for (circle, changed_index) in &self.shortest_solution {
            let mut circle_repr = String::new();
            for value in &circle.0 {
                circle_repr.push_str(if *value { "#" } else { "O" });
            }
            if counter == 0 {
                optimal_solution.push_str("Initial State: ");
            } else {
                optimal_solution.push_str(
                    format!(
                        "Step {}, Click circle at position {}: ",
                        counter,
                        changed_index + 1
                    )
                    .as_str(),
                );
            }
            optimal_solution.push_str(circle_repr.as_str());
            optimal_solution.push_str("\n");
            counter += 1;
        }
        optimal_solution.push_str(format!("Total moves: {}\n", counter - 1).as_str());
        optimal_solution.push_str(
            format!("True total moves: {}", true_length(&self.shortest_solution)).as_str(),
        );
        write!(
            f,
            "The shortest solution is as follows:\n{}",
            optimal_solution
        )
    }
}

pub fn true_length(solution: &SingleSolution) -> usize {
    let mut length_counter = 0;

    for (circle, change_index) in solution {
        if *change_index == -1 {
            continue;
        }
        let is_light_operation = circle.0[*change_index as usize];
        if is_light_operation {
            length_counter += 1;
        }
    }

    length_counter as usize
}

pub fn solve(circle: FireCircle, max_depth: i32) -> Result<Solution, &'static str> {
    if max_depth < 1 {
        panic!("Max depth must be 1 or higher");
    }

    if circle.0.len() < 3 {
        panic!("Circle must have 3 or more elements");
    }

    let mut solutions = Vec::<SingleSolution>::new();
    circle.solve(
        &[(circle.clone(), -1)].to_vec(),
        &mut solutions,
        max_depth,
        -1,
    );

    let shortest = solutions.iter().fold(&solutions[0], |acc, item| {
        if true_length(item) < true_length(acc) {
            &item
        } else {
            acc
        }
    });

    Ok(Solution {
        shortest_solution: shortest.clone(),
        all_solutions: solutions,
    })
}
