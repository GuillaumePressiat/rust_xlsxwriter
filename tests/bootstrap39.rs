// Test case that compares a file generated by rust_xlsxwriter with a file
// created by Excel.
//
// SPDX-License-Identifier: MIT OR Apache-2.0 Copyright 2022, John McNamara,
// jmcnamara@cpan.org

use rust_xlsxwriter::{Workbook, XlsxError};

mod common;

// Test case to demonstrate writing a future function, with explicit xlfn.
fn create_new_xlsx_file_1(filename: &str) -> Result<(), XlsxError> {
    let mut workbook = Workbook::new();
    let worksheet = workbook.add_worksheet();

    worksheet.write_formula_only(0, 0, "=_xlfn.ISOWEEKNUM(1)")?;
    worksheet.set_formula_result(0, 0, "52");

    workbook.save(filename)?;

    Ok(())
}

// Test case to demonstrate writing a future function, with implicit xlfn.
fn create_new_xlsx_file_2(filename: &str) -> Result<(), XlsxError> {
    let mut workbook = Workbook::new();
    let worksheet = workbook.add_worksheet();

    worksheet.use_future_functions(true);
    worksheet.write_formula_only(0, 0, "=ISOWEEKNUM(1)")?;
    worksheet.set_formula_result(0, 0, "52");

    workbook.save(filename)?;

    Ok(())
}

#[test]
fn bootstrap39_future_function_1() {
    let test_runner = common::TestRunner::new("bootstrap39")
        .unique("1")
        .ignore_calc_chain()
        .initialize();

    let result = create_new_xlsx_file_1(test_runner.output_file());
    assert_result!(result);

    test_runner.assert_eq();
    test_runner.cleanup();
}

#[test]
fn bootstrap39_future_function_2() {
    let test_runner = common::TestRunner::new("bootstrap39")
        .unique("2")
        .ignore_calc_chain()
        .initialize();

    let result = create_new_xlsx_file_2(test_runner.output_file());
    assert_result!(result);

    test_runner.assert_eq();
    test_runner.cleanup();
}
