// Test case that compares a file generated by rust_xlsxwriter with a file
// created by Excel.
//
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// Copyright 2022-2023, John McNamara, jmcnamara@cpan.org

use crate::common;
use rust_xlsxwriter::{Workbook, XlsxError};

// Test case to demonstrate setting freeze panes.
fn create_new_xlsx_file(filename: &str) -> Result<(), XlsxError> {
    let mut workbook = Workbook::new();

    let worksheet1 = workbook.add_worksheet();
    worksheet1.write_string(0, 0, "Foo")?;
    worksheet1.set_freeze_panes(1, 1)?;

    let worksheet2 = workbook.add_worksheet();
    worksheet2.write_string(0, 0, "Foo")?;
    worksheet2.set_freeze_panes(2, 3)?;

    let worksheet3 = workbook.add_worksheet();
    worksheet3.write_string(0, 0, "Foo")?;
    worksheet3.set_freeze_panes(1, 0)?;

    let worksheet4 = workbook.add_worksheet();
    worksheet4.write_string(0, 0, "Foo")?;
    worksheet4.set_freeze_panes(0, 1)?;

    let worksheet5 = workbook.add_worksheet();
    worksheet5.write_string(0, 0, "Foo")?;
    worksheet5.set_freeze_panes(1, 1)?;
    worksheet5.set_freeze_panes_top_cell(11, 3)?;

    let worksheet6 = workbook.add_worksheet();
    worksheet6.write_string(0, 0, "Foo")?;
    worksheet6.set_freeze_panes(2, 3)?;
    worksheet6.set_freeze_panes_top_cell(12, 4)?;

    let worksheet7 = workbook.add_worksheet();
    worksheet7.write_string(0, 0, "Foo")?;
    worksheet7.set_freeze_panes(1, 0)?;
    worksheet7.set_freeze_panes_top_cell(6, 0)?;

    let worksheet8 = workbook.add_worksheet();
    worksheet8.write_string(0, 0, "Foo")?;
    worksheet8.set_freeze_panes(0, 1)?;
    worksheet8.set_freeze_panes_top_cell(0, 2)?;

    workbook.save(filename)?;

    Ok(())
}

#[test]
fn bootstrap45_freeze_panes() {
    let test_runner = common::TestRunner::new()
        .set_name("bootstrap45")
        .set_function(create_new_xlsx_file)
        .initialize();

    test_runner.assert_eq();
    test_runner.cleanup();
}