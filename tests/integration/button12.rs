// Test case that compares a file generated by rust_xlsxwriter with a file
// created by Excel.
//
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// Copyright 2022-2024, John McNamara, jmcnamara@cpan.org

use crate::common;
use rust_xlsxwriter::{Button, Note, Workbook, XlsxError};

// Create rust_xlsxwriter file to compare against Excel file.
fn create_new_xlsx_file(filename: &str) -> Result<(), XlsxError> {
    let mut workbook = Workbook::new();

    // Worksheet 1.
    let worksheet1 = workbook.add_worksheet();
    worksheet1.set_default_note_author("John");

    let note = Note::new("Some text").add_author_prefix(false);
    worksheet1.insert_note(0, 0, &note)?;

    let button = Button::new();
    worksheet1.insert_button(1, 2, &button)?;

    // Worksheet 2.
    let _worksheet2 = workbook.add_worksheet();

    // Worksheet 3.
    let worksheet3 = workbook.add_worksheet();
    worksheet3.set_default_note_author("John");

    let note = Note::new("More text").add_author_prefix(false);
    worksheet3.insert_note(2, 2, &note)?;

    workbook.save(filename)?;

    Ok(())
}

#[test]
fn test_button12() {
    let test_runner = common::TestRunner::new()
        .set_name("button12")
        .set_function(create_new_xlsx_file)
        .initialize();

    test_runner.assert_eq();
    test_runner.cleanup();
}
