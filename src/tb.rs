use tabled::{Tabled, Table};

pub struct TwoColTbStruct {
    index: u8,
    name: &'static str,
}

pub fn tb_print() {    
    #[derive(Tabled)]
    struct Language {
        name: &'static str,
        designed_by: &'static str,
        invented_year: usize,
    }
    
    let languages = vec![
        Language{
            name: "C",
            designed_by: "Dennis Ritchie",
            invented_year: 1972
        },
        Language{
            name: "Rust",
            designed_by: "Graydon Hoare",
            invented_year: 2010
        },
        Language{
            name: "Go",
            designed_by: "Rob Pike",
            invented_year: 2009
        },
    ];
    
    let table = Table::new(languages).to_string();
    println!("{}", table);
}

pub fn two_col_tb_fn(table_array: Vec<TwoColTbStruct>) {
    #[derive(Tabled)]
    let table = Table::new(table_array).to_string();
    println!("{}", table);
}

pub fn color_print() {
    let two_col_tb_arr = vec![
        TwoColTbStruct{
            index: 0,
            name: "blue",
        },
        TwoColTbStruct{
            index: 1,
            name: "red",
        },
        TwoColTbStruct{
            index: 2,
            name: "green",
        },
        TwoColTbStruct{
            index: 3,
            name: "white",
        },
        TwoColTbStruct{
            index: 4,
            name: "white",
        },
        TwoColTbStruct{
            index: 5,
            name: "yellow",
        },
        TwoColTbStruct{
            index: 6,
            name: "orange",
        },
        TwoColTbStruct{
            index: 7,
            name: "purple",
        },
        TwoColTbStruct{
            index: 8,
            name: "pink",
        },
    ];
    two_col_tb_fn(two_col_tb_arr);
}
