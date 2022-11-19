use hdf5::File;

pub(crate) fn read_nwb_file(path: &str) -> Result<hdf5::File, hdf5::Error> {
    File::open(path)
}
