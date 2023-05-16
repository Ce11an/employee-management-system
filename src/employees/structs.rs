use crate::employees::enums::Role;
use crate::employees::errors::VacationDaysShortageError;

/// A person with a name and age.
///
/// # Arguments
///
/// * `name` - The person's name.
/// * `age` - The person's age.
///
/// # Examples
///
/// ```
/// use employee_management_system::Person;
///
/// let person = Person::new(String::from("John"), 30);
/// assert_eq!(person.name, "John");
/// assert_eq!(person.age, 30);
/// ```
#[derive(Debug)]
pub struct Person {
    pub name: String,
    pub age: u8,
}
impl Person {
    /// Creates a new person.
    ///
    /// # Arguments
    ///
    /// * `name` - The person's name.
    /// * `age` - The person's age.
    ///
    /// # Examples
    ///
    /// ```
    /// use employee_management_system::Person;
    ///
    /// let person = Person::new(String::from("John"), 30);
    /// assert_eq!(person.name, "John");
    /// assert_eq!(person.age, 30);
    /// ```
    pub fn new(name: String, age: u8) -> Self {
        Self { name, age }
    }
}

/// This trait is implemented by all employees.
pub trait Wage {
    fn calculate_pay(&self) -> f32;
}

/// A salaried wage.
///
/// # Arguments
///
/// * `monthly_salary` - The monthly salary.
///
/// # Examples
///
/// ```
/// use employee_management_system::SalariedWage;
///
/// let salaried_wage = SalariedWage::new(1000.0);
/// assert_eq!(salaried_wage.monthly_salary, 1000.0);
/// ```
#[derive(Debug)]
pub struct SalariedWage {
    pub monthly_salary: f32,
}
impl SalariedWage {
    /// Creates a new salaried wage.
    ///
    /// # Arguments
    ///
    /// * `monthly_salary` - The monthly salary.
    ///
    /// # Examples
    ///
    /// ```
    /// use employee_management_system::SalariedWage;
    ///
    /// let salaried_wage = SalariedWage::new(1000.0);
    /// assert_eq!(salaried_wage.monthly_salary, 1000.0);
    /// ```
    pub fn new(monthly_salary: f32) -> Self {
        Self { monthly_salary }
    }
}
impl Wage for SalariedWage {
    /// Calculates the pay for a salaried wage.
    ///
    /// # Examples
    ///
    /// ```
    /// use employee_management_system::{SalariedWage, Wage};
    ///
    /// let salaried_wage = SalariedWage::new(1000.0);
    /// assert_eq!(salaried_wage.calculate_pay(), 1000.0);
    /// ```
    fn calculate_pay(&self) -> f32 {
        self.monthly_salary
    }
}
/// A hourly wage.
///
/// # Arguments
///
/// * `hourly_rate` - The hourly rate.
/// * `hours_worked` - The hours worked.
///
/// # Examples
///
/// ```
/// use employee_management_system::HourlyWage;
///
/// let hourly_wage = HourlyWage::new(10.0, 40.0);
/// assert_eq!(hourly_wage.hourly_rate, 10.0);
/// assert_eq!(hourly_wage.hours_worked, 40.0);
/// ```
#[derive(Debug)]
pub struct HourlyWage {
    pub hourly_rate: f32,
    pub hours_worked: f32,
}
impl HourlyWage {
    /// Creates a new hourly wage.
    ///
    /// # Arguments
    ///
    /// * `hourly_rate` - The hourly rate.
    /// * `hours_worked` - The hours worked.
    ///
    /// # Examples
    ///
    /// ```
    /// use employee_management_system::HourlyWage;
    ///
    /// let hourly_wage = HourlyWage::new(10.0, 40.0);
    /// assert_eq!(hourly_wage.hourly_rate, 10.0);
    /// assert_eq!(hourly_wage.hours_worked, 40.0);
    /// ```
    pub fn new(hourly_rate: f32, hours_worked: f32) -> Self {
        HourlyWage {
            hourly_rate,
            hours_worked,
        }
    }
}
impl Wage for HourlyWage {
    /// Calculates the pay for a hourly wage.
    ///
    /// # Examples
    ///
    /// ```
    ///
    /// use employee_management_system::{HourlyWage, Wage};
    ///
    /// let hourly_wage = HourlyWage::new(10.0, 40.0);
    /// assert_eq!(hourly_wage.calculate_pay(), 400.0);
    /// ```
    fn calculate_pay(&self) -> f32 {
        self.hourly_rate * self.hours_worked
    }
}
/// An employee.
///
/// # Arguments
///
/// * `person` - The person.
/// * `role` - The role.
/// * `wage` - The wage.
/// * `vacation_days` - The vacation days.
///
/// # Examples
///
/// ```
/// use employee_management_system::{Employee, Person, SalariedWage};
/// use employee_management_system::enums::Role;
///
/// let mut  employee = Employee::new(
///     Person::new(String::from("John"), 30),
///     Role::Manager,
///     SalariedWage::new(1000.0),
///     20,
/// );
/// assert_eq!(employee.person.name, "John");
/// assert_eq!(employee.person.age, 30);
/// assert_eq!(employee.role, Role::Manager);
/// assert_eq!(employee.wage.monthly_salary, 1000.0);
/// assert_eq!(employee.vacation_days, 20);
/// ```
#[derive(Debug)]
pub struct Employee<W: Wage> {
    pub person: Person,
    pub role: Role,
    pub wage: W,
    pub vacation_days: u8,
}
impl<W: Wage> Employee<W> {
    const FIXED_VACATION_DAYS_PAYOUT: u8 = 5;
    /// Creates a new employee.
    ///
    /// # Arguments
    ///
    /// * `person` - The person.
    /// * `role` - The role.
    /// * `wage` - The wage.
    /// * `vacation_days` - The vacation days.
    ///
    /// # Examples
    ///
    /// ```
    /// use employee_management_system::{Employee, Person, SalariedWage};
    /// use employee_management_system::enums::Role;
    ///
    /// let mut  employee = Employee::new(
    ///     Person::new(String::from("John"), 30),
    ///     Role::Manager,
    ///     SalariedWage::new(1000.0),
    ///     20,
    /// );
    /// assert_eq!(employee.person.name, "John");
    /// assert_eq!(employee.person.age, 30);
    /// assert_eq!(employee.role, Role::Manager);
    /// assert_eq!(employee.wage.monthly_salary, 1000.0);
    /// assert_eq!(employee.vacation_days, 20);
    /// ```
    pub fn new(person: Person, role: Role, wage: W, vacation_days: u8) -> Self {
        Self {
            person,
            role,
            wage,
            vacation_days,
        }
    }
    /// Takes vacation days.
    ///
    /// # Arguments
    ///
    /// * `days` - The days to take.    
    ///
    /// # Errors
    ///
    /// Returns a `VacationDaysShortageError` if there are not enough vacation days.
    ///
    /// # Examples
    ///
    /// ```
    /// use employee_management_system::{Employee, Person, SalariedWage};
    /// use employee_management_system::enums::Role;
    ///
    /// let mut  employee = Employee::new(
    ///     Person::new(String::from("John"), 30),
    ///     Role::Manager,
    ///     SalariedWage::new(1000.0),
    ///     20,
    /// );
    /// assert_eq!(employee.take_vacation(5), Ok(()));
    /// assert_eq!(employee.vacation_days, 15);
    /// ```
    pub fn pay(&mut self) {
        println!(
            "Paying {} for {}.",
            self.wage.calculate_pay(),
            self.person.name
        );
    }

    pub fn take_vacation(&mut self, days: u8) -> Result<(), VacationDaysShortageError> {
        if self.vacation_days < days {
            return Err(VacationDaysShortageError {
                requested_days: days,
                remaining_days: self.vacation_days,
                message: String::from("Not enough vacation days are available."),
            });
        }
        self.subtract_vacation_days(days);
        println!("Taking a vacation!. Holidays left: {}", self.vacation_days);
        Ok(())
    }
    /// Pays out vacation days.
    ///
    /// # Errors
    ///
    /// Returns a `VacationDaysShortageError` if the employee's vacation days are less than 5.
    ///
    /// # Examples
    ///
    /// ```
    ///
    /// use employee_management_system::{Employee, Person, SalariedWage};
    /// use employee_management_system::enums::Role;
    ///
    /// let mut  employee = Employee::new(
    ///     Person::new(String::from("John"), 30),
    ///     Role::Manager,
    ///     SalariedWage::new(1000.0),
    ///     20,
    /// );
    /// assert_eq!(employee.payout_vacation(), Ok(()));
    /// assert_eq!(employee.vacation_days, 15);
    /// ```
    pub fn payout_vacation(&mut self) -> Result<(), VacationDaysShortageError> {
        if self.vacation_days < Self::FIXED_VACATION_DAYS_PAYOUT {
            return Err(VacationDaysShortageError {
                requested_days: Self::FIXED_VACATION_DAYS_PAYOUT,
                remaining_days: self.vacation_days,
                message: String::from("Not enough vacation days are available."),
            });
        }
        self.subtract_vacation_days(Self::FIXED_VACATION_DAYS_PAYOUT);
        println!(
            "Paying out a holiday. Holidays left: {}",
            self.vacation_days
        );
        Ok(())
    }
    fn subtract_vacation_days(&mut self, days: u8) {
        self.vacation_days -= days;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn employee_new() {
        let employee = Employee::new(
            Person::new(String::from("John"), 30),
            Role::Manager,
            SalariedWage::new(1000.0),
            20,
        );
        assert_eq!(employee.person.name, "John");
        assert_eq!(employee.person.age, 30);
        assert_eq!(employee.role, Role::Manager);
        assert_eq!(employee.wage.monthly_salary, 1000.0);
        assert_eq!(employee.vacation_days, 20);
    }
    #[test]
    fn employee_take_vacation() {
        let mut employee = Employee::new(
            Person::new(String::from("John"), 30),
            Role::Manager,
            SalariedWage::new(1000.0),
            20,
        );
        assert_eq!(employee.take_vacation(5), Ok(()));
        assert_eq!(employee.vacation_days, 15);
    }
    #[test]
    fn employee_take_vacation_overdraft() {
        let mut employee = Employee::new(
            Person::new(String::from("John"), 30),
            Role::Manager,
            SalariedWage::new(1000.0),
            20,
        );
        assert_eq!(
            employee.take_vacation(25),
            Err(VacationDaysShortageError {
                requested_days: 25,
                remaining_days: 20,
                message: String::from("Not enough vacation days are available."),
            })
        );
        assert_eq!(employee.vacation_days, 20);
    }
    #[test]
    fn employee_payout_vacation() {
        let mut employee = Employee::new(
            Person::new(String::from("John"), 30),
            Role::Manager,
            SalariedWage::new(1000.0),
            20,
        );
        assert_eq!(employee.payout_vacation(), Ok(()));
        assert_eq!(employee.vacation_days, 15);
    }
    #[test]
    fn employee_payout_vacation_overdraft() {
        let mut employee = Employee::new(
            Person::new(String::from("John"), 30),
            Role::Manager,
            SalariedWage::new(1000.0),
            2,
        );
        assert_eq!(
            employee.payout_vacation(),
            Err(VacationDaysShortageError {
                requested_days: 5,
                remaining_days: 2,
                message: String::from("Not enough vacation days are available."),
            })
        );
        assert_eq!(employee.vacation_days, 2);
    }
}
