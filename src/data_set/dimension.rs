use crate::InvalidDataSet;

use crate::name_string::is_valid_name;

use std::cell::RefCell;

/// NetCDF-3 dimension
///
/// `Dimension` instances are managed by a [`DataSet`](struct.DataSet.html).
///
/// `DataSet`s allow to create, read, remove and rename `Dimension`s.
///
/// # Examples
///
/// ## Create and read *fixed-size* and *unlimited-size* dimensions
///
/// ```
/// use std::rc::Rc;
/// use netcdf3::{DataSet, Dimension, DimensionType};
///
/// const DIM_NAME_1: &str = "dim_1";
/// const DIM_SIZE_1: usize = 10;
/// const DIM_NAME_2: &str = "dim_2";
/// const DIM_SIZE_2: usize = 20;
///
/// // First create a data set
/// let mut data_set = DataSet::new();
///
/// // Add one *fixed-size* dimensions and set the *unlimited-size* dimension
/// data_set.set_unlimited_dim(DIM_NAME_1, DIM_SIZE_1).unwrap();
/// data_set.add_fixed_dim(DIM_NAME_2, DIM_SIZE_2).unwrap();
///
/// // Read values throught the data set
/// assert_eq!(2,                                   data_set.num_dims());
/// assert_eq!(true,                                data_set.has_unlimited_dim());
/// assert_eq!(true,                                data_set.has_dim(DIM_NAME_1));
/// assert_eq!(Some(DIM_SIZE_1),                    data_set.get_dim_size(DIM_NAME_1));
/// assert_eq!(Some(DimensionType::UnlimitedSize),  data_set.get_dim_type(DIM_NAME_1));
/// assert_eq!(true,                                data_set.has_dim(DIM_NAME_2));
/// assert_eq!(Some(DIM_SIZE_2),                    data_set.get_dim_size(DIM_NAME_2));
/// assert_eq!(Some(DimensionType::FixedSize),      data_set.get_dim_type(DIM_NAME_2));
///
/// // Or through references of the dimensions
/// let dim_1: Rc<Dimension> = data_set.get_dim(DIM_NAME_1).unwrap();
/// assert_eq!(DIM_NAME_1,                      dim_1.name());
/// assert_eq!(DIM_SIZE_1,                      dim_1.size());
/// assert_eq!(true,                            dim_1.is_unlimited());
/// assert_eq!(false,                           dim_1.is_fixed());
/// assert_eq!(DimensionType::UnlimitedSize,    dim_1.dim_type());
///
/// let dim_2: Rc<Dimension> = data_set.get_dim(DIM_NAME_2).unwrap();
/// assert_eq!(DIM_NAME_2,                      dim_2.name());
/// assert_eq!(DIM_SIZE_2,                      dim_2.size());
/// assert_eq!(false,                           dim_2.is_unlimited());
/// assert_eq!(true,                            dim_2.is_fixed());
/// assert_eq!(DimensionType::FixedSize,        dim_2.dim_type());
///
/// ```
///
/// ## Rename a dimension
///
/// ```
/// use netcdf3::{DataSet, DimensionType};
///
/// const DIM_NAME_1: &str = "dim_1";
/// const DIM_NAME_2: &str = "dim_2";
/// const DIM_SIZE: usize = 10;
///
/// // First create a data set
/// let mut data_set = DataSet::new();
///
/// // Add a *fixed-size* dimension
/// data_set.add_fixed_dim(DIM_NAME_1, DIM_SIZE).unwrap();
///
/// assert_eq!(1,                               data_set.num_dims());
/// assert_eq!(false,                           data_set.has_unlimited_dim());
/// assert_eq!(true,                            data_set.has_dim(DIM_NAME_1));
/// assert_eq!(Some(DIM_SIZE),                  data_set.get_dim_size(DIM_NAME_1));
/// assert_eq!(Some(DimensionType::FixedSize),  data_set.get_dim_type(DIM_NAME_1));
/// assert_eq!(false,                           data_set.has_dim(DIM_NAME_2));
/// assert_eq!(None,                            data_set.get_dim_size(DIM_NAME_2));
/// assert_eq!(None,                            data_set.get_dim_type(DIM_NAME_2));
///
/// // Rename the *fixed-size* dimension
/// data_set.rename_dim(DIM_NAME_1, DIM_NAME_2).unwrap();
///
/// assert_eq!(1,                               data_set.num_dims());
/// assert_eq!(false,                           data_set.has_unlimited_dim());
/// assert_eq!(false,                           data_set.has_dim(DIM_NAME_1));
/// assert_eq!(None,                            data_set.get_dim_size(DIM_NAME_1));
/// assert_eq!(None,                            data_set.get_dim_type(DIM_NAME_1));
/// assert_eq!(true,                            data_set.has_dim(DIM_NAME_2));
/// assert_eq!(Some(DIM_SIZE),                  data_set.get_dim_size(DIM_NAME_2));
/// assert_eq!(Some(DimensionType::FixedSize),  data_set.get_dim_type(DIM_NAME_2));
/// ```
///
/// ## Remove a dimension
///
/// ```
/// use std::rc::Rc;
/// use netcdf3::{DataSet, Dimension, DimensionType};
///
/// const DIM_NAME: &str = "dim_1";
/// const DIM_SIZE: usize = 10;
///
/// // First create a data set
/// let mut data_set = DataSet::new();
///
/// // Set the *unlimited-size* dimension
/// data_set.set_unlimited_dim(DIM_NAME, DIM_SIZE).unwrap();
///
/// assert_eq!(1,                                   data_set.num_dims());
/// assert_eq!(true,                                data_set.has_unlimited_dim());
/// assert_eq!(true,                                data_set.has_dim(DIM_NAME));
/// assert_eq!(Some(DIM_SIZE),                      data_set.get_dim_size(DIM_NAME));
/// assert_eq!(Some(DimensionType::UnlimitedSize),  data_set.get_dim_type(DIM_NAME));
///
/// // Remove the *unlimited-size* dimension
/// let _removed_dim: Rc<Dimension> = data_set.remove_dim(DIM_NAME).unwrap();
///
/// assert_eq!(0,       data_set.num_dims());
/// assert_eq!(false,   data_set.has_unlimited_dim());
/// assert_eq!(false,   data_set.has_dim(DIM_NAME));
/// assert_eq!(None,    data_set.get_dim_size(DIM_NAME));
/// assert_eq!(None,    data_set.get_dim_type(DIM_NAME));
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Dimension {
    pub(in crate::data_set) name: RefCell<String>,
    pub(in crate::data_set) size: DimensionSize,
}

/// Internal representation of the size of a dimension.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(in crate::data_set) enum DimensionSize {
    /// *Unlimited-size* dimension, the unlimited size can be modifed by the NetCDF-3 dataset.
    Unlimited(RefCell<usize>),
    /// *Fixed-size* dimension
    Fixed(usize),
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[repr(u8)]
/// Type of a dimension, *fixed* or *unlimited* size
pub enum DimensionType {
    UnlimitedSize = 0,
    FixedSize = 1,
}

impl DimensionSize {
    /// Create a new *unlimited* or *fixed* size.
    pub(in crate::data_set) fn new(size: usize, r#type: DimensionType) -> DimensionSize {
        return match r#type {
            DimensionType::FixedSize => DimensionSize::Fixed(size),
            DimensionType::UnlimitedSize => DimensionSize::Unlimited(RefCell::new(size)),
        };
    }

    #[inline]
    /// Return the size of the dimension.
    pub(in crate::data_set) fn size(&self) -> usize {
        return match self {
            DimensionSize::Unlimited(size) => size.borrow().clone(),
            DimensionSize::Fixed(size) => size.clone(),
        };
    }

    #[inline]
    /// Return the size of the dimension.
    pub(in crate::data_set) fn r#type(&self) -> DimensionType {
        return match self {
            DimensionSize::Unlimited(_) => DimensionType::UnlimitedSize,
            DimensionSize::Fixed(_) => DimensionType::FixedSize,
        };
    }
}

impl Dimension {

    /// Creates a new *fixed size* NetCDF-3 dimension.
    pub(in crate::data_set) fn new_fixed_size(name: &str, size: usize) -> Result<Dimension, InvalidDataSet> {
        Dimension::check_dim_name(name)?;
        return Ok(Dimension {
            name: RefCell::new(name.to_string()),
            size: DimensionSize::new(size, DimensionType::FixedSize),
        });
    }

    /// Creates a new *unlimited size* NetCDF-3 dimension.
    pub(in crate::data_set) fn new_unlimited_size(name: &str, size: usize) -> Result<Dimension, InvalidDataSet> {
        Dimension::check_dim_name(name)?;
        return Ok(Dimension {
            name: RefCell::new(name.to_string()),
            size: DimensionSize::new(size, DimensionType::UnlimitedSize),
        });
    }

    /// Returns the name of the NetCDF-3 dimension.
    pub fn name(&self) -> String {
        return self.name.borrow().clone();
    }

    /// Returns the size of the NetCDF-3 dimension.
    pub fn size(&self) -> usize {
        return self.size.size();
    }

    /// Returns the dimension type (*fixed size* ou *unlimited size*) of the NetCDF-3 dimension.
    pub fn dim_type(&self) -> DimensionType {
        return self.size.r#type();
    }

    /// Returns `true` if the dimension is a *unlimited size* dimension, otherwise return `false`.
    pub fn is_unlimited(&self) -> bool {
        return self.dim_type() == DimensionType::UnlimitedSize;
    }

    /// Returns `true` if the dimension is a *fixed size* dimension, otherwise return `false`.
    pub fn is_fixed(&self) -> bool {
        return self.dim_type() == DimensionType::FixedSize;
    }

    pub(in crate::data_set) fn check_dim_name(dim_name: &str) -> Result<(), InvalidDataSet> {
        return match is_valid_name(dim_name) {
            true => Ok(()),
            false => Err(InvalidDataSet::DimensionNameNotValid(dim_name.to_string())),
        };
    }
}


#[cfg(test)]
mod tests {

    use std::rc::Rc;
    use crate::{Dimension, DimensionType};

    #[test]
    fn test_dim_new_fixed_size() {
        const DIM_NAME: &str = "dim_1";
        const DIM_SIZE: usize = 10;

        let dim = Dimension::new_fixed_size(DIM_NAME, DIM_SIZE).unwrap();

        assert_eq!(DIM_NAME, dim.name());
        assert_eq!(DIM_SIZE, dim.size());
        assert_eq!(DimensionType::FixedSize, dim.dim_type());
        assert_eq!(true, dim.is_fixed());
        assert_eq!(false, dim.is_unlimited())
    }

    #[test]
    fn test_dim_new_unlimited_size() {
        const DIM_NAME: &str = "dim_1";
        const DIM_SIZE: usize = 10;

        let dim = Dimension::new_unlimited_size(DIM_NAME, DIM_SIZE).unwrap();

        assert_eq!(DIM_NAME, dim.name());
        assert_eq!(DIM_SIZE, dim.size());
        assert_eq!(DimensionType::UnlimitedSize, dim.dim_type());
        assert_eq!(false, dim.is_fixed());
        assert_eq!(true, dim.is_unlimited());
    }

    #[test]
    fn test_dim_equality() {

        // test equality between 2 fixed-size dimension
        {
            let dim_a: Dimension = Dimension::new_fixed_size("name_1", 180).unwrap();
            let dim_b: Dimension = Dimension::new_fixed_size("name_1", 180).unwrap();
            assert_eq!(dim_a, dim_b);
        }

        // test equality between 2 fixed-size dimension with different sizes
        {
            let dim_a: Dimension = Dimension::new_fixed_size("name_1", 90).unwrap();
            let dim_b: Dimension = Dimension::new_fixed_size("name_1", 180).unwrap();
            assert_ne!(dim_a, dim_b);
        }

        // test equality between 2 fixed-size dimension with different names
        {
            let dim_a: Dimension = Dimension::new_fixed_size("name_1", 180).unwrap();
            let dim_b: Dimension = Dimension::new_fixed_size("name_2", 180).unwrap();
            assert_ne!(dim_a, dim_b);
        }

        // test equality between 2 unlimited-size dimension
        {
            let dim_a: Dimension = Dimension::new_unlimited_size("name_1", 180).unwrap();
            let dim_b: Dimension = Dimension::new_unlimited_size("name_1", 180).unwrap();
            assert_eq!(dim_a, dim_b);
        }

        // test equality between 2 unlimited-size dimension with different sizes
        {
            let dim_a: Dimension = Dimension::new_unlimited_size("name_1", 90).unwrap();
            let dim_b: Dimension = Dimension::new_unlimited_size("name_1", 180).unwrap();
            assert_ne!(dim_a, dim_b);
        }

        // test equality between 2 unlimited-size dimension with different names
        {
            let dim_a: Dimension = Dimension::new_unlimited_size("name_1", 180).unwrap();
            let dim_b: Dimension = Dimension::new_unlimited_size("name_2", 180).unwrap();
            assert_ne!(dim_a, dim_b);
        }

        // test equality between 1 unlimited-size dimension and 1 fixed-size dimension
        {
            let dim_a: Dimension = Dimension::new_fixed_size("name_1", 180).unwrap();
            let dim_b: Dimension = Dimension::new_unlimited_size("name_1", 180).unwrap();
            assert_ne!(dim_a, dim_b);
        }
    }

    #[test]
    fn test_rc_dim_equality() {
        // test equality between 2 fixed-size dimension
        {
            let dim_a: Rc<Dimension> = Rc::new(Dimension::new_fixed_size("name_1", 180).unwrap());
            let dim_b: Rc<Dimension> = Rc::new(Dimension::new_fixed_size("name_1", 180).unwrap());

            assert_eq!(dim_a, dim_b);
            assert!(!Rc::ptr_eq(&dim_a, &dim_b));

            let dim_c: Rc<Dimension> = Rc::clone(&dim_a);
            assert_eq!(dim_a, dim_c);
            assert_eq!(dim_b, dim_c);
            assert!(Rc::ptr_eq(&dim_a, &dim_c));
            assert!(!Rc::ptr_eq(&dim_b, &dim_c));
        }
    }
}