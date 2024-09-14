// Test case that compares a file generated by rust_xlsxwriter with a file
// created by Excel.
//
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// Copyright 2022-2024, John McNamara, jmcnamara@cpan.org

use crate::common;
use rust_xlsxwriter::{Workbook, XlsxError};

// Create rust_xlsxwriter file to compare against Excel file.
fn create_new_xlsx_file(filename: &str) -> Result<(), XlsxError> {
    let mut workbook = Workbook::new();

    // Add the VBA macro file.
    workbook.add_vba_project("tests/input/macros/vbaProject04.bin")?;

    let worksheet = workbook.add_worksheet().set_name("Foo")?;

    worksheet.write(0, 0, 123)?;

    workbook.save(filename)?;

    Ok(())
}

#[test]
fn test_macro01() {
    let test_runner = common::TestRunner::new()
        .set_name("macro03")
        .has_macros()
        .set_function(create_new_xlsx_file)
        .initialize();

    test_runner.assert_eq();
    test_runner.cleanup();
}
