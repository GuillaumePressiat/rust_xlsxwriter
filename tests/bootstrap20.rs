// Test case that compares a file generated by rust_xlsxwriter with a file
// created by Excel.
//
// SPDX-License-Identifier: MIT OR Apache-2.0 Copyright 2022, John McNamara,
// jmcnamara@cpan.org

use rust_xlsxwriter::{Format, Workbook, XlsxError, XlsxUnderline};

mod common;

// Test case to demonstrate cell font formatting.
fn create_new_xlsx_file(filename: &str) -> Result<(), XlsxError> {
    let mut workbook = Workbook::new(filename);

    let format1 = Format::new().set_underline(XlsxUnderline::None);
    let format2 = Format::new().set_underline(XlsxUnderline::Single);
    let format3 = Format::new().set_underline(XlsxUnderline::Double);
    let format4 = Format::new().set_underline(XlsxUnderline::SingleAccounting);
    let format5 = Format::new().set_underline(XlsxUnderline::DoubleAccounting);

    let worksheet = workbook.add_worksheet();
    worksheet.write_string(0, 0, "Rust", &format1)?;
    worksheet.write_string(1, 0, "Rust", &format2)?;
    worksheet.write_string(2, 0, "Rust", &format3)?;
    worksheet.write_string(3, 0, "Rust", &format4)?;
    worksheet.write_string(4, 0, "Rust", &format5)?;

    workbook.close()?;

    Ok(())
}

#[test]
fn bootstrap20_font_underlines() {
    let testcase = "bootstrap20";

    let (excel_file, xlsxwriter_file) = common::get_xlsx_filenames(testcase);
    _ = create_new_xlsx_file(&xlsxwriter_file);
    common::assert_eq(&excel_file, &xlsxwriter_file);
    common::remove_test_xlsx_file(&xlsxwriter_file);
}