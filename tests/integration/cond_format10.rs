// Test case that compares a file generated by rust_xlsxwriter with a file
// created by Excel.
//
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// Copyright 2022-2023, John McNamara, jmcnamara@cpan.org

use crate::common;
use rust_xlsxwriter::{
    ConditionalFormatCell, ConditionalFormatCellCriteria, Format, Workbook, XlsxError,
};

// Create rust_xlsxwriter file to compare against Excel file.
fn create_new_xlsx_file(filename: &str) -> Result<(), XlsxError> {
    let mut workbook = Workbook::new();

    let worksheet = workbook.add_worksheet();

    let format = Format::new().set_bold().set_italic();

    worksheet.write_with_format(0, 0, "Hello", &format)?;

    worksheet.write(2, 1, 10)?;
    worksheet.write(3, 1, 20)?;
    worksheet.write(4, 1, 30)?;
    worksheet.write(5, 1, 40)?;

    let conditional_format = ConditionalFormatCell::new()
        .set_criteria(ConditionalFormatCellCriteria::GreaterThan)
        .set_value(20)
        .set_format(format);

    worksheet.add_conditional_format(2, 1, 5, 1, &conditional_format)?;

    workbook.save(filename)?;

    Ok(())
}

#[test]
fn test_cond_format10() {
    let test_runner = common::TestRunner::new()
        .set_name("cond_format10")
        .set_function(create_new_xlsx_file)
        .initialize();

    test_runner.assert_eq();
    test_runner.cleanup();
}
