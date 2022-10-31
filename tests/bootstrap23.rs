// Test case that compares a file generated by rust_xlsxwriter with a file
// created by Excel.
//
// SPDX-License-Identifier: MIT OR Apache-2.0 Copyright 2022, John McNamara,
// jmcnamara@cpan.org

use rust_xlsxwriter::{Format, Workbook, XlsxError};

mod common;

// Test case to demonstrate creating a basic file with user defined row.
fn create_new_xlsx_file_1(filename: &str) -> Result<(), XlsxError> {
    let mut workbook = Workbook::new();
    let worksheet = workbook.add_worksheet();
    let format1 = Format::new().set_bold();
    let format2 = Format::new().set_italic();

    worksheet.write_string_only(0, 0, "Rust")?;
    worksheet.write_string_only(2, 0, "Rust")?;
    worksheet.write_string(6, 0, "Rust", &format1)?;

    worksheet.set_row_height(0, 24)?;
    worksheet.set_row_height(4, 39)?;
    worksheet.set_row_format(8, &format2)?;

    worksheet.set_row_height(10, 23.25)?;
    worksheet.set_row_format(10, &format1)?;

    workbook.save(filename)?;

    Ok(())
}

// Test case to demonstrate creating a basic file with user defined row.
fn create_new_xlsx_file_2(filename: &str) -> Result<(), XlsxError> {
    let mut workbook = Workbook::new();
    let worksheet = workbook.add_worksheet();
    let format1 = Format::new().set_bold();
    let format2 = Format::new().set_italic();

    worksheet.write_string_only(0, 0, "Rust")?;
    worksheet.write_string_only(2, 0, "Rust")?;
    worksheet.write_string(6, 0, "Rust", &format1)?;

    worksheet.set_row_height_pixels(0, 32)?;
    worksheet.set_row_height_pixels(4, 52)?;
    worksheet.set_row_format(8, &format2)?;

    worksheet.set_row_height_pixels(10, 31)?;
    worksheet.set_row_format(10, &format1)?;

    workbook.save(filename)?;

    Ok(())
}

#[test]
fn bootstrap23_set_row() {
    let test_runner = common::TestRunner::new("bootstrap23")
        .unique("1")
        .initialize();

    let result = create_new_xlsx_file_1(test_runner.output_file());
    assert_result!(result);

    test_runner.assert_eq();
    test_runner.cleanup();
}

#[test]
fn bootstrap23_set_row_pixels() {
    let test_runner = common::TestRunner::new("bootstrap23")
        .unique("2")
        .initialize();

    let result = create_new_xlsx_file_2(test_runner.output_file());
    assert_result!(result);

    test_runner.assert_eq();
    test_runner.cleanup();
}
