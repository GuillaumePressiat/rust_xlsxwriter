// Test case that compares a file generated by rust_xlsxwriter with a file
// created by Excel.
//
// SPDX-License-Identifier: MIT OR Apache-2.0 Copyright 2022, John McNamara,
// jmcnamara@cpan.org

use rust_xlsxwriter::{Format, Workbook, XlsxError};

mod common;

// Test case to test array formulas, with a format.
fn create_new_xlsx_file(filename: &str) -> Result<(), XlsxError> {
    let mut workbook = Workbook::new();

    let worksheet = workbook.add_worksheet();
    let bold = Format::new().set_bold();

    worksheet.write_array_formula(0, 0, 2, 0, "=SUM(B1:C1*B2:C2)", &bold)?;

    worksheet.write_number_only(0, 1, 0)?;
    worksheet.write_number_only(1, 1, 0)?;
    worksheet.write_number_only(2, 1, 0)?;
    worksheet.write_number_only(0, 2, 0)?;
    worksheet.write_number_only(1, 2, 0)?;
    worksheet.write_number_only(2, 2, 0)?;

    workbook.save(filename)?;

    Ok(())
}

#[test]
fn test_array_formula02() {
    let test_runner = common::TestRunner::new("array_formula02")
        .ignore_calc_chain()
        .initialize();

    let result = create_new_xlsx_file(test_runner.output_file());
    assert_result!(result);

    test_runner.assert_eq();
    test_runner.cleanup();
}
