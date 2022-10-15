// Test case that compares a file generated by rust_xlsxwriter with a file
// created by Excel.
//
// SPDX-License-Identifier: MIT OR Apache-2.0 Copyright 2022, John McNamara,
// jmcnamara@cpan.org

use rust_xlsxwriter::{Format, Workbook, XlsxError};

mod common;

// Test case to demonstrate creating a basic file with some string cell data.
fn create_new_xlsx_file(filename: &str) -> Result<(), XlsxError> {
    let mut workbook = Workbook::new(filename);

    let format = Format::new().set_font_strikethrough();

    let worksheet = workbook.add_worksheet();
    worksheet.write_string(0, 0, "Strikeout Text", &format)?;

    workbook.close()?;

    Ok(())
}

#[test]
fn bootstrap21_strikethrough_text() {
    let test_runner = common::TestRunner::new("bootstrap21").initialize();

    _ = create_new_xlsx_file(test_runner.output_file());

    test_runner.assert_eq();
    test_runner.cleanup();
}
