use std::io::Write;

use tempdir::TempDir;

// Empty data set
pub static EMPTY_DATA_SET_FILE_NAME: &'static str = "empty_data_set";
pub static EMPTY_DATA_SET_FILE_BYTES: &'static[u8] = include_bytes!("../../empty_data_set.nc");

// NetCDF-3 (classic version)
pub static NC3_CLASSIC_FILE_NAME: &'static str = "temp_3D_classic.nc";
pub static NC3_CLASSIC_FILE_BYTES: &'static[u8] = include_bytes!("../../temp_3D_classic.nc");

// NetCDF-3 (64-bit offset version)
pub static NC3_64BIT_OFFSET_FILE_NAME: &'static str = "temp_3D_64bit_offset.nc";
pub static NC3_64BIT_OFFSET_FILE_BYTES: &'static[u8] = include_bytes!("../../temp_3D_64bit_offset.nc");

// Scalar variables contaning the default `NC_FILL` values
pub static NC3_FILL_VALUES_FILE_NAME: &'static str = "nc_fill_values.nc";
pub static NC3_FILL_VALUES_FILE_BYTES: &'static[u8] = include_bytes!("../../nc_fill_values.nc");

// Scalar variables
pub static SCALAR_VARIABLES_FILE_NAME: &'static str = "scalar_vars.nc";
pub static SCALAR_VARIABLES_FILE_BYTES: &'static[u8] = include_bytes!("../../scalar_vars.nc");

// Another basic Classic NetCDF-3 file used by documention examples
pub static NC3_BASIC_CLASSIC_FILE_NAME: &'static str = "basic_temp_3D_classic.nc";
pub static NC3_BASIC_CLASSIC_FILE_BYTES: &'static[u8] = include_bytes!("../../basic_temp_3D_classic.nc");

/// Copies bytes to a file located in a temporary directory.
///
/// Do not forget to close the returned temporary directy explicitly to remove it.
pub fn copy_bytes_to_tmp_file(bytes: &[u8], file_name: &str) -> (TempDir, std::path::PathBuf)
{
    // Crete the temporary directory
    let tmp_dir: TempDir = TempDir::new("netcdf3_test_data").unwrap();
    // Crete the temporary file
    let tmp_file_path = std::path::PathBuf::from(tmp_dir.path()).join(file_name);
    let mut tmp_file = std::fs::File::create(tmp_file_path.clone()).unwrap();
    // Copy all bytes
    let _ = tmp_file.write_all(bytes).unwrap();

    return (tmp_dir, tmp_file_path);
}
