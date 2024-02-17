// Test case that compares a file generated by rust_xlsxwriter with a file
// created by Excel.
//
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// Copyright 2022-2024, John McNamara, jmcnamara@cpan.org

use crate::common;
use rust_xlsxwriter::{Table, TableColumn, TableFunction, Workbook, XlsxError};

// Create rust_xlsxwriter file to compare against Excel file.
fn create_new_xlsx_file_1(filename: &str) -> Result<(), XlsxError> {
    let mut workbook = Workbook::new();

    let worksheet = workbook.add_worksheet();

    for col_num in 1..=5u16 {
        worksheet.set_column_width(col_num, 10.288)?;
    }

    worksheet.write(0, 0, "Column1")?;
    worksheet.write(0, 1, "Total")?;
    worksheet.write(0, 2, "Column'")?;
    worksheet.write(0, 3, "Column#")?;
    worksheet.write(0, 4, "Column[")?;
    worksheet.write(0, 5, "Column]")?;

    let columns = vec![
        TableColumn::new()
            .set_header("Column1")
            .set_total_label("Total"),
        TableColumn::new()
            .set_header("Column'")
            .set_total_function(TableFunction::Sum),
        TableColumn::new()
            .set_header("Column#")
            .set_total_function(TableFunction::Sum),
        TableColumn::new()
            .set_header("Column[")
            .set_total_function(TableFunction::Sum),
        TableColumn::new()
            .set_header("Column]")
            .set_total_function(TableFunction::Sum),
    ];

    let table = Table::new().set_columns(&columns).set_total_row(true);

    worksheet.add_table(2, 1, 8, 5, &table)?;

    workbook.save(filename)?;

    Ok(())
}

// Write a table that takes the header name from the worksheet cell data.
fn create_new_xlsx_file_2(filename: &str) -> Result<(), XlsxError> {
    let mut workbook = Workbook::new();

    let worksheet = workbook.add_worksheet();

    for col_num in 1..=5u16 {
        worksheet.set_column_width(col_num, 10.288)?;
    }

    worksheet.write(0, 0, "Column1")?;
    worksheet.write(0, 1, "Total")?;
    worksheet.write(0, 2, "Column'")?;
    worksheet.write(0, 3, "Column#")?;
    worksheet.write(0, 4, "Column[")?;
    worksheet.write(0, 5, "Column]")?;

    // Write the header string, the table should read these and add them.
    worksheet.write(2, 2, "Column'")?;
    worksheet.write(2, 3, "Column#")?;
    worksheet.write(2, 4, "Column[")?;
    worksheet.write(2, 5, "Column]")?;

    let columns = vec![
        TableColumn::new().set_total_label("Total"),
        TableColumn::new().set_total_function(TableFunction::Sum),
        TableColumn::new().set_total_function(TableFunction::Sum),
        TableColumn::new().set_total_function(TableFunction::Sum),
        TableColumn::new().set_total_function(TableFunction::Sum),
    ];

    let table = Table::new().set_columns(&columns).set_total_row(true);

    worksheet.add_table(2, 1, 8, 5, &table)?;

    workbook.save(filename)?;

    Ok(())
}

#[test]
fn test_table23_1() {
    let test_runner = common::TestRunner::new()
        .set_name("table23")
        .set_function(create_new_xlsx_file_1)
        .ignore_calc_chain()
        .unique("1")
        .initialize();

    test_runner.assert_eq();
    test_runner.cleanup();
}

#[test]
fn test_table23_2() {
    let test_runner = common::TestRunner::new()
        .set_name("table23")
        .set_function(create_new_xlsx_file_2)
        .ignore_calc_chain()
        .unique("2")
        .initialize();

    test_runner.assert_eq();
    test_runner.cleanup();
}
