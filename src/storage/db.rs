struct Table {
    table_info_file: [u8; 5],
    table_info: TableInfo,
}

impl Table {
    fn new(
        table_info_file: [u8; 5],
        file_path: [u8; 5],
        size_of_element: u16,
        number_of_element: u16,
    ) -> Self {
        TableInfo
    }
}

struct TableInfo {
    file_path: [u8; 5],
    size_of_element: u16,
    number_of_element: u16,
}
