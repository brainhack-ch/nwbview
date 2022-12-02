use hdf5::File;

#[cfg(test)]
#[path = "../src/hdf.rs"]
mod hdf;

#[test]
fn read_valid_file() {
    assert_eq!(
        hdf::read_nwb_file("data/sub-anm266951_ses-20141201_behavior+icephys+ogen.nwb").is_none(),
        false
    );
}

#[test]
fn read_invalid_file() {
    assert_eq!(hdf::read_nwb_file("data/UNKNOWN.nwb").is_none(), true);
}

#[test]
fn get_subgroups_from_file() {
    let input_file =
        hdf::read_nwb_file("data/sub-anm266951_ses-20141201_behavior+icephys+ogen.nwb");
    let groups = hdf::get_subgroups(&input_file.unwrap());
    let expected_groups: Vec<String> = vec![
        "/acquisition".to_string(),
        "/analysis".to_string(),
        "/general".to_string(),
        "/intervals".to_string(),
        "/processing".to_string(),
        "/specifications".to_string(),
        "/stimulus".to_string(),
    ];
    for group in groups.iter().zip(expected_groups.iter()) {
        assert_eq!(group.0.name(), *group.1);
    }
}
