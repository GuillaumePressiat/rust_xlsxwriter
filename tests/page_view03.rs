// Test case that compares a file generated by rust_xlsxwriter with a file
// created by Excel.
//
// SPDX-License-Identifier: MIT OR Apache-2.0
// Copyright 2022, John McNamara, jmcnamara@cpan.org

use rust_xlsxwriter::{Workbook, XlsxError};

mod common;

// Test to demonstrate page view page breaks + zoom.
fn create_new_xlsx_file(filename: &str) -> Result<(), XlsxError> {
    let mut workbook = Workbook::new();

    let worksheet = workbook.add_worksheet();

    worksheet.set_view_page_break_preview();
    worksheet.set_zoom(75);
    worksheet.set_paper_size(9);

    worksheet.write_string_only(0, 0, "Foo")?;

    workbook.save(filename)?;

    Ok(())
}

#[test]
fn test_page_view03() {
    let test_runner = common::TestRunner::new("page_view03").initialize();

    let result = create_new_xlsx_file(test_runner.output_file());
    assert_result!(result);

    test_runner.assert_eq();
    test_runner.cleanup();
}
