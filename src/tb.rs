use tabled::{Tabled, Table, Rotate};

#[derive(Tabled)]
struct TwoColTbStruct {
    index: usize,
    name: &'static str,
}

fn print_col_fn(table_array: Vec<TwoColTbStruct>) {
    let table = Table::new(table_array).with(Rotate::Left);
    print!("{}", table.to_string());
}

pub fn color_print() {
    let two_col_tb_arr = vec![
        TwoColTbStruct{
            index: 0,
            name: "\u{001b}[34m blue \u{001b}[0m"
        },
        TwoColTbStruct{
            index: 1,
            name: "\u{001b}[31m red \u{001b}[0m" // red + color code
        },
        TwoColTbStruct{
            index: 2,
            name: "\u{001b}[32m green\u{001b}[0m"
        },
        TwoColTbStruct{
            index: 3,
            name: "white \u{001b}[0m"
        },
        TwoColTbStruct{
            index: 4,
            name: "\u{001b}[33m yellow \u{001b}[0m",
        },
        TwoColTbStruct{
            index: 5,
            name: "\u{001b}[38;5;208m orange \u{001b}[0m", // lol ansi color code go wild
        },
        TwoColTbStruct{
            index: 6,
            name: "\u{001b}[35m purple \u{001b}[0m",
        },
        TwoColTbStruct{
            index: 7,
            name: "\u{001b}[38;5;199m pink \u{001b}[0m",
        },
    ];
    print_col_fn(two_col_tb_arr);
}
