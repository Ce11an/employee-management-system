use crate::employees::structs::Employee;
use crate::employees::structs::Wage;

/// A company with a name and employees.
///
/// # Arguments
///
/// * `name` - The company's name.
/// * `employees` - The company's employees.
///
/// # Examples
///
/// ```
///
/// use employee_management_system::{Company, Employee, Person, SalariedWage};
/// use employee_management_system::enums::Role;
///
/// let mut my_company = Company::new(
///     String::from("My Company"),
///     vec![],
/// );
/// my_company.add_employee(Employee::new(
///     Person::new(String::from("Jane"), 25),
///     Role::Developer,
///     SalariedWage::new(500.0),
///     20,
/// ));
/// assert_eq!(my_company.name, "My Company");
/// my_company.all_employees().iter().for_each(|employee| {
///     assert_eq!(employee.person.name, "Jane");
///     assert_eq!(employee.person.age, 25);
///     assert_eq!(employee.role, Role::Developer);
/// });
#[derive(Debug)]
pub struct Company<W: Wage> {
    pub name: String,
    employees: Vec<Employee<W>>,
}
impl<W: Wage> Company<W> {
    /// Creates a new company.
    ///
    /// # Arguments
    ///
    /// * `name` - The company's name.
    /// * `employees` - The company's employees.
    ///
    /// # Examples
    ///
    /// ```
    /// use employee_management_system::{Company, Employee, Person, SalariedWage};
    /// use employee_management_system::enums::Role;
    ///
    /// let mut my_company = Company::new(
    ///     String::from("My Company"),
    ///     vec![Employee::new(
    ///         Person::new(String::from("John"), 30),
    ///         Role::Manager,
    ///         SalariedWage::new(1000.0),
    ///         20,
    ///     )],
    /// );
    ///
    /// assert_eq!(my_company.name, "My Company");
    /// my_company.all_employees().iter().for_each(|employee| {
    ///    assert_eq!(employee.person.name, "John");
    ///    assert_eq!(employee.person.age, 30);
    ///    assert_eq!(employee.role, Role::Manager);
    /// });
    ///```
    pub fn new(name: String, employees: Vec<Employee<W>>) -> Self {
        Self { name, employees }
    }
    /// Adds an employee to the company.
    ///
    /// # Arguments
    ///
    /// * `employee` - The employee to add.
    ///
    /// # Examples
    ///
    /// ```
    /// use employee_management_system::{Company, Employee, Person, HourlyWage};
    /// use employee_management_system::enums::Role;
    ///
    /// let mut my_company = Company::new(
    ///     String::from("My Company"),
    ///     vec![],
    /// );
    /// my_company.add_employee(Employee::new(
    ///     Person::new(String::from("Jane"), 25),
    ///     Role::Developer,
    ///     HourlyWage::new(20.0, 40.0),
    ///     20,
    /// ));
    /// assert_eq!(my_company.name, "My Company");
    /// my_company.all_employees().iter().for_each(|employee| {
    ///     assert_eq!(employee.person.name, "Jane");
    ///     assert_eq!(employee.person.age, 25);
    ///     assert_eq!(employee.role, Role::Developer);
    /// });
    /// ```
    pub fn add_employee(&mut self, employee: Employee<W>) {
        self.employees.push(employee);
    }
    /// Gets all employees.
    ///
    /// # Examples
    ///
    /// ```
    /// use employee_management_system::{Company, Employee, Person, HourlyWage};
    /// use employee_management_system::enums::Role;
    ///
    /// let mut my_company = Company::new(
    ///     String::from("My Company"),
    ///     vec![],
    /// );
    /// my_company.add_employee(Employee::new(
    ///     Person::new(String::from("Jane"), 25),
    ///     Role::Developer,
    ///     HourlyWage::new(20.0, 40.0),
    ///     20,
    /// ));
    /// assert_eq!(my_company.name, "My Company");
    /// my_company.all_employees().iter().for_each(|employee| {
    ///     assert_eq!(employee.person.name, "Jane");
    ///     assert_eq!(employee.person.age, 25);
    ///     assert_eq!(employee.role, Role::Developer);
    /// });
    pub fn all_employees(&self) -> &Vec<Employee<W>> {
        &self.employees
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::employees::enums::Role;
    use crate::employees::structs::{Employee, Person, SalariedWage};

    #[test]
    fn test_company() {
        let my_company = Company::new(
            String::from("My Company"),
            vec![Employee::new(
                Person::new(String::from("John"), 30),
                Role::Manager,
                SalariedWage::new(1000.0),
                20,
            )],
        );

        assert_eq!(my_company.name, "My Company");

        my_company.all_employees().iter().for_each(|employee| {
            assert_eq!(employee.person.name, "John");
            assert_eq!(employee.person.age, 30);
            assert_eq!(employee.role, Role::Manager);
        });
    }
}
