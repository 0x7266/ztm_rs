// Topic: Result & the question mark operator
//
// Requirements:
// * Determine if an employee can access a building using a digital keycard
// * Employees that can access the building are:
//   * Maintenance crews
//   * Marketing department employees
//   * Managers
// * Other employees that work at the company are:
//   * Line supervisors
//   * Kitchen staff
//   * Assembly technicians
// * Ensure that terminated employees cannot access the building
//   regardless of their position
//
// Notes:
// * Use an enum to represent all types of employees
// * Use a struct to store the employee type and whether they are
//   still employed
// * Use a function that returns a Result to determine if the employee
//   may enter the building
// * Print whether the employee may access the building
//   * Must use a function that utilizes the question mark operator to do this

enum Role {
    AssemblyTechnician,
    KitchenStaff,
    LineSupervisor,
    Maintance,
    Manager,
    Marketing,
}

enum Status {
    Employed,
    Terminated,
}

struct Employee {
    role: Role,
    status: Status,
}

impl Employee {
    fn new(role: Role, status: Status) -> Employee {
        Employee { role, status }
    }

    fn try_access(&self) -> Result<String, String> {
        match self {
            Employee {
                status: Status::Terminated,
                ..
            } => Err("🚨 Access denied! Not a employee".to_string()),
            Employee {
                role: Role::Maintance | Role::Manager | Role::Marketing,
                ..
            } => Ok("✅ Access alowed".to_string()),
            _ => Err("🚨 Access dennied".to_string()),
        }
    }

    fn print_access(&self) -> Result<(), String> {
        let access = self.try_access()?;
        println!("{}", access);
        Ok(())
    }
}

fn main() -> Result<(), String> {
    let john = Employee::new(Role::Manager, Status::Employed);
    john.print_access()?;
    let mary = Employee::new(Role::KitchenStaff, Status::Employed);
    mary.print_access()?; // program ends here because this returns the Err of fn print_access(&self) -> Result<(), String> {}
    let peter = Employee::new(Role::Marketing, Status::Terminated);
    peter.print_access()?;
    Ok(())
}
