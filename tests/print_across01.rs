// Test case that compares a file generated by rust_xlsxwriter with a file
// created by Excel.
//
// SPDX-License-Identifier: MIT OR Apache-2.0
// Copyright 2022, John McNamara, jmcnamara@cpan.org

use rust_xlsxwriter::{Workbook, XlsxError};

mod common;

// Test to demonstrate print-across print option.
fn create_new_xlsx_file(filename: &str) -> Result<(), XlsxError> {
    let mut workbook = Workbook::new();

    let worksheet = workbook.add_worksheet();

    worksheet.set_page_order(false);
    worksheet.set_paper_size(9);
    worksheet.set_portrait(); // Secondary test. Should be the default.

    worksheet.write_string_only(0, 0, "Foo")?;

    workbook.save(filename)?;

    Ok(())
}

#[test]
fn test_print_across01() {
    let test_runner = common::TestRunner::new("print_across01")
        .ignore_elements("xl/worksheets/sheet1.xml", "<pageMargins")
        .initialize();

    let result = create_new_xlsx_file(test_runner.output_file());
    assert_result!(result);

    test_runner.assert_eq();
    test_runner.cleanup();
}
