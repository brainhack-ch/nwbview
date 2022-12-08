use hdf5::File;

pub struct GroupTree {
    pub handler: hdf5::Group,
    pub groups: Vec<GroupTree>,
    pub datasets: Vec<String>,
}

pub struct FileTree {
    pub file: hdf5::File,
    pub tree: GroupTree,
    pub is_opened: bool,
}

pub(crate) fn build_tree(group: hdf5::Group) -> GroupTree {
    let groups: Vec<hdf5::Group> = group.groups().unwrap();
    let datasets: Vec<String> = group
        .datasets()
        .unwrap()
        .into_iter()
        .map(|x| x.name())
        .collect();
    let mut sub_trees: Vec<GroupTree> = Vec::new();
    for sub_group in groups {
        sub_trees.push(build_tree(sub_group));
    }
    GroupTree {
        handler: group,
        groups: sub_trees,
        datasets,
    }
}

pub(crate) fn read_nwb_file(path: &str) -> Option<FileTree> {
    let file = match File::open(path) {
        Err(_) => None,
        Ok(hdf_file) => Some(hdf_file),
    };

    match file {
        None => None,
        Some(x) => match x.as_group() {
            Err(_) => None,
            Ok(y) => Some(FileTree {
                file: x,
                tree: build_tree(y),
                is_opened: true,
            }),
        },
    }
}
