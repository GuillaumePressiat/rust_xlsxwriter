// Test case that compares a file generated by rust_xlsxwriter with a file
// created by Excel.
//
// SPDX-License-Identifier: MIT OR Apache-2.0 Copyright 2022, John McNamara,
// jmcnamara@cpan.org

use rust_xlsxwriter::{Format, Workbook, XlsxColor, XlsxError, XlsxPattern};

mod common;

// Test case to demonstrate creating a basic file with cell patterns and color.
fn create_new_xlsx_file(filename: &str) -> Result<(), XlsxError> {
    let mut workbook = Workbook::new();
    let worksheet = workbook.add_worksheet();

    // Test paper size as well.
    worksheet.set_paper_size(11);

    let format1 = Format::new().set_bold();
    let format2 = Format::new().set_foreground_color(XlsxColor::Red);
    let format3 = Format::new().set_pattern(XlsxPattern::MediumGray);
    let format4 = Format::new()
        .set_background_color(XlsxColor::Yellow)
        .set_foreground_color(XlsxColor::Red)
        .set_pattern(XlsxPattern::DarkVertical);
    let format5 = Format::new().set_background_color(XlsxColor::RGB(0x00B050));

    worksheet.write_blank(0, 0, &format1)?;
    worksheet.write_blank(1, 0, &format2)?;
    worksheet.write_blank(2, 0, &format3)?;
    worksheet.write_blank(3, 0, &format4)?;
    worksheet.write_blank(4, 0, &format5)?;

    workbook.save(filename)?;

    Ok(())
}

#[test]
fn bootstrap31_format_patterns() {
    let test_runner = common::TestRunner::new("bootstrap31").initialize();

    let result = create_new_xlsx_file(test_runner.output_file());
    assert_result!(result);

    test_runner.assert_eq();
    test_runner.cleanup();
}
