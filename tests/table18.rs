// Test case that compares a file generated by rust_xlsxwriter with a file
// created by Excel.
//
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// Copyright 2022-2023, John McNamara, jmcnamara@cpan.org

use rust_xlsxwriter::{Format, Table, TableColumn, Workbook, XlsxError};

#[macro_use]
extern crate lazy_static;

mod common;

// Test to demonstrate charts.
fn create_new_xlsx_file(filename: &str) -> Result<(), XlsxError> {
    let mut workbook = Workbook::new();

    let worksheet = workbook.add_worksheet();
    let text_wrap = Format::new().set_text_wrap();

    for col_num in 2..=5u16 {
        worksheet.set_column_width(col_num, 10.288)?;
    }
    worksheet.set_row_height(2, 39)?;
    worksheet.write(15, 0, "hello")?;

    let mut table = Table::new();
    let columns = vec![
        TableColumn::default(),
        TableColumn::default(),
        TableColumn::default(),
        TableColumn::new().set_header("Column\n4"),
    ];

    table.set_columns(&columns);

    worksheet.add_table(2, 2, 12, 5, &table)?;

    // Overwrite cell until we have header formatting.
    worksheet.write_with_format(2, 5, "Column\n4", &text_wrap)?;

    workbook.save(filename)?;

    Ok(())
}

#[test]
fn test_table18() {
    let test_runner = common::TestRunner::new()
        .set_name("table18")
        .set_function(create_new_xlsx_file)
        .initialize();

    test_runner.assert_eq();
    test_runner.cleanup();
}