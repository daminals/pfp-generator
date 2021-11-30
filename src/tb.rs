use tabled::{Tabled, Table};

#[derive(Tabled)]
struct TwoColTbStruct {
    index: usize,
    name: &'static str,
}

fn print_col_fn(table_array: Vec<TwoColTbStruct>) {
    let table = Table::new(table_array);
    print!("{}", table.to_string());
}

pub fn color_print() {
    let two_col_tb_arr = vec![
        TwoColTbStruct{
            index: 0,
            name: "blue"
        },
        TwoColTbStruct{
            index: 1,
            name: "red"
        },
        TwoColTbStruct{
            index: 2,
            name: "green"
        },
        TwoColTbStruct{
            index: 3,
            name: "white"
        },
        TwoColTbStruct{
            index: 4,
            name: "yellow",
        },
        TwoColTbStruct{
            index: 5,
            name: "orange",
        },
        TwoColTbStruct{
            index: 6,
            name: "purple",
        },
        TwoColTbStruct{
            index: 7,
            name: "pink",
        },
    ];
    print_col_fn(two_col_tb_arr);
}
