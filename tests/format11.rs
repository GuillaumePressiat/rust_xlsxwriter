// Test case that compares a file generated by rust_xlsxwriter with a file
// created by Excel.
//
// SPDX-License-Identifier: MIT OR Apache-2.0 Copyright 2022, John McNamara,
// jmcnamara@cpan.org

use rust_xlsxwriter::{Format, Workbook, XlsxAlign, XlsxError};

mod common;

// Test case to test simple formatting.
fn create_new_xlsx_file(filename: &str) -> Result<(), XlsxError> {
    let mut workbook = Workbook::new();

    let worksheet = workbook.add_worksheet();

    let centered = Format::new()
        .set_align(XlsxAlign::Center)
        .set_align(XlsxAlign::VerticalCenter);

    worksheet.write_string(1, 1, "Foo", &centered)?;

    workbook.save(filename)?;

    Ok(())
}

#[test]
fn test_format11() {
    let test_runner = common::TestRunner::new("format11").initialize();

    let result = create_new_xlsx_file(test_runner.output_file());
    assert_result!(result);

    test_runner.assert_eq();
    test_runner.cleanup();
}
