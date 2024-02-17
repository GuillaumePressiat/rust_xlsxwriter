// Test case that compares a file generated by rust_xlsxwriter with a file
// created by Excel.
//
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// Copyright 2022-2023, John McNamara, jmcnamara@cpan.org

use crate::common;
use rust_xlsxwriter::{Format, FormatAlign, Workbook, XlsxError};

// Create rust_xlsxwriter file to compare against Excel file.
fn create_new_xlsx_file(filename: &str) -> Result<(), XlsxError> {
    let mut workbook = Workbook::new();

    let worksheet = workbook.add_worksheet();

    let format1 = Format::new()
        .set_rotation(270)
        .set_indent(1)
        .set_align(FormatAlign::Center)
        .set_align(FormatAlign::Top);

    worksheet.set_row_height(0, 75)?;

    worksheet.write_with_format(0, 0, "ABCD", &format1)?;

    workbook.save(filename)?;

    Ok(())
}

#[test]
fn test_format24() {
    let test_runner = common::TestRunner::new()
        .set_name("format24")
        .set_function(create_new_xlsx_file)
        .initialize();

    test_runner.assert_eq();
    test_runner.cleanup();
}
