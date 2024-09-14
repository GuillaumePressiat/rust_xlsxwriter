// Test case that compares a file generated by rust_xlsxwriter with a file
// created by Excel.
//
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// Copyright 2022-2024, John McNamara, jmcnamara@cpan.org

use crate::common;
use rust_xlsxwriter::{Format, FormatBorder, Workbook, XlsxError};

// Create rust_xlsxwriter file to compare against Excel file.
fn create_new_xlsx_file(filename: &str) -> Result<(), XlsxError> {
    let mut workbook = Workbook::new();

    let worksheet = workbook.add_worksheet();
    let cell_format = Format::new().set_background_color("#FFFF00");

    let border_format = Format::new()
        .set_border(FormatBorder::Thin)
        .set_border_color("#FF0000");

    // Add the data, unformatted.
    worksheet.write(2, 1, 123)?;

    // Add the formatting with border.
    worksheet.set_range_format_with_border(2, 1, 2, 1, &cell_format, &border_format)?;

    workbook.save(filename)?;

    Ok(())
}

#[test]
fn test_overlay03() {
    let test_runner = common::TestRunner::new()
        .set_name("overlay03")
        .set_function(create_new_xlsx_file)
        .initialize();

    test_runner.assert_eq();
    test_runner.cleanup();
}
