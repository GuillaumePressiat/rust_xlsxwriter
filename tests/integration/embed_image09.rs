// Test case that compares a file generated by rust_xlsxwriter with a file
// created by Excel.
//
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// Copyright 2022-2023, John McNamara, jmcnamara@cpan.org

use crate::common;
use rust_xlsxwriter::{Image, Workbook, XlsxError};

// Create rust_xlsxwriter file to compare against Excel file.
fn create_new_xlsx_file(filename: &str) -> Result<(), XlsxError> {
    let mut workbook = Workbook::new();

    let worksheet = workbook.add_worksheet();

    let image = Image::new("tests/input/images/red.png")?
        .set_alt_text("Some alt text")
        .set_decorative(true);

    worksheet.embed_image(0, 0, &image)?;

    workbook.save(filename)?;

    Ok(())
}

#[test]
fn test_embed_image09() {
    let test_runner = common::TestRunner::new()
        .set_name("embed_image09")
        .set_function(create_new_xlsx_file)
        .initialize();

    test_runner.assert_eq();
    test_runner.cleanup();
}
