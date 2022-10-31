// Test case that compares a file generated by rust_xlsxwriter with a file
// created by Excel.
//
// SPDX-License-Identifier: MIT OR Apache-2.0 Copyright 2022, John McNamara,
// jmcnamara@cpan.org

use rust_xlsxwriter::{Format, Workbook, XlsxAlign, XlsxError};

mod common;

// Test case to demonstrate creating a basic file with alignment formatting.
fn create_new_xlsx_file(filename: &str) -> Result<(), XlsxError> {
    let mut workbook = Workbook::new();
    let worksheet = workbook.add_worksheet();
    worksheet.set_paper_size(9);

    worksheet.set_row_height(0, 85.5)?;
    worksheet.set_row_height(1, 85.5)?;
    worksheet.set_row_height(2, 85.5)?;
    worksheet.set_row_height(4, 45)?;

    let format1 = Format::new().set_rotation(30);
    let format2 = Format::new().set_rotation(-30);
    let format3 = Format::new().set_rotation(270);
    let format4 = Format::new().set_text_wrap();
    let format5 = Format::new().set_shrink();
    let format6 = Format::new().set_indent(1);
    let format7 = Format::new().set_indent(1).set_align(XlsxAlign::Right);
    let format8 = Format::new().set_reading_direction(1);
    let format9 = Format::new().set_reading_direction(2);

    worksheet.write_string(0, 0, "Rust", &format1)?;
    worksheet.write_string(1, 0, "Rust", &format2)?;
    worksheet.write_string(2, 0, "Rust", &format3)?;
    worksheet.write_string(3, 0, "Rust", &format4)?;
    worksheet.write_string(4, 0, "Text\nWrap", &format4)?;
    worksheet.write_string(5, 0, "Rust", &format5)?;
    worksheet.write_string(6, 0, "Rust", &format6)?;
    worksheet.write_string(7, 0, "Rust", &format7)?;
    worksheet.write_string(8, 0, "Rust", &format8)?;
    worksheet.write_string(9, 0, "Rust", &format9)?;

    workbook.save(filename)?;

    Ok(())
}

#[test]
fn bootstrap30_alignment() {
    let test_runner = common::TestRunner::new("bootstrap30").initialize();

    let result = create_new_xlsx_file(test_runner.output_file());
    assert_result!(result);

    test_runner.assert_eq();
    test_runner.cleanup();
}
