enum Part {
    Bolt,
    Panel,
}

struct RobotArm<'a> {
    part: &'a Part,
}

struct AssemblyLine {
    parts: Vec<Part>,
}

fn main() {
    let line = AssemblyLine {
        parts: vec![Part::Bolt, Part::Panel],
    };
    {
        let arm = RobotArm {
            part: &line.parts[0],
        };
    }
}

// lifetime syntax within a function
// fn name<'a>(arg: &'a DataType) -> &'a DataType {}

// lifetime annotations indicate that there exists some owned data that:
// - "lives at least as long" as the borrowed data
// - "outlives or outlasts" the scope of a borrow
// - "exists longer than" the scope of a borrow

// structures utilizing borrowed data must:
// - always be created after the owner was created
// - always be destroyed before the owner is destroyed
