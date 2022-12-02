use hdf5::File;

pub(crate) fn read_nwb_file(path: &str) -> Option<hdf5::File> {
    match File::open(path) {
        Err(_) => None,
        Ok(hdf_file) => Some(hdf_file),
    }
}

pub fn get_subgroups(group: &hdf5::Group) -> Vec<hdf5::Group> {
    group.groups().unwrap()
}

pub fn get_datasets(group: &hdf5::Group) -> Vec<hdf5::Dataset> {
    group.datasets().unwrap()
}

// pub fn print_subgroups(group: &hdf5::Group) {
//     let groups = get_subgroups(group);
//     for subgroup in groups {
//         println!("In get_subgroups for {}: {}", group.name(), subgroup.name());
//     }
// }
