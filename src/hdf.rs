use hdf5::File;

pub(crate) fn read_nwb_file(path: &str) -> Option<hdf5::File> {
    let handler = match File::open(path) {
        Err(_) => None,
        Ok(hdf_file) => Some(hdf_file),
    };
    return handler;
}

pub fn get_subgroups(group: &hdf5::Group) -> Vec<hdf5::Group> {
    return group.groups().unwrap();
}

pub fn print_subgroups(group: &hdf5::Group) {
    // let mut all_subgroups: Vec<&hdf5::Group>;
    // all_subgroups.append(group);
    let groups = get_subgroups(&group);
    // all_subgroups.append(groups);
    for subgroup in groups {
        println!("In get_subgroups for {}: {}", group.name(), subgroup.name());
        get_subgroups(&subgroup);
        // all_subgroups.append(subgroup);
    }
    // return groups;
}